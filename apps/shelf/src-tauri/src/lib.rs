use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use librewin_common::config::{read_presets, FadePreset};
use librewin_common::os::resolve_binary;
use librewin_common::xattr as lx;
use librewin_common::{get_accent as lw_get_accent, get_theme as lw_get_theme};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use tauri::command;
use tauri::{Emitter, Manager};
use xz2::read::XzDecoder;

// ── Types ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<u64>, // Unix timestamp (seconds)
    pub extension: Option<String>,
    pub tags: Vec<String>, // xattr user.tags — comma-separated, parsed here
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VolumeInfo {
    pub name: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct WineStatus {
    pub winepath_available: bool,
    pub wineprefix_initialized: bool,
}

// ── Archive types ─────────────────────────────────────────────────────────────

#[derive(Serialize, Clone, Debug)]
pub struct ArchiveEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub compressed_size: Option<u64>,
    pub modified: Option<u64>,
    pub is_symlink: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct ArchiveInfo {
    pub format: String,
    pub file_count: usize,
    pub total_uncompressed: u64,
    pub total_compressed: u64,
    pub is_password_protected: bool,
}

// ── Archive helpers ───────────────────────────────────────────────────────────

/// Prevent zip-slip: returns None if entry_path tries to escape dest.
fn safe_extract_path(dest: &Path, entry_path: &str) -> Option<PathBuf> {
    let stripped = entry_path.trim_start_matches('/');
    let mut result = dest.to_path_buf();
    for component in Path::new(stripped).components() {
        match component {
            std::path::Component::Normal(c) => result.push(c),
            std::path::Component::ParentDir => return None,
            _ => {}
        }
    }
    if result.starts_with(dest) { Some(result) } else { None }
}

fn archive_format(path: &str) -> &'static str {
    let p = path.to_lowercase();
    if p.ends_with(".tar.gz") || p.ends_with(".tgz") { "tar.gz" }
    else if p.ends_with(".tar.bz2") || p.ends_with(".tbz2") { "tar.bz2" }
    else if p.ends_with(".tar.xz") || p.ends_with(".txz") { "tar.xz" }
    else if p.ends_with(".tar") { "tar" }
    else if p.ends_with(".zip") { "zip" }
    else if p.ends_with(".7z") { "7z" }
    else if p.ends_with(".rar") { "rar" }
    else { "unknown" }
}

fn list_archive_zip(path: &str) -> Result<Vec<ArchiveEntry>, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut entries: Vec<ArchiveEntry> = Vec::new();
    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(|e| e.to_string())?;
        let raw_path = entry.name().to_string();
        let is_dir = entry.is_dir();
        let name = raw_path.trim_end_matches('/').split('/').next_back().unwrap_or("").to_string();
        let path_field = raw_path.trim_end_matches('/').to_string();
        let modified = entry.last_modified().map(|dt| {
            // Convert to unix seconds (approximate)
            let year = dt.year() as u64;
            let days = (year.saturating_sub(1970)) * 365 + (dt.month() as u64) * 30 + (dt.day() as u64);
            days * 86400 + (dt.hour() as u64) * 3600 + (dt.minute() as u64) * 60 + (dt.second() as u64)
        });
        entries.push(ArchiveEntry {
            name,
            path: path_field,
            is_dir,
            size: Some(entry.size()),
            compressed_size: Some(entry.compressed_size()),
            modified,
            is_symlink: false,
        });
    }
    sort_archive_entries(&mut entries);
    Ok(entries)
}

fn list_archive_7z(path: &str) -> Result<Vec<ArchiveEntry>, String> {
    let mut reader = sevenz_rust2::ArchiveReader::open(path, sevenz_rust2::Password::empty())
        .map_err(|e| e.to_string())?;
    let mut entries: Vec<ArchiveEntry> = Vec::new();
    reader
        .for_each_entries(&mut |entry: &sevenz_rust2::ArchiveEntry, _: &mut dyn std::io::Read| {
            let raw_path = entry.name().to_string();
            let is_dir = entry.is_directory();
            let name = raw_path.trim_end_matches('/').split('/').next_back().unwrap_or("").to_string();
            let path_field = raw_path.trim_end_matches('/').to_string();
            entries.push(ArchiveEntry {
                name,
                path: path_field,
                is_dir,
                size: Some(entry.size()),
                compressed_size: None,
                modified: None,
                is_symlink: false,
            });
            Ok(true)
        })
        .map_err(|e| e.to_string())?;
    sort_archive_entries(&mut entries);
    Ok(entries)
}

