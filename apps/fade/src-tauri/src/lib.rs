use librewin_common::{get_accent as lw_get_accent, get_theme as lw_get_theme};
use librewin_common::config::{read_presets, write_presets, FadePreset};
use librewin_common::media::media_type_for;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use tauri::{command, Emitter, Window};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct JobProgress {
    job_id: String,
    percent: f32,
    message: String,
}

#[derive(Serialize, Clone)]
struct JobDone {
    job_id: String,
    output_path: String,
}

#[derive(Serialize, Clone)]
struct JobError {
    job_id: String,
    message: String,
}

#[derive(Deserialize, Clone)]
pub struct ConvertOptions {
    pub output_format: String,
    pub output_dir: Option<String>,
    // Image
    pub resize_mode: Option<String>,   // "none" | "percent" | "pixels"
    pub resize_percent: Option<u32>,
    pub resize_width: Option<u32>,
    pub resize_height: Option<u32>,
    pub quality: Option<u32>,          // 1–100 for lossy formats
    pub crop_x: Option<u32>,           // crop origin x (pixels from left)
    pub crop_y: Option<u32>,           // crop origin y (pixels from top)
    pub crop_width: Option<u32>,       // crop region width
    pub crop_height: Option<u32>,      // crop region height
    pub rotation: Option<u32>,         // 0 | 90 | 180 | 270
    pub flip_h: Option<bool>,          // horizontal mirror
    pub flip_v: Option<bool>,          // vertical mirror
    pub auto_rotate: Option<bool>,     // apply EXIF orientation (-auto-orient)
    // Video
    pub codec: Option<String>,         // "copy" | "h264" | "h265" | "vp9" | "av1"
    pub resolution: Option<String>,    // "original" | "1920x1080" | "1280x720" | "854x480"
    pub trim_start: Option<f64>,       // seconds
    pub trim_end: Option<f64>,         // seconds
    pub remove_audio: Option<bool>,
    pub extract_audio: Option<bool>,
    pub audio_format: Option<String>,  // for extract_audio path
    // Audio
    pub bitrate: Option<u32>,          // kbps
    pub sample_rate: Option<u32>,      // Hz
    pub normalize_loudness: Option<bool>,
    // Output naming
    pub output_suffix: Option<String>, // appended to stem before extension (default: "converted")
}

#[derive(Serialize)]
pub struct FileInfo {
    pub duration_secs: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codec: Option<String>,
    pub format: Option<String>,
    pub file_size: u64,
    pub media_type: String, // "image" | "video" | "audio" | "unknown"
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Get duration from ffprobe JSON output; returns None if unavailable.
fn probe_duration(path: &str) -> Option<f64> {
    let out = Command::new("ffprobe")
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            path,
        ])
        .output()
        .ok()?;
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
    let dur_str = json["format"]["duration"].as_str()?;
    dur_str.parse::<f64>().ok()
}

/// Build the output path: same dir as input (or output_dir), stem + suffix + new ext.
fn build_output_path(input: &str, new_ext: &str, output_dir: Option<&str>, suffix: &str) -> String {
    let p = Path::new(input);
    let stem = p.file_stem().unwrap_or_default().to_string_lossy();
    let dir = output_dir
        .map(|d| d.to_string())
        .unwrap_or_else(|| {
            p.parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| ".".to_string())
        });
    if suffix.is_empty() {
        format!("{}/{}.{}", dir, stem, new_ext)
    } else {
        format!("{}/{}_{}.{}", dir, stem, suffix, new_ext)
    }
}

/// Validate that a suffix only contains safe characters (alphanumeric, hyphen, underscore).
fn validate_suffix(suffix: &str) -> Result<(), String> {
    if suffix.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        Ok(())
    } else {
        Err(format!("Invalid suffix '{}': only letters, digits, hyphens, and underscores allowed", suffix))
    }
}

/// Parse out_time_ms line from ffmpeg -progress output to get elapsed seconds.
fn parse_out_time_ms(line: &str) -> Option<f64> {
    let val = line.strip_prefix("out_time_ms=")?;
    val.trim().parse::<f64>().ok().map(|ms| ms / 1_000_000.0)
}

// ── Commands ──────────────────────────────────────────────────────────────────

