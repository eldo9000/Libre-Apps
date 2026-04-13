use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use tauri::command;
use tauri::{Emitter, Manager};

// ── Types ────────────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<u64>, // Unix timestamp (seconds)
    pub extension: Option<String>,
    pub tags: Vec<String>,     // xattr user.tags — comma-separated, parsed here
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

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Read user.tags xattr from a path; returns empty vec if not set or error.
fn read_tags_for(path: &str) -> Vec<String> {
    xattr::get(path, "user.tags")
        .ok()
        .flatten()
        .and_then(|v| String::from_utf8(v).ok())
        .map(|s| {
            s.split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect()
        })
        .unwrap_or_default()
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
        let is_dir = metadata.as_ref().map_or(false, |m| m.is_dir());
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
                    "tmpfs" | "devtmpfs" | "squashfs" | "overlay"
                        | "cgroup" | "cgroup2" | "pstore" | "bpf"
                        | "tracefs" | "debugfs" | "securityfs" | "efivarfs"
                );

            if skip {
                continue;
            }

            let label = mount.split('/').filter(|s| !s.is_empty()).last()
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
    let winepath_available = std::process::Command::new("which")
        .arg("winepath")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let home = std::env::var("HOME").unwrap_or_default();
    let wineprefix_initialized = Path::new(&format!("{home}/.wine")).exists();

    WineStatus { winepath_available, wineprefix_initialized }
}