fn list_archive_tar<R: std::io::Read>(reader: R) -> Result<Vec<ArchiveEntry>, String> {
    let mut archive = tar::Archive::new(reader);
    let mut entries: Vec<ArchiveEntry> = Vec::new();
    for entry_result in archive.entries().map_err(|e| e.to_string())? {
        let entry = entry_result.map_err(|e| e.to_string())?;
        let raw_path = entry.path().map_err(|e| e.to_string())?.to_string_lossy().to_string();
        let is_dir = entry.header().entry_type().is_dir();
        let is_symlink = entry.header().entry_type().is_symlink();
        let name = raw_path.trim_end_matches('/').split('/').next_back().unwrap_or("").to_string();
        let path_field = raw_path.trim_end_matches('/').to_string();
        let size = entry.header().size().ok();
        let modified = entry.header().mtime().ok();
        entries.push(ArchiveEntry {
            name,
            path: path_field,
            is_dir,
            size,
            compressed_size: None,
            modified,
            is_symlink,
        });
    }
    sort_archive_entries(&mut entries);
    Ok(entries)
}

fn list_archive_rar(path: &str) -> Result<Vec<ArchiveEntry>, String> {
    // RAR: shell out to `unrar l -p- <path>`
    let output = std::process::Command::new("unrar")
        .args(["l", "-p-", path])
        .output()
        .map_err(|_| "RAR support requires 'unrar' to be installed".to_string())?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() { "unrar failed".to_string() } else { stderr });
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries: Vec<ArchiveEntry> = Vec::new();
    // Parse `unrar l` output: lines after the dashed separator with size + name columns
    let mut in_listing = false;
    for line in stdout.lines() {
        if line.starts_with("---") { in_listing = !in_listing; continue; }
        if !in_listing { continue; }
        // Format: "  Attr  Size  Packed Ratio  Date   Time   Name"
        // Name is at end of line after multiple whitespace-separated fields
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 { continue; }
        let name_part = parts[parts.len() - 1];
        let is_dir = parts[0].contains('D');
        let size: Option<u64> = parts[1].parse().ok();
        entries.push(ArchiveEntry {
            name: name_part.split('/').next_back().unwrap_or(name_part).to_string(),
            path: name_part.to_string(),
            is_dir,
            size,
            compressed_size: parts[2].parse().ok(),
            modified: None,
            is_symlink: false,
        });
    }
    sort_archive_entries(&mut entries);
    Ok(entries)
}

fn sort_archive_entries(entries: &mut [ArchiveEntry]) {
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.path.to_lowercase().cmp(&b.path.to_lowercase()),
    });
}