/// Return file info (duration, dimensions, codec, media type, size).
#[command]
fn get_file_info(path: String) -> Result<FileInfo, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }
    let file_size = p.metadata().map(|m| m.len()).unwrap_or(0);
    let ext = p.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    let mtype = media_type_for(&ext);

    if mtype == "image" {
        // Use ImageMagick identify for image info
        let out = Command::new("identify")
            .args(["-format", "%wx%h\n", &path])
            .output()
            .map_err(|e| e.to_string())?;
        let s = String::from_utf8_lossy(&out.stdout);
        let dims: Vec<&str> = s.trim().splitn(2, 'x').collect();
        let width = dims.first().and_then(|v| v.parse().ok());
        let height = dims.get(1).and_then(|v| v.parse().ok());
        return Ok(FileInfo {
            duration_secs: None,
            width,
            height,
            codec: None,
            format: Some(ext.to_string()),
            file_size,
            media_type: "image".to_string(),
        });
    }

    // Video / audio — use ffprobe
    let out = Command::new("ffprobe")
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            &path,
        ])
        .output()
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| e.to_string())?;

    let duration_secs = json["format"]["duration"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());
    let format = json["format"]["format_name"]
        .as_str()
        .map(|s| s.split(',').next().unwrap_or(s).to_string());

    let mut width = None;
    let mut height = None;
    let mut codec = None;

    if let Some(streams) = json["streams"].as_array() {
        for stream in streams {
            let ct = stream["codec_type"].as_str().unwrap_or("");
            if ct == "video" {
                width = stream["width"].as_u64().map(|v| v as u32);
                height = stream["height"].as_u64().map(|v| v as u32);
                codec = stream["codec_name"].as_str().map(|s| s.to_string());
                break;
            }
            if ct == "audio" && codec.is_none() {
                codec = stream["codec_name"].as_str().map(|s| s.to_string());
            }
        }
    }

    Ok(FileInfo {
        duration_secs,
        width,
        height,
        codec,
        format,
        file_size,
        media_type: mtype.to_string(),
    })
}

/// Convert a media file. Runs in a background thread and emits progress events.
/// Events emitted: "job-progress" (JobProgress), "job-done" (JobDone), "job-error" (JobError).
#[command]
fn convert_file(
    window: Window,
    job_id: String,
    input_path: String,
    options: ConvertOptions,
) -> Result<(), String> {
    let p = Path::new(&input_path);
    if !p.exists() || !p.is_file() {
        return Err(format!("File not found or not a regular file: {}", input_path));
    }

    let ext = options.output_format.to_lowercase();

    // Validate output format is a safe file extension (alpha + digits only)
    if ext.is_empty() || !ext.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(format!("Invalid output format: {}", options.output_format));
    }

    let mtype = media_type_for(&ext);
    if mtype == "unknown" {
        return Err(format!("Unsupported output format: {}", options.output_format));
    }

    let suffix = options.output_suffix.as_deref().unwrap_or("converted");
    validate_suffix(suffix)?;

    let output_path = build_output_path(&input_path, &ext, options.output_dir.as_deref(), suffix);

    // Spawn background thread — command returns immediately
    std::thread::spawn(move || {
        let result = match mtype {
            "image" => run_image_convert(&window, &job_id, &input_path, &output_path, &options),
            "video" => run_video_convert(&window, &job_id, &input_path, &output_path, &options),
            "audio" => run_audio_convert(&window, &job_id, &input_path, &output_path, &options),
            _ => Err("Unsupported format".to_string()),
        };

        match result {
            Ok(()) => {
                let _ = window.emit("job-done", JobDone {
                    job_id: job_id.clone(),
                    output_path,
                });
            }
            Err(msg) => {
                let _ = window.emit("job-error", JobError { job_id, message: msg });
            }
        }
    });

    Ok(())
}

// ── Image conversion (ImageMagick) ────────────────────────────────────────────

