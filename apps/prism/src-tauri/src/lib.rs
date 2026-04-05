use serde::Serialize;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use tauri::command;

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub category: String,
    pub path: String,
}

fn category_for_ext(ext: &str) -> &'static str {
    match ext {
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" | "ico" | "avif" => "image",
        "heic" | "heif" | "tiff" | "tif" | "psd" | "exr" | "hdr" | "dds"
        | "raw" | "cr2" | "cr3" | "nef" | "arw" | "dng" | "orf" | "rw2" | "xcf" => "image_convert",
        "mp4" | "m4v" | "webm" | "mov" | "avi" | "mkv" | "flv" | "wmv"
        | "mpg" | "mpeg" | "ogv" | "ts" | "3gp" | "divx" | "rmvb" | "asf" => "video",
        "mp3" | "aac" | "ogg" | "oga" | "wav" | "flac" | "m4a" | "opus"
        | "wma" | "aiff" | "aif" | "alac" | "ac3" | "dts" => "audio",
        "pdf" => "pdf",
        "obj" | "gltf" | "glb" | "stl" | "fbx" | "ply" | "3ds" => "model",
        _ => "unknown",
    }
}

fn mime_for_ext(ext: &str) -> &'static str {
    match ext {
        "mp4" | "m4v" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "avi" => "video/x-msvideo",
        "mkv" => "video/x-matroska",
        "ogv" => "video/ogg",
        "ts" => "video/mp2t",
        "flv" => "video/x-flv",
        "wmv" => "video/x-ms-wmv",
        "3gp" => "video/3gpp",
        "mpeg" | "mpg" => "video/mpeg",
        "mp3" => "audio/mpeg",
        "ogg" | "oga" => "audio/ogg",
        "wav" => "audio/wav",
        "flac" => "audio/flac",
        "aac" => "audio/aac",
        "m4a" | "alac" => "audio/mp4",
        "opus" => "audio/ogg; codecs=opus",
        "wma" => "audio/x-ms-wma",
        "aiff" | "aif" => "audio/aiff",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "bmp" => "image/bmp",
        "avif" => "image/avif",
        "pdf" => "application/pdf",
        "gltf" => "model/gltf+json",
        "glb" => "model/gltf-binary",
        "obj" => "text/plain",
        "stl" => "application/octet-stream",
        _ => "application/octet-stream",
    }
}

fn decode_path(encoded: &str) -> String {
    let mut out = String::with_capacity(encoded.len());
    let bytes = encoded.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = char::from(bytes[i + 1]).to_digit(16);
            let lo = char::from(bytes[i + 2]).to_digit(16);
            if let (Some(h), Some(l)) = (hi, lo) {
                out.push(char::from((h * 16 + l) as u8));
                i += 3;
                continue;
            }
        }
        out.push(char::from(bytes[i]));
        i += 1;
    }
    out
}

fn tmp_tag() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    format!("{}-{}", std::process::id(), nanos)
}

#[command]
fn get_initial_file() -> Option<String> {
    std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
}

#[command]
fn get_file_info(path: String) -> Result<FileInfo, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }
    let meta = fs::metadata(p).map_err(|e| e.to_string())?;
    let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
    let ext = p.extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
        .to_string();
    let category = category_for_ext(&ext).to_string();
    Ok(FileInfo { name, ext, size: meta.len(), category, path })
}

#[command]
fn convert_image_to_png(path: String) -> Result<String, String> {
    let tmp = format!("/tmp/librewin-mv-{}.png", tmp_tag());

    // Multi-frame formats (HEIC, TIFF, PSD) produce numbered output files
    // (e.g. out-0.png, out-1.png) unless we pin to frame [0].
    // Append [0] for known multi-frame container formats.
    let ext = std::path::Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let src = if matches!(ext.as_str(), "heic" | "heif" | "tiff" | "tif" | "psd" | "psb") {
        format!("{path}[0]")
    } else {
        path.clone()
    };

    // Try ImageMagick 7 (magick) then IM6 (convert)
    let ok = std::process::Command::new("magick")
        .args(["convert", &src, &tmp])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !ok {
        let out = std::process::Command::new("convert")
            .args([&src, &tmp])
            .output()
            .map_err(|e| format!("ImageMagick not found: {e}"))?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
    }
    Ok(tmp)
}

fn handle_stream(req: tauri::http::Request<Vec<u8>>) -> tauri::http::Response<Vec<u8>> {
    let uri = req.uri().to_string();

    // Strip scheme+host prefix
    let path_encoded = uri
        .strip_prefix("mvstream://localhost")
        .unwrap_or_default();
    let path_str = decode_path(path_encoded);
    let path = Path::new(&path_str);

    if !path.exists() {
        return tauri::http::Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body(b"Not found".to_vec())
            .unwrap();
    }

    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let mime = mime_for_ext(&ext);

    let file_size = match fs::metadata(path) {
        Ok(m) => m.len(),
        Err(e) => {
            return tauri::http::Response::builder()
                .status(500)
                .body(format!("Metadata error: {e}").into_bytes())
                .unwrap();
        }
    };

    // Parse Range header
    let range = req.headers()
        .get("range")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("bytes="))
        .and_then(|s| {
            let mut parts = s.splitn(2, '-');
            let start = parts.next()?.parse::<u64>().ok()?;
            let end_str = parts.next().unwrap_or("");
            let end = if end_str.is_empty() {
                (start + 2 * 1024 * 1024).min(file_size.saturating_sub(1))
            } else {
                end_str.parse::<u64>().ok()?.min(file_size.saturating_sub(1))
            };
            if start > end { return None; }
            Some((start, end))
        });

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return tauri::http::Response::builder()
                .status(500)
                .body(format!("Open error: {e}").into_bytes())
                .unwrap();
        }
    };

    if let Some((start, end)) = range {
        let len = (end - start + 1) as usize;
        let mut buf = vec![0u8; len];
        if file.seek(SeekFrom::Start(start)).is_err() {
            return tauri::http::Response::builder()
                .status(500)
                .body(b"Seek error".to_vec())
                .unwrap();
        }
        let _ = file.read(&mut buf);

        tauri::http::Response::builder()
            .status(206)
            .header("Content-Type", mime)
            .header("Content-Range", format!("bytes {start}-{end}/{file_size}"))
            .header("Accept-Ranges", "bytes")
            .header("Content-Length", len.to_string())
            .header("Access-Control-Allow-Origin", "*")
            .body(buf)
            .unwrap()
    } else {
        // No Range — serve up to 4 MB; HTML5 video will use Range for the rest
        let cap = file_size.min(4 * 1024 * 1024) as usize;
        let mut buf = vec![0u8; cap];
        let _ = file.read(&mut buf);

        tauri::http::Response::builder()
            .status(200)
            .header("Content-Type", mime)
            .header("Accept-Ranges", "bytes")
            .header("Content-Length", file_size.to_string())
            .header("Access-Control-Allow-Origin", "*")
            .body(buf)
            .unwrap()
    }
}

// ── Theme ─────────────────────────────────────────────────────────────────────

/// Read the LibreWin theme preference from the shared config file.
/// Returns "dark", "light", or "system" (default when file absent).
#[tauri::command]
fn get_theme() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    std::fs::read_to_string(format!("{}/.config/librewin/theme", home))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "system".to_string())
}

#[tauri::command]
fn get_accent() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    std::fs::read_to_string(format!("{}/.config/librewin/accent", home))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "#297acc".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("mvstream", |_app, req| handle_stream(req))
        .invoke_handler(tauri::generate_handler![
            get_initial_file,
            get_file_info,
            convert_image_to_png,
            get_theme,
            get_accent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