fn extract_zip(path: &str, dest: &Path, files: Option<&[String]>, password: Option<&[u8]>) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    for i in 0..archive.len() {
        let mut entry = if let Some(pw) = password {
            archive.by_index_decrypt(i, pw).map_err(|e| e.to_string())?
        } else {
            archive.by_index(i).map_err(|e| {
                if e.to_string().contains("password") || e.to_string().contains("encrypt") {
                    "PASSWORD_REQUIRED".to_string()
                } else {
                    e.to_string()
                }
            })?
        };
        // Check if entry requires password that wasn't provided
        if entry.encrypted() && password.is_none() {
            return Err("PASSWORD_REQUIRED".to_string());
        }
        let raw = entry.name().to_string();
        if let Some(filter) = files {
            let clean = raw.trim_end_matches('/');
            if !filter.iter().any(|f| f == clean) { continue; }
        }
        let out_path = match safe_extract_path(dest, &raw) {
            Some(p) => p,
            None => { eprintln!("zip-slip blocked: {raw}"); continue; }
        };
        if entry.is_dir() {
            fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut out = fs::File::create(&out_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn extract_7z(path: &str, dest: &Path, files: Option<&[String]>, password: Option<&str>) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let pw = password
        .map(sevenz_rust2::Password::new)
        .unwrap_or_else(sevenz_rust2::Password::empty);
    let mut reader = sevenz_rust2::ArchiveReader::open(path, pw).map_err(|e| e.to_string())?;
    let dest_owned = dest.to_path_buf();
    let mut io_err: Option<String> = None;
    reader
        .for_each_entries(
            &mut |entry: &sevenz_rust2::ArchiveEntry, reader: &mut dyn std::io::Read| {
                let raw = entry.name().to_string();
                if let Some(filter) = files {
                    let clean = raw.trim_end_matches('/');
                    if !filter.iter().any(|f| f == clean) {
                        return Ok(true);
                    }
                }
                let out_path = match safe_extract_path(&dest_owned, &raw) {
                    Some(p) => p,
                    None => {
                        eprintln!("zip-slip blocked (7z): {raw}");
                        return Ok(true);
                    }
                };
                let result = if entry.is_directory() {
                    fs::create_dir_all(&out_path).map(|_| ())
                } else {
                    (|| {
                        if let Some(parent) = out_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        let mut out = fs::File::create(&out_path)?;
                        std::io::copy(reader, &mut out)?;
                        Ok(())
                    })()
                };
                if let Err(e) = result {
                    io_err = Some(e.to_string());
                    return Ok(false); // stop iteration
                }
                Ok(true)
            },
        )
        .map_err(|e| e.to_string())?;
    if let Some(e) = io_err { return Err(e); }
    Ok(())
}

fn extract_tar_reader<R: std::io::Read>(reader: R, dest: &Path, files: Option<&[String]>) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let mut archive = tar::Archive::new(reader);
    for entry_result in archive.entries().map_err(|e| e.to_string())? {
        let mut entry = entry_result.map_err(|e| e.to_string())?;
        let raw = entry.path().map_err(|e| e.to_string())?.to_string_lossy().to_string();
        if let Some(filter) = files {
            let clean = raw.trim_end_matches('/');
            if !filter.iter().any(|f| f == clean) { continue; }
        }
        let out_path = match safe_extract_path(dest, &raw) {
            Some(p) => p,
            None => { eprintln!("zip-slip blocked (tar): {raw}"); continue; }
        };
        let is_dir = entry.header().entry_type().is_dir();
        let is_symlink = entry.header().entry_type().is_symlink();
        if is_symlink {
            // Extract symlinks as regular files for security
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            fs::write(&out_path, b"").map_err(|e| e.to_string())?;
        } else if is_dir {
            fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut out = fs::File::create(&out_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Read user.tags xattr from a path; returns empty vec if not set or error.
fn read_tags_for(path: &str) -> Vec<String> {
    lx::read_tags(path)
}

// ── Archive commands ──────────────────────────────────────────────────────────

#[command]
fn list_archive(path: String) -> Result<Vec<ArchiveEntry>, String> {
    let fmt = archive_format(&path);
    match fmt {
        "zip" => list_archive_zip(&path),
        "7z" => list_archive_7z(&path),
        "tar" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            list_archive_tar(file)
        }
        "tar.gz" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            list_archive_tar(GzDecoder::new(file))
        }
        "tar.bz2" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            list_archive_tar(BzDecoder::new(file))
        }
        "tar.xz" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            list_archive_tar(XzDecoder::new(file))
        }
        "rar" => list_archive_rar(&path),
        _ => Err(format!("Unsupported archive format: {path}")),
    }
}

#[command]
fn extract_archive(path: String, dest: String) -> Result<(), String> {
    let dest_path = Path::new(&dest);
    let fmt = archive_format(&path);
    match fmt {
        "zip" => extract_zip(&path, dest_path, None, None),
        "7z" => extract_7z(&path, dest_path, None, None),
        "tar" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(file, dest_path, None)
        }
        "tar.gz" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(GzDecoder::new(file), dest_path, None)
        }
        "tar.bz2" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(BzDecoder::new(file), dest_path, None)
        }
        "tar.xz" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(XzDecoder::new(file), dest_path, None)
        }
        _ => Err(format!("Unsupported archive format for extraction: {path}")),
    }
}

#[command]
fn extract_files(path: String, files: Vec<String>, dest: String) -> Result<(), String> {
    let dest_path = Path::new(&dest);
    let fmt = archive_format(&path);
    match fmt {
        "zip" => extract_zip(&path, dest_path, Some(&files), None),
        "7z" => extract_7z(&path, dest_path, Some(&files), None),
        "tar" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(file, dest_path, Some(&files))
        }
        "tar.gz" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(GzDecoder::new(file), dest_path, Some(&files))
        }
        "tar.bz2" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(BzDecoder::new(file), dest_path, Some(&files))
        }
        "tar.xz" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            extract_tar_reader(XzDecoder::new(file), dest_path, Some(&files))
        }
        _ => Err(format!("Unsupported archive format for selective extraction: {path}")),
    }
}