fn run_image_convert(
    window: &Window,
    job_id: &str,
    input: &str,
    output: &str,
    opts: &ConvertOptions,
) -> Result<(), String> {
    let _ = window.emit("job-progress", JobProgress {
        job_id: job_id.to_string(),
        percent: 0.0,
        message: "Converting image…".to_string(),
    });

    let mut args: Vec<String> = vec![input.to_string()];

    // EXIF auto-orient first — corrects rotation before any other transforms
    if opts.auto_rotate == Some(true) {
        args.push("-auto-orient".to_string());
    }

    // Crop
    if let (Some(cw), Some(ch)) = (opts.crop_width, opts.crop_height) {
        if cw > 0 && ch > 0 {
            let cx = opts.crop_x.unwrap_or(0);
            let cy = opts.crop_y.unwrap_or(0);
            args.push("-crop".to_string());
            args.push(format!("{}x{}+{}+{}", cw, ch, cx, cy));
            args.push("+repage".to_string()); // clear virtual canvas after crop
        }
    }

    // Resize
    match opts.resize_mode.as_deref() {
        Some("percent") => {
            let pct = opts.resize_percent.unwrap_or(100);
            args.push("-resize".to_string());
            args.push(format!("{}%", pct));
        }
        Some("pixels") => {
            let w = opts.resize_width.unwrap_or(0);
            let h = opts.resize_height.unwrap_or(0);
            if w > 0 && h > 0 {
                args.push("-resize".to_string());
                args.push(format!("{}x{}", w, h));
            } else if w > 0 {
                args.push("-resize".to_string());
                args.push(format!("{}x", w));
            } else if h > 0 {
                args.push("-resize".to_string());
                args.push(format!("x{}", h));
            }
        }
        _ => {}
    }

    // Rotation
    if let Some(deg) = opts.rotation {
        if deg == 90 || deg == 180 || deg == 270 {
            args.push("-rotate".to_string());
            args.push(deg.to_string());
        }
    }

    // Flip
    if opts.flip_v == Some(true) { args.push("-flip".to_string()); }
    if opts.flip_h == Some(true) { args.push("-flop".to_string()); }

    // Quality (lossy formats)
    if let Some(q) = opts.quality {
        args.push("-quality".to_string());
        args.push(q.to_string());
    }

    // Strip metadata for clean output
    args.push("-strip".to_string());

    args.push(output.to_string());

    let status = Command::new("magick")
        .args(&args)
        .status()
        .map_err(|e| format!("ImageMagick not found: {e}"))?;

    if status.success() {
        let _ = window.emit("job-progress", JobProgress {
            job_id: job_id.to_string(),
            percent: 100.0,
            message: "Done".to_string(),
        });
        Ok(())
    } else {
        Err("ImageMagick convert failed".to_string())
    }
}

// ── Video conversion (FFmpeg) ─────────────────────────────────────────────────

fn run_video_convert(
    window: &Window,
    job_id: &str,
    input: &str,
    output: &str,
    opts: &ConvertOptions,
) -> Result<(), String> {
    let duration = probe_duration(input);

    let mut args: Vec<String> = vec!["-y".to_string()];

    // Trim start
    if let Some(ss) = opts.trim_start {
        args.extend(["-ss".to_string(), ss.to_string()]);
    }

    args.extend(["-i".to_string(), input.to_string()]);

    // Trim end (ffmpeg -to is relative to -ss when placed after -i)
    if let Some(t) = opts.trim_end {
        let end = if let Some(ss) = opts.trim_start { t - ss } else { t };
        args.extend(["-t".to_string(), end.to_string()]);
    }

    // Video codec / stream copy
    let codec = opts.codec.as_deref().unwrap_or("copy");
    if opts.extract_audio == Some(true) {
        // Extract audio only — no video stream
        args.extend(["-vn".to_string()]);
    } else if opts.remove_audio == Some(true) {
        args.extend(["-an".to_string()]);
        if codec == "copy" {
            args.extend(["-vcodec".to_string(), "copy".to_string()]);
        } else {
            args.extend(ffmpeg_video_codec_args(codec));
        }
    } else {
        if codec == "copy" {
            args.extend(["-c".to_string(), "copy".to_string()]);
        } else {
            args.extend(ffmpeg_video_codec_args(codec));
        }
    }

    // Resolution scaling
    if let Some(res) = &opts.resolution {
        if res != "original" && opts.extract_audio != Some(true) {
            let scale = resolution_to_scale(res);
            args.extend(["-vf".to_string(), scale]);
        }
    }

    // Audio bitrate/sample rate (for video output with audio)
    if opts.remove_audio != Some(true) {
        if let Some(br) = opts.bitrate {
            args.extend(["-b:a".to_string(), format!("{}k", br)]);
        }
        if let Some(sr) = opts.sample_rate {
            args.extend(["-ar".to_string(), sr.to_string()]);
        }
    }

    // Progress reporting via pipe
    args.extend([
        "-progress".to_string(), "pipe:1".to_string(),
        "-nostats".to_string(),
    ]);

    args.push(output.to_string());

    let mut child = Command::new("ffmpeg")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("ffmpeg not found: {e}"))?;

    // Stream progress events
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            if let Some(elapsed) = parse_out_time_ms(&line) {
                let percent = if let Some(dur) = duration {
                    ((elapsed / dur) * 100.0).min(99.0) as f32
                } else {
                    0.0
                };
                let _ = window.emit("job-progress", JobProgress {
                    job_id: job_id.to_string(),
                    percent,
                    message: format!("{:.0}s elapsed", elapsed),
                });
            }
        }
    }

    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("FFmpeg conversion failed".to_string())
    }
}

