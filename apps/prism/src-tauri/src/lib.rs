use librewin_common::{get_accent as lw_get_accent, get_theme as lw_get_theme};
use librewin_common::media::{category_for_ext, mime_for_ext};
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

#[tauri::command]
fn get_theme() -> String { lw_get_theme() }

#[tauri::command]
fn get_accent() -> String { lw_get_accent() }

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
        .expect("error while running prism");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_for_ext_image() {
        assert_eq!(category_for_ext("png"), "image");
        assert_eq!(category_for_ext("jpg"), "image");
        assert_eq!(category_for_ext("webp"), "image");
        assert_eq!(category_for_ext("gif"), "image");
    }

    #[test]
    fn category_for_ext_image_convert() {
        assert_eq!(category_for_ext("heic"), "image_convert");
        assert_eq!(category_for_ext("tiff"), "image_convert");
        assert_eq!(category_for_ext("raw"), "image_convert");
    }

    #[test]
    fn category_for_ext_video() {
        assert_eq!(category_for_ext("mp4"), "video");
        assert_eq!(category_for_ext("mkv"), "video");
        assert_eq!(category_for_ext("webm"), "video");
    }

    #[test]
    fn category_for_ext_audio() {
        assert_eq!(category_for_ext("mp3"), "audio");
        assert_eq!(category_for_ext("flac"), "audio");
        assert_eq!(category_for_ext("ogg"), "audio");
    }

    #[test]
    fn category_for_ext_pdf() {
        assert_eq!(category_for_ext("pdf"), "pdf");
    }

    #[test]
    fn category_for_ext_model() {
        assert_eq!(category_for_ext("obj"), "model");
        assert_eq!(category_for_ext("gltf"), "model");
        assert_eq!(category_for_ext("stl"), "model");
    }

    #[test]
    fn category_for_ext_unknown() {
        assert_eq!(category_for_ext("xyz"), "unknown");
        assert_eq!(category_for_ext(""), "unknown");
    }

    #[test]
    fn mime_for_ext_video() {
        assert_eq!(mime_for_ext("mp4"), "video/mp4");
        assert_eq!(mime_for_ext("webm"), "video/webm");
        assert_eq!(mime_for_ext("mkv"), "video/x-matroska");
    }

    #[test]
    fn mime_for_ext_audio() {
        assert_eq!(mime_for_ext("mp3"), "audio/mpeg");
        assert_eq!(mime_for_ext("flac"), "audio/flac");
        assert_eq!(mime_for_ext("wav"), "audio/wav");
    }

    #[test]
    fn mime_for_ext_image() {
        assert_eq!(mime_for_ext("png"), "image/png");
        assert_eq!(mime_for_ext("jpg"), "image/jpeg");
        assert_eq!(mime_for_ext("webp"), "image/webp");
    }

    #[test]
    fn mime_for_ext_pdf() {
        assert_eq!(mime_for_ext("pdf"), "application/pdf");
    }

    #[test]
    fn decode_path_handles_percent_encoding() {
        assert_eq!(decode_path("/path%20with%20spaces"), "/path with spaces");
        assert_eq!(decode_path("/normal"), "/normal");
        assert_eq!(decode_path("/file%2Fname"), "/file/name");
    }
}