#[command]
fn archive_info(path: String) -> Result<ArchiveInfo, String> {
    let fmt = archive_format(&path);
    match fmt {
        "zip" => {
            let file = fs::File::open(&path).map_err(|e| e.to_string())?;
            let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
            let mut total_uncompressed: u64 = 0;
            let mut total_compressed: u64 = 0;
            let mut is_pw = false;
            let count = archive.len();
            for i in 0..count {
                let entry = archive.by_index(i).map_err(|e| e.to_string())?;
                total_uncompressed += entry.size();
                total_compressed += entry.compressed_size();
                if entry.encrypted() { is_pw = true; }
            }
            Ok(ArchiveInfo {
                format: "zip".to_string(),
                file_count: count,
                total_uncompressed,
                total_compressed,
                is_password_protected: is_pw,
            })
        }
        "7z" => {
            let mut reader = sevenz_rust2::ArchiveReader::open(&path, sevenz_rust2::Password::empty())
                .map_err(|e| e.to_string())?;
            let mut count = 0usize;
            let mut total: u64 = 0;
            reader
                .for_each_entries(
                    &mut |entry: &sevenz_rust2::ArchiveEntry, _: &mut dyn std::io::Read| {
                        count += 1;
                        total += entry.size();
                        Ok(true)
                    },
                )
                .map_err(|e: sevenz_rust2::Error| e.to_string())?;
            Ok(ArchiveInfo {
                format: "7z".to_string(),
                file_count: count,
                total_uncompressed: total,
                total_compressed: 0,
                is_password_protected: false,
            })
        }
        "tar" | "tar.gz" | "tar.bz2" | "tar.xz" => {
            let entries = list_archive(path.clone())?;
            let total: u64 = entries.iter().filter_map(|e| e.size).sum();
            Ok(ArchiveInfo {
                format: fmt.to_string(),
                file_count: entries.len(),
                total_uncompressed: total,
                total_compressed: 0,
                is_password_protected: false,
            })
        }
        _ => Err(format!("Unsupported archive: {path}")),
    }
}

#[command]
fn extract_archive_with_password(path: String, dest: String, password: String) -> Result<(), String> {
    let dest_path = Path::new(&dest);
    let fmt = archive_format(&path);
    match fmt {
        "zip" => extract_zip(&path, dest_path, None, Some(password.as_bytes())),
        "7z" => extract_7z(&path, dest_path, None, Some(password.as_str())),
        _ => Err("Password-protected extraction only supported for ZIP and 7Z".to_string()),
    }
}

#[command]
fn pick_directory() -> Option<String> {
    rfd::FileDialog::new().pick_folder().map(|p| p.to_string_lossy().to_string())
}

// ── Commands ─────────────────────────────────────────────────────────────────

/// List directory contents, sorted folders-first then alpha.
/// Skips hidden entries (names starting with '.').
#[command]
fn list_dir(path: String) -> Result<Vec<FileEntry>, String> {
    let dir = Path::new(&path);
    if !dir.exists() {
        return Err(format!("Path does not exist: {path}"));
    }
    if !dir.is_dir() {
        return Err(format!("Not a directory: {path}"));
    }

    let read = fs::read_dir(dir).map_err(|e| e.to_string())?;
    let mut entries: Vec<FileEntry> = Vec::new();

    for result in read {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        let entry_path = entry.path().to_string_lossy().to_string();
        let metadata = entry.metadata().ok();
        let is_dir = metadata.as_ref().is_some_and(|m| m.is_dir());
        let size = if is_dir {
            None
        } else {
            metadata.as_ref().map(|m| m.len())
        };
        let modified = metadata
            .as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
        let extension = if is_dir {
            None
        } else {
            Path::new(&name)
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase().to_string())
        };
        let tags = read_tags_for(&entry_path);

        entries.push(FileEntry {
            name,
            path: entry_path,
            is_dir,
            size,
            modified,
            extension,
            tags,
        });
    }

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

/// Returns the current user's home directory.
#[command]
fn get_home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| {
        #[cfg(unix)]
        {
            extern "C" {
                fn getuid() -> u32;
            }
            let uid = unsafe { getuid() };
            format!("/home/{uid}")
        }
        #[cfg(not(unix))]
        "/home/user".to_string()
    })
}

/// Open a file or directory with the system's default application (xdg-open).
/// .desktop files are blocked — they can execute arbitrary commands and must
/// not be launched by double-click; use the application's own launcher instead.
#[command]
fn open_file(path: String) -> Result<(), String> {
    let ext = Path::new(&path)
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase());
    if ext.as_deref() == Some("desktop") {
        return Err("Opening .desktop files is not allowed".to_string());
    }
    std::process::Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("xdg-open failed: {e}"))?;
    Ok(())
}