fn ffmpeg_video_codec_args(codec: &str) -> Vec<String> {
    match codec {
        "h264" => vec!["-vcodec".to_string(), "libx264".to_string(),
                        "-preset".to_string(), "medium".to_string()],
        "h265" => vec!["-vcodec".to_string(), "libx265".to_string(),
                        "-preset".to_string(), "medium".to_string()],
        "vp9"  => vec!["-vcodec".to_string(), "libvpx-vp9".to_string(),
                        "-b:v".to_string(), "0".to_string(),
                        "-crf".to_string(), "33".to_string()],
        "av1"  => vec!["-vcodec".to_string(), "libaom-av1".to_string(),
                        "-crf".to_string(), "30".to_string(),
                        "-b:v".to_string(), "0".to_string()],
        _      => vec!["-c".to_string(), "copy".to_string()],
    }
}

fn resolution_to_scale(res: &str) -> String {
    match res {
        "1920x1080" => "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2".to_string(),
        "1280x720"  => "scale=1280:720:force_original_aspect_ratio=decrease,pad=1280:720:(ow-iw)/2:(oh-ih)/2".to_string(),
        "854x480"   => "scale=854:480:force_original_aspect_ratio=decrease,pad=854:480:(ow-iw)/2:(oh-ih)/2".to_string(),
        other       => format!("scale={}", other),
    }
}

// ── Audio conversion (FFmpeg) ─────────────────────────────────────────────────

fn run_audio_convert(
    window: &Window,
    job_id: &str,
    input: &str,
    output: &str,
    opts: &ConvertOptions,
) -> Result<(), String> {
    let duration = probe_duration(input);

    let mut args: Vec<String> = vec!["-y".to_string()];

    if let Some(ss) = opts.trim_start {
        args.extend(["-ss".to_string(), ss.to_string()]);
    }

    args.extend(["-i".to_string(), input.to_string()]);

    if let Some(t) = opts.trim_end {
        let end = if let Some(ss) = opts.trim_start { t - ss } else { t };
        args.extend(["-t".to_string(), end.to_string()]);
    }

    args.extend(["-vn".to_string()]); // no video output

    if let Some(br) = opts.bitrate {
        args.extend(["-b:a".to_string(), format!("{}k", br)]);
    }
    if let Some(sr) = opts.sample_rate {
        args.extend(["-ar".to_string(), sr.to_string()]);
    }

    // Loudness normalization (EBU R128)
    if opts.normalize_loudness == Some(true) {
        args.extend(["-af".to_string(), "loudnorm".to_string()]);
    }

    args.extend([
        "-progress".to_string(), "pipe:1".to_string(),
        "-nostats".to_string(),
    ]);

    args.push(output.to_string());

    let mut child = Command::new("ffmpeg")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("ffmpeg not found: {e}"))?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().map_while(Result::ok) {
            if let Some(elapsed) = parse_out_time_ms(&line) {
                let percent = if let Some(dur) = duration {
                    ((elapsed / dur) * 100.0).min(99.0) as f32
                } else {
                    0.0
                };
                let _ = window.emit("job-progress", JobProgress {
                    job_id: job_id.to_string(),
                    percent,
                    message: format!("{:.0}s elapsed", elapsed),
                });
            }
        }
    }

    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("FFmpeg audio conversion failed".to_string())
    }
}

// ── Theme / accent ────────────────────────────────────────────────────────────

#[command]
fn get_theme() -> String { lw_get_theme() }

#[command]
fn get_accent() -> String { lw_get_accent() }

// ── Custom presets ────────────────────────────────────────────────────────────