/// Translate a Linux path to its Windows equivalent via `winepath -w`.
/// Returns Err("winepath_not_found") if Wine is not installed (aarch64).
/// Returns Err("wine_not_initialized") if ~/.wine does not exist yet.
#[command]
fn get_windows_path(path: String) -> Result<String, String> {
    let which = std::process::Command::new("which")
        .arg("winepath")
        .output()
        .map_err(|e| e.to_string())?;
    if !which.status.success() {
        return Err("winepath_not_found".to_string());
    }

    let home = std::env::var("HOME").unwrap_or_default();
    if !Path::new(&format!("{home}/.wine")).exists() {
        return Err("wine_not_initialized".to_string());
    }

    let out = std::process::Command::new("winepath")
        .arg("-w")
        .arg(&path)
        .output()
        .map_err(|e| format!("winepath exec failed: {e}"))?;

    if !out.status.success() {
        return Err(format!("winepath error: {}", String::from_utf8_lossy(&out.stderr).trim()));
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
    let value: String = tags.iter()
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join(",");

    if value.is_empty() {
        match xattr::remove(&path, "user.tags") {
            Ok(_) => {}
            Err(e) if e.raw_os_error() == Some(61) => {} // ENODATA — already gone
            Err(e) => return Err(e.to_string()),
        }
    } else {
        xattr::set(&path, "user.tags", value.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ── Quick Convert ─────────────────────────────────────────────────────────────

// ── Fade custom presets ─────────────────────────────────────────────────────
//
// COUPLING RISK: This FadePreset struct must stay in sync with the identical
// struct in apps/fade/src-tauri/src/lib.rs. Both read the same JSON file
// (~/.config/librewin/fade-presets.json). A field rename or type change in
// either copy will silently break the other. Once E3 (librewin-common) lands,
// move this struct there and import it from both apps.

#[derive(Serialize, Deserialize, Clone)]
struct FadePreset {
    id: String,
    name: String,
    media_type: String,
    output_format: String,
    quality: Option<u32>,
    codec: Option<String>,
    bitrate: Option<u32>,
    sample_rate: Option<u32>,
}

fn fade_presets_path() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    format!("{}/.config/librewin/fade-presets.json", home)
}

#[command]
fn list_fade_presets() -> Vec<FadePreset> {
    std::fs::read_to_string(fade_presets_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Run a custom Fade preset — fire-and-forget, emits "quick-convert-done" or "quick-convert-error".
#[command]
fn run_fade_preset(window: tauri::Window, path: String, preset_id: String) -> Result<(), String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }

    let presets: Vec<FadePreset> = std::fs::read_to_string(fade_presets_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let preset = presets.into_iter().find(|pr| pr.id == preset_id)
        .ok_or_else(|| format!("Preset not found: {preset_id}"))?;

    let stem = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let dir = p.parent().map(|d| d.to_string_lossy().to_string()).unwrap_or_else(|| ".".to_string());
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
        }
        "video" => {
            let mut a = vec!["-y".into(), "-i".into(), path.clone()];
            let codec = preset.codec.as_deref().unwrap_or("copy");
            if codec == "copy" {
                a.extend(["-c".into(), "copy".into()]);
            } else {
                let codec_args: Vec<String> = match codec {
                    "h264" => vec!["-vcodec".into(), "libx264".into(), "-preset".into(), "medium".into()],
                    "h265" => vec!["-vcodec".into(), "libx265".into(), "-preset".into(), "medium".into()],
                    "vp9"  => vec!["-vcodec".into(), "libvpx-vp9".into(), "-b:v".into(), "0".into(), "-crf".into(), "33".into()],
                    "av1"  => vec!["-vcodec".into(), "libaom-av1".into(), "-crf".into(), "30".into(), "-b:v".into(), "0".into()],
                    _      => vec!["-c".into(), "copy".into()],
                };
                a.extend(codec_args);
            }
            if let Some(br) = preset.bitrate { a.extend(["-b:a".into(), format!("{}k", br)]); }
            if let Some(sr) = preset.sample_rate { a.extend(["-ar".into(), sr.to_string()]); }
            ("ffmpeg", a)
        }
        "audio" => {
            let mut a = vec!["-y".into(), "-i".into(), path.clone(), "-vn".into()];
            if let Some(br) = preset.bitrate { a.extend(["-b:a".into(), format!("{}k", br)]); }
            if let Some(sr) = preset.sample_rate { a.extend(["-ar".into(), sr.to_string()]); }
            ("ffmpeg", a)
        }
        other => return Err(format!("Unknown media type: {other}")),
    };

    args.push(output_path.clone());
    let cmd = cmd.to_string();

    std::thread::spawn(move || {
        #[derive(serde::Serialize, Clone)]
        struct ConvertResult { input: String, output: String }
        #[derive(serde::Serialize, Clone)]
        struct ConvertFail { input: String, message: String }

        let output = std::process::Command::new(&cmd)
            .args(&args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .output();

        match output {
            Ok(out) if out.status.success() => { let _ = window.emit("quick-convert-done", ConvertResult { input: path, output: output_path }); }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let message = if stderr.is_empty() { format!("{cmd} failed") } else { stderr };
                let _ = window.emit("quick-convert-error", ConvertFail { input: path, message });
            }
            Err(e) => { let _ = window.emit("quick-convert-error", ConvertFail { input: path, message: format!("{cmd} not found: {e}") }); }
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

    let stem = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let dir = p.parent().map(|d| d.to_string_lossy().to_string()).unwrap_or_else(|| ".".to_string());

    let (cmd, args, out_ext): (&str, Vec<String>, &str) = match preset.as_str() {
        "video_mp4" => (
            "ffmpeg",
            vec![
                "-y".into(), "-i".into(), path.clone(),
                "-vcodec".into(), "libx264".into(), "-preset".into(), "medium".into(),
                "-b:a".into(), "192k".into(), "-ar".into(), "48000".into(),
            ],
            "mp4",
        ),
        "image_jpeg" => (
            "magick",
            vec![path.clone(), "-quality".into(), "85".into(), "-strip".into()],
            "jpg",
        ),
        "image_png" => (
            "magick",
            vec![path.clone(), "-strip".into()],
            "png",
        ),
        "image_webp" => (
            "magick",
            vec![path.clone(), "-quality".into(), "85".into(), "-strip".into()],
            "webp",
        ),
        "audio_mp3" => (
            "ffmpeg",
            vec![
                "-y".into(), "-i".into(), path.clone(),
                "-vn".into(), "-b:a".into(), "192k".into(), "-ar".into(), "44100".into(),
            ],
            "mp3",
        ),
        "audio_flac" => (
            "ffmpeg",
            vec![
                "-y".into(), "-i".into(), path.clone(),
                "-vn".into(), "-c:a".into(), "flac".into(), "-ar".into(), "44100".into(),
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
        struct ConvertResult { input: String, output: String }
        #[derive(serde::Serialize, Clone)]
        struct ConvertFail { input: String, message: String }

        let output = std::process::Command::new(&cmd)
            .args(&full_args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let _ = window.emit("quick-convert-done", ConvertResult {
                    input: path,
                    output: output_path,
                });
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                let message = if stderr.is_empty() { format!("{cmd} conversion failed") } else { stderr };
                let _ = window.emit("quick-convert-error", ConvertFail { input: path, message });
            }
            Err(e) => {
                let _ = window.emit("quick-convert-error", ConvertFail {
                    input: path,
                    message: format!("{cmd} not found: {e}"),
                });
            }
        }
    });

    Ok(())
}

// ── Theme ─────────────────────────────────────────────────────────────────────

/// Read the LibreWin theme preference from the shared config file.
/// Returns "dark", "light", or "system" (default when file absent).
#[tauri::command]
fn get_theme() -> String { librewin_common::get_theme() }

#[tauri::command]
fn get_accent() -> String { librewin_common::get_accent() }

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
            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
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