/// Returns mounted volumes/filesystems the user cares about.
#[command]
fn get_volumes() -> Vec<VolumeInfo> {
    let mut volumes: Vec<VolumeInfo> = Vec::new();

    if let Ok(contents) = fs::read_to_string("/proc/mounts") {
        for line in contents.lines() {
            let parts: Vec<&str> = line.splitn(6, ' ').collect();
            if parts.len() < 3 {
                continue;
            }
            let device = parts[0];
            let mount = parts[1];
            let fstype = parts[2];

            let skip = !device.starts_with('/')
                || mount.starts_with("/proc")
                || mount.starts_with("/sys")
                || mount.starts_with("/dev")
                || mount.starts_with("/run")
                || mount.starts_with("/snap")
                || mount == "/"
                || matches!(
                    fstype,
                    "tmpfs"
                        | "devtmpfs"
                        | "squashfs"
                        | "overlay"
                        | "cgroup"
                        | "cgroup2"
                        | "pstore"
                        | "bpf"
                        | "tracefs"
                        | "debugfs"
                        | "securityfs"
                        | "efivarfs"
                );

            if skip {
                continue;
            }

            let label = mount
                .split('/')
                .rfind(|s| !s.is_empty())
                .unwrap_or(mount)
                .to_string();

            volumes.push(VolumeInfo {
                name: label,
                path: mount.to_string(),
            });
        }
    }

    volumes
}

/// Check whether Wine's winepath tool is installed and the default WINEPREFIX exists.
/// Called on startup to decide whether to show the "Copy Windows path" menu item.
#[command]
fn get_wine_status() -> WineStatus {
    let winepath_available = resolve_binary("winepath").is_some();

    let home = std::env::var("HOME").unwrap_or_default();
    let wineprefix_initialized = Path::new(&format!("{home}/.wine")).exists();

    WineStatus {
        winepath_available,
        wineprefix_initialized,
    }
}

/// Translate a Linux path to its Windows equivalent via `winepath -w`.
/// Returns Err("winepath_not_found") if Wine is not installed (aarch64).
/// Returns Err("wine_not_initialized") if ~/.wine does not exist yet.
#[command]
fn get_windows_path(path: String) -> Result<String, String> {
    let winepath = resolve_binary("winepath").ok_or_else(|| "winepath_not_found".to_string())?;

    let home = std::env::var("HOME").unwrap_or_default();
    if !Path::new(&format!("{home}/.wine")).exists() {
        return Err("wine_not_initialized".to_string());
    }

    let out = std::process::Command::new(&winepath)
        .arg("-w")
        .arg(&path)
        .output()
        .map_err(|e| format!("winepath exec failed: {e}"))?;

    if !out.status.success() {
        return Err(format!(
            "winepath error: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }

    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Read tags for a single file path. Returns [] if none set.
#[command]
fn get_tags(path: String) -> Vec<String> {
    read_tags_for(&path)
}

/// Write tags to a file's user.tags xattr.
/// Pass an empty vec to remove all tags.
#[command]
fn set_tags(path: String, tags: Vec<String>) -> Result<(), String> {
    lx::write_tags(&path, &tags)
}

// ── Quick Convert ─────────────────────────────────────────────────────────────

#[command]
fn list_fade_presets() -> Vec<FadePreset> {
    read_presets()
}

/// Run a custom Fade preset — fire-and-forget, emits "quick-convert-done" or "quick-convert-error".
#[command]
fn run_fade_preset(window: tauri::Window, path: String, preset_id: String) -> Result<(), String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }

    let presets = read_presets();

    let preset = presets
        .into_iter()
        .find(|pr| pr.id == preset_id)
        .ok_or_else(|| format!("Preset not found: {preset_id}"))?;

    let stem = p
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let dir = p
        .parent()
        .map(|d| d.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());
    let out_ext = preset.output_format.to_lowercase();
    let output_path = format!("{}/{}_converted.{}", dir, stem, out_ext);

    let (cmd, mut args): (&str, Vec<String>) = match preset.media_type.as_str() {
        "image" => {
            let mut a = vec![path.clone()];
            if let Some(q) = preset.quality {
                a.extend(["-quality".into(), q.to_string()]);
            }
            a.push("-strip".into());
            ("magick", a)
        },
        "video" => {
            let mut a = vec!["-y".into(), "-i".into(), path.clone()];
            let codec = preset.codec.as_deref().unwrap_or("copy");
            if codec == "copy" {
                a.extend(["-c".into(), "copy".into()]);
            } else {
                let codec_args: Vec<String> = match codec {
                    "h264" => vec![
                        "-vcodec".into(),
                        "libx264".into(),
                        "-preset".into(),
                        "medium".into(),
                    ],
                    "h265" => vec![
                        "-vcodec".into(),
                        "libx265".into(),
                        "-preset".into(),
                        "medium".into(),
                    ],
                    "vp9" => vec![
                        "-vcodec".into(),
                        "libvpx-vp9".into(),
                        "-b:v".into(),
                        "0".into(),
                        "-crf".into(),
                        "33".into(),
                    ],
                    "av1" => vec![
                        "-vcodec".into(),
                        "libaom-av1".into(),
                        "-crf".into(),
                        "30".into(),
                        "-b:v".into(),
                        "0".into(),
                    ],
                    _ => vec!["-c".into(), "copy".into()],
                };
                a.extend(codec_args);
            }
            if let Some(br) = preset.bitrate {
                a.extend(["-b:a".into(), format!("{}k", br)]);
            }
            if let Some(sr) = preset.sample_rate {
                a.extend(["-ar".into(), sr.to_string()]);
            }
            ("ffmpeg", a)
        },
        "audio" => {
            let mut a = vec!["-y".into(), "-i".into(), path.clone(), "-vn".into()];
            if let Some(br) = preset.bitrate {
                a.extend(["-b:a".into(), format!("{}k", br)]);
            }
            if let Some(sr) = preset.sample_rate {
                a.extend(["-ar".into(), sr.to_string()]);
            }
            ("ffmpeg", a)
        },
        other => return Err(format!("Unknown media type: {other}")),
    };

    args.push(output_path.clone());
    let cmd = cmd.to_string();

    std::thread::spawn(move || {
        #[derive(serde::Serialize, Clone)]
        struct ConvertResult {
            input: String,
            output: String,
        }
        #[derive(serde::Serialize, Clone)]
        struct ConvertFail {
            input: String,
            message: String,
        }

        let output = std::process::Command::new(&cmd)
            .args(&args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let _ = window.emit(
                    "quick-convert-done",
                    ConvertResult {
                        input: path,
                        output: output_path,
                    },
                );
            },
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let message = if stderr.is_empty() {
                    format!("{cmd} failed")
                } else {
                    stderr
                };
                let _ = window.emit(
                    "quick-convert-error",
                    ConvertFail {
                        input: path,
                        message,
                    },
                );
            },
            Err(e) => {
                let _ = window.emit(
                    "quick-convert-error",
                    ConvertFail {
                        input: path,
                        message: format!("{cmd} not found: {e}"),
                    },
                );
            },
        }
    });

    Ok(())
}