#[command]
fn list_presets() -> Vec<FadePreset> {
    read_presets()
}

#[command]
fn save_preset(
    name: String,
    media_type: String,
    output_format: String,
    quality: Option<u32>,
    codec: Option<String>,
    bitrate: Option<u32>,
    sample_rate: Option<u32>,
) -> Result<FadePreset, String> {
    // Validate name
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err("Preset name cannot be empty".to_string());
    }
    if name.len() > 64 {
        return Err("Preset name too long (max 64 chars)".to_string());
    }

    let preset = FadePreset {
        id: uuid_v4().to_string(),
        name,
        media_type,
        output_format,
        quality,
        codec,
        bitrate,
        sample_rate,
    };

    let mut presets = read_presets();
    presets.push(preset.clone());
    write_presets(&presets)?;
    Ok(preset)
}

#[command]
fn delete_preset(id: String) -> Result<(), String> {
    let mut presets = read_presets();
    let before = presets.len();
    presets.retain(|p| p.id != id);
    if presets.len() == before {
        return Err(format!("Preset not found: {id}"));
    }
    write_presets(&presets)
}

fn uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_file_info,
            convert_file,
            get_theme,
            get_accent,
            list_presets,
            save_preset,
            delete_preset,
        ])
        .run(tauri::generate_context!())
        .expect("error while running fade");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_output_path_with_suffix() {
        let result = build_output_path("/home/user/video.mp4", "mkv", None, "converted");
        assert_eq!(result, "/home/user/video_converted.mkv");
    }

    #[test]
    fn build_output_path_empty_suffix() {
        let result = build_output_path("/home/user/video.mp4", "mkv", None, "");
        assert_eq!(result, "/home/user/video.mkv");
    }

    #[test]
    fn build_output_path_custom_output_dir() {
        let result = build_output_path("/home/user/video.mp4", "mp3", Some("/tmp/out"), "converted");
        assert_eq!(result, "/tmp/out/video_converted.mp3");
    }

    #[test]
    fn validate_suffix_accepts_safe_chars() {
        assert!(validate_suffix("converted").is_ok());
        assert!(validate_suffix("my-export_v2").is_ok());
        assert!(validate_suffix("").is_ok()); // empty is valid — means no suffix
    }

    #[test]
    fn validate_suffix_rejects_unsafe_chars() {
        assert!(validate_suffix("bad/path").is_err());
        assert!(validate_suffix("has space").is_err());
        assert!(validate_suffix("dot.dot").is_err());
        assert!(validate_suffix("semi;colon").is_err());
    }

    #[test]
    fn media_type_for_image() {
        assert_eq!(media_type_for("jpg"), "image");
        assert_eq!(media_type_for("png"), "image");
        assert_eq!(media_type_for("webp"), "image");
        assert_eq!(media_type_for("heic"), "image");
    }

    #[test]
    fn media_type_for_video() {
        assert_eq!(media_type_for("mp4"), "video");
        assert_eq!(media_type_for("mkv"), "video");
        assert_eq!(media_type_for("webm"), "video");
    }

    #[test]
    fn media_type_for_audio() {
        assert_eq!(media_type_for("mp3"), "audio");
        assert_eq!(media_type_for("flac"), "audio");
        assert_eq!(media_type_for("wav"), "audio");
    }

    #[test]
    fn media_type_for_case_insensitive() {
        assert_eq!(media_type_for("JPG"), "image");
        assert_eq!(media_type_for("MP4"), "video");
        assert_eq!(media_type_for("FLAC"), "audio");
    }

    #[test]
    fn media_type_for_unknown() {
        assert_eq!(media_type_for("xyz"), "unknown");
        assert_eq!(media_type_for(""), "unknown");
    }

    #[test]
    fn parse_out_time_ms_parses_microseconds() {
        assert_eq!(parse_out_time_ms("out_time_ms=1000000"), Some(1.0));
        assert_eq!(parse_out_time_ms("out_time_ms=500000"), Some(0.5));
        assert_eq!(parse_out_time_ms("out_time_ms=0"), Some(0.0));
    }

    #[test]
    fn parse_out_time_ms_ignores_other_lines() {
        assert_eq!(parse_out_time_ms("frame=42"), None);
        assert_eq!(parse_out_time_ms("speed=1.0x"), None);
        assert_eq!(parse_out_time_ms(""), None);
    }
}