/// Quick Convert presets — fire-and-forget conversions from the right-click menu.
/// Runs in a background thread; emits "quick-convert-done" or "quick-convert-error" events.
///
/// Preset identifiers:
///   "video_mp4"      — H.264 MP4, 192k audio, 48kHz
///   "image_jpeg"     — JPEG quality 85, strip metadata
///   "image_png"      — PNG lossless, strip metadata
///   "image_webp"     — WebP quality 85, strip metadata
///   "audio_mp3"      — MP3 192kbps 44.1kHz
///   "audio_flac"     — FLAC lossless 44.1kHz
#[command]
fn quick_convert(window: tauri::Window, path: String, preset: String) -> Result<(), String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }

    let stem = p
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let dir = p
        .parent()
        .map(|d| d.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".to_string());

    let (cmd, args, out_ext): (&str, Vec<String>, &str) = match preset.as_str() {
        "video_mp4" => (
            "ffmpeg",
            vec![
                "-y".into(),
                "-i".into(),
                path.clone(),
                "-vcodec".into(),
                "libx264".into(),
                "-preset".into(),
                "medium".into(),
                "-b:a".into(),
                "192k".into(),
                "-ar".into(),
                "48000".into(),
            ],
            "mp4",
        ),
        "image_jpeg" => (
            "magick",
            vec![
                path.clone(),
                "-quality".into(),
                "85".into(),
                "-strip".into(),
            ],
            "jpg",
        ),
        "image_png" => ("magick", vec![path.clone(), "-strip".into()], "png"),
        "image_webp" => (
            "magick",
            vec![
                path.clone(),
                "-quality".into(),
                "85".into(),
                "-strip".into(),
            ],
            "webp",
        ),
        "audio_mp3" => (
            "ffmpeg",
            vec![
                "-y".into(),
                "-i".into(),
                path.clone(),
                "-vn".into(),
                "-b:a".into(),
                "192k".into(),
                "-ar".into(),
                "44100".into(),
            ],
            "mp3",
        ),
        "audio_flac" => (
            "ffmpeg",
            vec![
                "-y".into(),
                "-i".into(),
                path.clone(),
                "-vn".into(),
                "-c:a".into(),
                "flac".into(),
                "-ar".into(),
                "44100".into(),
            ],
            "flac",
        ),
        other => return Err(format!("Unknown preset: {other}")),
    };

    let output_path = format!("{}/{}_converted.{}", dir, stem, out_ext);
    let mut full_args = args;
    full_args.push(output_path.clone());

    let cmd = cmd.to_string();
    std::thread::spawn(move || {
        #[derive(serde::Serialize, Clone)]
        struct ConvertResult {
            input: String,
            output: String,
        }
        #[derive(serde::Serialize, Clone)]
        struct ConvertFail {
            input: String,
            message: String,
        }

        let output = std::process::Command::new(&cmd)
            .args(&full_args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let _ = window.emit(
                    "quick-convert-done",
                    ConvertResult {
                        input: path,
                        output: output_path,
                    },
                );
            },
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let message = if stderr.is_empty() {
                    format!("{cmd} conversion failed")
                } else {
                    stderr
                };
                let _ = window.emit(
                    "quick-convert-error",
                    ConvertFail {
                        input: path,
                        message,
                    },
                );
            },
            Err(e) => {
                let _ = window.emit(
                    "quick-convert-error",
                    ConvertFail {
                        input: path,
                        message: format!("{cmd} not found: {e}"),
                    },
                );
            },
        }
    });

    Ok(())
}

// ── Theme ─────────────────────────────────────────────────────────────────────

#[tauri::command]
fn get_theme() -> String {
    lw_get_theme()
}

#[tauri::command]
fn get_accent() -> String {
    lw_get_accent()
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

            // Super+E — toggle Shelf visibility (Shelf mode)
            let shortcut: Shortcut = "Super+E".parse()?;
            let handle = app.handle().clone();
            app.global_shortcut()
                .on_shortcut(shortcut, move |_app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        if let Some(window) = handle.get_webview_window("main") {
                            if window.is_focused().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_dir,
            get_home_dir,
            open_file,
            get_volumes,
            get_tags,
            set_tags,
            get_wine_status,
            get_windows_path,
            quick_convert,
            list_fade_presets,
            run_fade_preset,
            get_theme,
            get_accent,
            list_archive,
            extract_archive,
            extract_files,
            archive_info,
            extract_archive_with_password,
            pick_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running shelf");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn list_dir_returns_entries_sorted_dirs_first() {
        let base = std::env::temp_dir().join(format!("shelf_test_{}", std::process::id()));
        fs::create_dir_all(&base).unwrap();
        fs::write(base.join("aardvark.txt"), "a").unwrap();
        fs::create_dir_all(base.join("zebra")).unwrap();

        let result = list_dir(base.to_string_lossy().to_string()).unwrap();
        fs::remove_dir_all(&base).unwrap();

        assert_eq!(result[0].name, "zebra");
        assert!(result[0].is_dir);
        assert_eq!(result[1].name, "aardvark.txt");
        assert!(!result[1].is_dir);
    }

    #[test]
    fn list_dir_skips_hidden_files() {
        let base = std::env::temp_dir().join(format!("shelf_hidden_{}", std::process::id()));
        fs::create_dir_all(&base).unwrap();
        fs::write(base.join("visible.txt"), "v").unwrap();
        fs::write(base.join(".hidden"), "h").unwrap();

        let result = list_dir(base.to_string_lossy().to_string()).unwrap();
        fs::remove_dir_all(&base).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "visible.txt");
    }

    #[test]
    fn list_dir_errors_on_missing_path() {
        let result = list_dir("/nonexistent/shelf/test/path".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn list_dir_errors_on_file_not_dir() {
        let p = std::env::temp_dir().join(format!("shelf_file_{}.txt", std::process::id()));
        fs::write(&p, "content").unwrap();
        let result = list_dir(p.to_string_lossy().to_string());
        fs::remove_file(&p).unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Not a directory"));
    }

    // ── Archive helpers ────────────────────────────────────────────────────

    fn make_test_zip(dest: &std::path::Path) {
        use std::io::Write as _;
        let file = fs::File::create(dest).expect("create zip");
        let mut zip = zip::ZipWriter::new(file);
        let opts = zip::write::FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated);
        zip.start_file("hello.txt", opts).expect("start hello.txt");
        zip.write_all(b"hello").expect("write hello");
        zip.add_directory("subdir/", opts).expect("add subdir");
        zip.start_file("subdir/world.txt", opts).expect("start world.txt");
        zip.write_all(b"world").expect("write world");
        zip.start_file("subdir/nested/deep.txt", opts).expect("start deep.txt");
        zip.write_all(b"deep").expect("write deep");
        zip.finish().expect("finish zip");
    }

    fn make_test_tar_gz(dest: &std::path::Path) {
        let file = fs::File::create(dest).expect("create tar.gz");
        let enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut tar = tar::Builder::new(enc);
        let mut add = |name: &str, data: &[u8]| {
            let mut header = tar::Header::new_gnu();
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append_data(&mut header, name, data).expect("tar append");
        };
        add("hello.txt", b"hello");
        add("subdir/world.txt", b"world");
        add("subdir/nested/deep.txt", b"deep");
        tar.finish().expect("tar finish");
    }

    #[test]
    fn safe_extract_path_blocks_traversal() {
        let dest = std::path::Path::new("/tmp/safe_test");
        assert!(safe_extract_path(dest, "../etc/passwd").is_none());
        assert!(safe_extract_path(dest, "../../escape").is_none());
        assert!(safe_extract_path(dest, "subdir/file.txt").is_some());
        assert!(safe_extract_path(dest, "normal.txt").is_some());
    }

    #[test]
    fn list_archive_zip_contents() {
        let tmp = std::env::temp_dir().join(format!("shelf_test_zip_{}.zip", std::process::id()));
        make_test_zip(&tmp);
        let entries = list_archive(tmp.to_string_lossy().to_string()).expect("list zip");
        fs::remove_file(&tmp).ok();
        // Expect: hello.txt, subdir (synthesized dirs are handled in frontend, but zip crate
        // returns explicit entries). We have: hello.txt, subdir/, subdir/world.txt, subdir/nested/deep.txt
        assert_eq!(entries.len(), 4);
        assert!(entries.iter().any(|e| e.path == "hello.txt"));
        assert!(entries.iter().any(|e| e.path == "subdir/world.txt"));
        assert!(entries.iter().any(|e| e.path == "subdir/nested/deep.txt"));
    }

    #[test]
    fn list_archive_tar_gz_contents() {
        let tmp = std::env::temp_dir().join(format!("shelf_test_{}.tar.gz", std::process::id()));
        make_test_tar_gz(&tmp);
        let entries = list_archive(tmp.to_string_lossy().to_string()).expect("list tar.gz");
        fs::remove_file(&tmp).ok();
        assert_eq!(entries.len(), 3);
        assert!(entries.iter().any(|e| e.path == "hello.txt"));
        assert!(entries.iter().any(|e| e.path == "subdir/world.txt"));
        assert!(entries.iter().any(|e| e.path == "subdir/nested/deep.txt"));
    }

    #[test]
    fn list_archive_corrupted_returns_err() {
        let tmp = std::env::temp_dir().join(format!("shelf_corrupted_{}.zip", std::process::id()));
        fs::write(&tmp, b"\x00\x01\x02garbage\xff").expect("write corrupted");
        let result = list_archive(tmp.to_string_lossy().to_string());
        fs::remove_file(&tmp).ok();
        assert!(result.is_err());
    }

    #[test]
    fn extract_archive_zip_creates_files() {
        let tmp_zip = std::env::temp_dir().join(format!("shelf_ext_{}.zip", std::process::id()));
        let tmp_dest = std::env::temp_dir().join(format!("shelf_ext_dest_{}", std::process::id()));
        make_test_zip(&tmp_zip);
        extract_archive(tmp_zip.to_string_lossy().to_string(), tmp_dest.to_string_lossy().to_string())
            .expect("extract");
        fs::remove_file(&tmp_zip).ok();
        assert!(tmp_dest.join("hello.txt").exists());
        assert_eq!(fs::read_to_string(tmp_dest.join("hello.txt")).unwrap(), "hello");
        assert!(tmp_dest.join("subdir").join("world.txt").exists());
        fs::remove_dir_all(&tmp_dest).ok();
    }

    #[test]
    fn tags_roundtrip() {
        let p = std::env::temp_dir().join(format!("shelf_tags_{}.txt", std::process::id()));
        fs::write(&p, "tag test").unwrap();
        let path = p.to_string_lossy().to_string();

        set_tags(path.clone(), vec!["work".to_string(), "urgent".to_string()]).unwrap();
        let tags = get_tags(path.clone());

        set_tags(path.clone(), vec![]).unwrap();
        let empty = get_tags(path.clone());
        fs::remove_file(&p).unwrap();

        assert_eq!(tags, vec!["work", "urgent"]);
        assert!(empty.is_empty());
    }
}
