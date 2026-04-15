use librewin_common::media::{category_for_ext, mime_for_ext};
use librewin_common::{get_accent as lw_get_accent, get_theme as lw_get_theme};
use serde::Serialize;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use tauri::command;

// ── FileInfo ──────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub category: String,
    pub path: String,
}

// ── Metadata ──────────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct Metadata {
    pub file_size: u64,
    pub category: String,
    // Image
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub color_space: Option<String>,
    pub bit_depth: Option<u32>,
    pub exif_camera_make: Option<String>,
    pub exif_camera_model: Option<String>,
    pub exif_lens: Option<String>,
    pub exif_date_taken: Option<String>,
    pub exif_gps_lat: Option<String>,
    pub exif_gps_lng: Option<String>,
    // Video / Audio (ffprobe)
    pub codec: Option<String>,
    pub bitrate: Option<u64>,
    pub duration_secs: Option<f64>,
    pub frame_rate: Option<String>,
    pub audio_tracks: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u32>,
    pub id3_artist: Option<String>,
    pub id3_album: Option<String>,
    pub id3_title: Option<String>,
    // PDF
    pub pdf_page_count: Option<u32>,
    pub pdf_author: Option<String>,
    pub pdf_title: Option<String>,
    pub pdf_creation_date: Option<String>,
    pub pdf_version: Option<String>,
    // 3D
    pub vertex_count: Option<u64>,
    pub face_count: Option<u64>,
    pub material_count: Option<u32>,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

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

/// Read the first N bytes of a file.
fn read_header(path: &Path, n: usize) -> Option<Vec<u8>> {
    let mut f = fs::File::open(path).ok()?;
    let mut buf = vec![0u8; n];
    let read = f.read(&mut buf).ok()?;
    buf.truncate(read);
    Some(buf)
}

/// Parse image pixel dimensions from the file header (PNG/JPEG/GIF/WebP).
fn image_dimensions_from_header(path: &Path, ext: &str) -> (Option<u32>, Option<u32>) {
    let header = match read_header(path, 26) {
        Some(h) if h.len() >= 12 => h,
        _ => return (None, None),
    };

    match ext {
        "png" => {
            // PNG IHDR: bytes 16-19 = width, 20-23 = height (big-endian u32)
            if header.len() < 24 {
                return (None, None);
            }
            let w = u32::from_be_bytes([header[16], header[17], header[18], header[19]]);
            let h = u32::from_be_bytes([header[20], header[21], header[22], header[23]]);
            (Some(w), Some(h))
        },
        "gif" => {
            // GIF header: bytes 6-7 width, 8-9 height (little-endian u16)
            if header.len() < 10 {
                return (None, None);
            }
            let w = u16::from_le_bytes([header[6], header[7]]) as u32;
            let h = u16::from_le_bytes([header[8], header[9]]) as u32;
            (Some(w), Some(h))
        },
        "webp" => {
            // WebP: check for "RIFF....WEBPVP8 " (lossy) or "RIFF....WEBPVP8L" (lossless)
            // VP8 : width at bytes 26-27 (LE u16, mask 0x3FFF), height 28-29 (LE u16, mask 0x3FFF)
            // VP8L: bits after signature 0x2F
            // Simplified: just return None if we can't read enough
            let full = match read_header(path, 30) {
                Some(h) if h.len() >= 30 => h,
                _ => return (None, None),
            };
            if full.len() < 30 {
                return (None, None);
            }
            if &full[8..12] == b"WEBP" && &full[12..16] == b"VP8 " {
                let w = (u16::from_le_bytes([full[26], full[27]]) & 0x3FFF) as u32 + 1;
                let h = (u16::from_le_bytes([full[28], full[29]]) & 0x3FFF) as u32 + 1;
                return (Some(w), Some(h));
            }
            (None, None)
        },
        "jpg" | "jpeg" => {
            // JPEG: scan for SOF0/SOF1/SOF2 markers
            jpeg_dimensions(path)
        },
        "bmp" => {
            // BMP: bytes 18-21 width, 22-25 height (LE i32)
            if header.len() < 26 {
                return (None, None);
            }
            let w =
                i32::from_le_bytes([header[18], header[19], header[20], header[21]]).unsigned_abs();
            let h =
                i32::from_le_bytes([header[22], header[23], header[24], header[25]]).unsigned_abs();
            (Some(w), Some(h))
        },
        _ => (None, None),
    }
}

fn jpeg_dimensions(path: &Path) -> (Option<u32>, Option<u32>) {
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(_) => return (None, None),
    };
    let mut i = 0usize;
    while i + 1 < data.len() {
        if data[i] != 0xFF {
            i += 1;
            continue;
        }
        let marker = data[i + 1];
        // SOF0, SOF1, SOF2 markers hold dimensions
        if (0xC0..=0xC2).contains(&marker) && i + 9 < data.len() {
            let h = u16::from_be_bytes([data[i + 5], data[i + 6]]) as u32;
            let w = u16::from_be_bytes([data[i + 7], data[i + 8]]) as u32;
            return (Some(w), Some(h));
        }
        // Skip marker payload
        if i + 3 < data.len() && !matches!(marker, 0xD8 | 0xD9 | 0x01) {
            let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
            i += 2 + len;
        } else {
            i += 2;
        }
    }
    (None, None)
}

// ── Metadata extraction ───────────────────────────────────────────────────────

fn metadata_image(path: &Path, ext: &str, file_size: u64) -> Metadata {
    let (mut width, mut height) = image_dimensions_from_header(path, ext);

    let mut color_space: Option<String> = None;
    let mut bit_depth: Option<u32> = None;
    let mut exif_camera_make: Option<String> = None;
    let mut exif_camera_model: Option<String> = None;
    let mut exif_lens: Option<String> = None;
    let mut exif_date_taken: Option<String> = None;
    let mut exif_gps_lat: Option<String> = None;
    let mut exif_gps_lng: Option<String> = None;

    if let Ok(mut f) = fs::File::open(path) {
        let mut buf = Vec::new();
        let _ = f.read_to_end(&mut buf);
        if let Ok(exif) = exif::Reader::new().read_from_container(&mut std::io::Cursor::new(&buf)) {
            use exif::{In, Tag};

            let field_str = |tag| -> Option<String> {
                exif.get_field(tag, In::PRIMARY)
                    .map(|f| f.display_value().to_string().trim_matches('"').to_string())
            };

            // Dimensions from EXIF (override header-parsed values if present)
            if let Some(f) = exif.get_field(Tag::PixelXDimension, In::PRIMARY) {
                if let exif::Value::Long(ref v) = f.value {
                    width = v.first().copied();
                } else if let exif::Value::Short(ref v) = f.value {
                    width = v.first().map(|&x| x as u32);
                }
            }
            if let Some(f) = exif.get_field(Tag::PixelYDimension, In::PRIMARY) {
                if let exif::Value::Long(ref v) = f.value {
                    height = v.first().copied();
                } else if let exif::Value::Short(ref v) = f.value {
                    height = v.first().map(|&x| x as u32);
                }
            }

            // Color space
            if let Some(f) = exif.get_field(Tag::ColorSpace, In::PRIMARY) {
                if let exif::Value::Short(ref v) = f.value {
                    color_space = match v.first() {
                        Some(1) => Some("sRGB".to_string()),
                        Some(2) => Some("Adobe RGB".to_string()),
                        _ => Some(f.display_value().to_string()),
                    };
                }
            }

            // Bit depth
            if let Some(f) = exif.get_field(Tag::BitsPerSample, In::PRIMARY) {
                if let exif::Value::Short(ref v) = f.value {
                    bit_depth = v.first().map(|&x| x as u32);
                }
            }

            exif_camera_make = field_str(Tag::Make);
            exif_camera_model = field_str(Tag::Model);
            exif_lens = field_str(Tag::LensModel).or_else(|| field_str(Tag::LensSpecification));
            exif_date_taken = field_str(Tag::DateTimeOriginal).or_else(|| field_str(Tag::DateTime));

            // GPS
            exif_gps_lat = exif_gps_coord(&exif, Tag::GPSLatitude, Tag::GPSLatitudeRef, true);
            exif_gps_lng = exif_gps_coord(&exif, Tag::GPSLongitude, Tag::GPSLongitudeRef, false);
        }
    }

    Metadata {
        file_size,
        category: "image".to_string(),
        width,
        height,
        color_space,
        bit_depth,
        exif_camera_make,
        exif_camera_model,
        exif_lens,
        exif_date_taken,
        exif_gps_lat,
        exif_gps_lng,
        codec: None,
        bitrate: None,
        duration_secs: None,
        frame_rate: None,
        audio_tracks: None,
        sample_rate: None,
        channels: None,
        id3_artist: None,
        id3_album: None,
        id3_title: None,
        pdf_page_count: None,
        pdf_author: None,
        pdf_title: None,
        pdf_creation_date: None,
        pdf_version: None,
        vertex_count: None,
        face_count: None,
        material_count: None,
    }
}

fn exif_gps_coord(
    exif: &exif::Exif,
    coord_tag: exif::Tag,
    ref_tag: exif::Tag,
    is_lat: bool,
) -> Option<String> {
    let field = exif.get_field(coord_tag, exif::In::PRIMARY)?;
    let ref_field = exif.get_field(ref_tag, exif::In::PRIMARY);
    let ref_str = ref_field.map(|f| f.display_value().to_string().trim_matches('"').to_string());

    if let exif::Value::Rational(ref v) = field.value {
        if v.len() >= 3 {
            let deg = v[0].num as f64 / v[0].denom as f64;
            let min = v[1].num as f64 / v[1].denom as f64;
            let sec = v[2].num as f64 / v[2].denom as f64;
            let decimal = deg + min / 60.0 + sec / 3600.0;
            let suffix = ref_str.unwrap_or_else(|| {
                if is_lat {
                    "N".to_string()
                } else {
                    "E".to_string()
                }
            });
            return Some(format!("{:.6}° {}", decimal, suffix));
        }
    }
    None
}

fn metadata_media(path: &Path, file_size: u64, category: &str) -> Metadata {
    let path_str = path.to_string_lossy().to_string();

    let ffprobe_result = std::process::Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_streams",
            "-show_format",
            &path_str,
        ])
        .output();

    let mut codec: Option<String> = None;
    let mut bitrate: Option<u64> = None;
    let mut duration_secs: Option<f64> = None;
    let mut frame_rate: Option<String> = None;
    let mut audio_tracks: Option<u32> = None;
    let mut sample_rate: Option<u32> = None;
    let mut channels: Option<u32> = None;
    let mut id3_artist: Option<String> = None;
    let mut id3_album: Option<String> = None;
    let mut id3_title: Option<String> = None;

    let ffprobe_missing = ffprobe_result.is_err();

    if let Ok(output) = ffprobe_result {
        if output.status.success() {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                // Format-level info
                if let Some(fmt) = json.get("format") {
                    duration_secs = fmt
                        .get("duration")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse::<f64>().ok());
                    bitrate = fmt
                        .get("bit_rate")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse::<u64>().ok());

                    // ID3 tags
                    if let Some(tags) = fmt.get("tags") {
                        id3_artist = tag_value(tags, &["artist", "ARTIST"]);
                        id3_album = tag_value(tags, &["album", "ALBUM"]);
                        id3_title = tag_value(tags, &["title", "TITLE"]);
                    }
                }

                // Stream-level info
                let empty = serde_json::Value::Array(vec![]);
                let streams = json.get("streams").unwrap_or(&empty);
                let mut n_audio: u32 = 0;

                if let Some(arr) = streams.as_array() {
                    for stream in arr {
                        let stype = stream
                            .get("codec_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        match stype {
                            "video" if category == "video" => {
                                codec = stream
                                    .get("codec_name")
                                    .and_then(|v| v.as_str())
                                    .map(str::to_string);
                                // Frame rate: r_frame_rate is a fraction like "30000/1001"
                                if let Some(rfr) =
                                    stream.get("r_frame_rate").and_then(|v| v.as_str())
                                {
                                    frame_rate = Some(format_fraction_fps(rfr));
                                }
                            },
                            "audio" => {
                                n_audio += 1;
                                if codec.is_none() || category == "audio" {
                                    codec = stream
                                        .get("codec_name")
                                        .and_then(|v| v.as_str())
                                        .map(str::to_string);
                                    sample_rate = stream
                                        .get("sample_rate")
                                        .and_then(|v| v.as_str())
                                        .and_then(|s| s.parse::<u32>().ok());
                                    channels = stream
                                        .get("channels")
                                        .and_then(|v| v.as_u64())
                                        .map(|c| c as u32);
                                }
                                // Audio ID3 tags at stream level (fallback)
                                if let Some(tags) = stream.get("tags") {
                                    if id3_artist.is_none() {
                                        id3_artist = tag_value(tags, &["artist", "ARTIST"]);
                                    }
                                    if id3_album.is_none() {
                                        id3_album = tag_value(tags, &["album", "ALBUM"]);
                                    }
                                    if id3_title.is_none() {
                                        id3_title = tag_value(tags, &["title", "TITLE"]);
                                    }
                                }
                            },
                            _ => {},
                        }
                    }
                    if n_audio > 0 {
                        audio_tracks = Some(n_audio);
                    }
                }
            }
        }
    }

    // If ffprobe was missing, signal it via codec field
    if ffprobe_missing {
        codec = Some("ffprobe not found — install ffmpeg to see video/audio metadata".to_string());
    }

    Metadata {
        file_size,
        category: category.to_string(),
        width: None,
        height: None,
        color_space: None,
        bit_depth: None,
        exif_camera_make: None,
        exif_camera_model: None,
        exif_lens: None,
        exif_date_taken: None,
        exif_gps_lat: None,
        exif_gps_lng: None,
        codec,
        bitrate,
        duration_secs,
        frame_rate,
        audio_tracks,
        sample_rate,
        channels,
        id3_artist,
        id3_album,
        id3_title,
        pdf_page_count: None,
        pdf_author: None,
        pdf_title: None,
        pdf_creation_date: None,
        pdf_version: None,
        vertex_count: None,
        face_count: None,
        material_count: None,
    }
}

fn tag_value(tags: &serde_json::Value, keys: &[&str]) -> Option<String> {
    for &key in keys {
        if let Some(v) = tags.get(key).and_then(|v| v.as_str()) {
            let s = v.trim().to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }
    None
}

fn format_fraction_fps(frac: &str) -> String {
    let parts: Vec<&str> = frac.splitn(2, '/').collect();
    if parts.len() == 2 {
        if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            if den > 0.0 {
                let fps = num / den;
                // Round to 3 decimal places, trim trailing zeros
                return format!("{:.3}", fps)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
                    + " fps";
            }
        }
    }
    frac.to_string()
}

fn metadata_pdf(path: &Path, file_size: u64) -> Metadata {
    let mut pdf_page_count: Option<u32> = None;
    let mut pdf_author: Option<String> = None;
    let mut pdf_title: Option<String> = None;
    let mut pdf_creation_date: Option<String> = None;
    let mut pdf_version: Option<String> = None;

    if let Ok(doc) = lopdf::Document::load(path) {
        pdf_page_count = Some(doc.get_pages().len() as u32);
        pdf_version = Some(format!("PDF {:.1}", doc.version));

        // Info dictionary
        if let Ok(info_id) = doc.trailer.get(b"Info") {
            if let Ok(info_id) = info_id.as_reference() {
                if let Ok(info_obj) = doc.get_object(info_id) {
                    if let Ok(info_dict) = info_obj.as_dict() {
                        pdf_author = pdf_info_string(info_dict, b"Author");
                        pdf_title = pdf_info_string(info_dict, b"Title");
                        pdf_creation_date = pdf_info_string(info_dict, b"CreationDate");
                    }
                }
            }
        }
    }

    Metadata {
        file_size,
        category: "pdf".to_string(),
        width: None,
        height: None,
        color_space: None,
        bit_depth: None,
        exif_camera_make: None,
        exif_camera_model: None,
        exif_lens: None,
        exif_date_taken: None,
        exif_gps_lat: None,
        exif_gps_lng: None,
        codec: None,
        bitrate: None,
        duration_secs: None,
        frame_rate: None,
        audio_tracks: None,
        sample_rate: None,
        channels: None,
        id3_artist: None,
        id3_album: None,
        id3_title: None,
        pdf_page_count,
        pdf_author,
        pdf_title,
        pdf_creation_date,
        pdf_version,
        vertex_count: None,
        face_count: None,
        material_count: None,
    }
}

fn pdf_info_string(dict: &lopdf::Dictionary, key: &[u8]) -> Option<String> {
    let obj = dict.get(key).ok()?;
    match obj {
        lopdf::Object::String(bytes, _) => {
            // PDF strings may be UTF-16BE (BOM: 0xFE 0xFF) or PDFDocEncoding
            if bytes.starts_with(&[0xFE, 0xFF]) {
                // UTF-16BE
                let pairs: Vec<u16> = bytes[2..]
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect();
                String::from_utf16(&pairs).ok()
            } else {
                String::from_utf8(bytes.clone()).ok()
            }
        },
        _ => None,
    }
}

fn metadata_model(path: &Path, ext: &str, file_size: u64) -> Metadata {
    let (vertex_count, face_count, material_count) = match ext {
        "obj" => parse_obj(path),
        "stl" => parse_stl(path),
        "gltf" => parse_gltf(path),
        "glb" => parse_glb(path),
        _ => (None, None, None),
    };

    Metadata {
        file_size,
        category: "model".to_string(),
        width: None,
        height: None,
        color_space: None,
        bit_depth: None,
        exif_camera_make: None,
        exif_camera_model: None,
        exif_lens: None,
        exif_date_taken: None,
        exif_gps_lat: None,
        exif_gps_lng: None,
        codec: None,
        bitrate: None,
        duration_secs: None,
        frame_rate: None,
        audio_tracks: None,
        sample_rate: None,
        channels: None,
        id3_artist: None,
        id3_album: None,
        id3_title: None,
        pdf_page_count: None,
        pdf_author: None,
        pdf_title: None,
        pdf_creation_date: None,
        pdf_version: None,
        vertex_count,
        face_count,
        material_count,
    }
}

fn parse_obj(path: &Path) -> (Option<u64>, Option<u64>, Option<u32>) {
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut verts: u64 = 0;
    let mut faces: u64 = 0;
    let mut mats: u32 = 0;
    for line in content.lines() {
        if line.starts_with("v ") {
            verts += 1;
        } else if line.starts_with("f ") {
            faces += 1;
        } else if line.starts_with("mtllib ") {
            mats += 1;
        }
    }
    (
        if verts > 0 { Some(verts) } else { None },
        if faces > 0 { Some(faces) } else { None },
        if mats > 0 { Some(mats) } else { None },
    )
}

fn parse_stl(path: &Path) -> (Option<u64>, Option<u64>, Option<u32>) {
    // Check if ASCII or binary
    let header = match read_header(path, 5) {
        Some(h) => h,
        None => return (None, None, None),
    };

    if header.starts_with(b"solid") {
        // ASCII STL: count "facet normal" lines
        let content = fs::read_to_string(path).unwrap_or_default();
        let faces = content
            .lines()
            .filter(|l| l.trim_start().starts_with("facet normal"))
            .count() as u64;
        // Each face has 3 vertices
        let verts = faces * 3;
        return (
            if verts > 0 { Some(verts) } else { None },
            if faces > 0 { Some(faces) } else { None },
            None,
        );
    }

    // Binary STL: 80-byte header + u32 triangle count at byte 80
    let mut f = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return (None, None, None),
    };
    let mut buf = [0u8; 84];
    if f.read(&mut buf).unwrap_or(0) < 84 {
        return (None, None, None);
    }
    let faces = u32::from_le_bytes([buf[80], buf[81], buf[82], buf[83]]) as u64;
    let verts = faces * 3;
    (
        if verts > 0 { Some(verts) } else { None },
        if faces > 0 { Some(faces) } else { None },
        None,
    )
}

fn parse_gltf(path: &Path) -> (Option<u64>, Option<u64>, Option<u32>) {
    let content = fs::read_to_string(path).unwrap_or_default();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
        return gltf_json_counts(&json);
    }
    (None, None, None)
}

fn parse_glb(path: &Path) -> (Option<u64>, Option<u64>, Option<u32>) {
    // GLB: 12-byte file header, then JSON chunk at offset 12
    // Chunk 0: length (4 bytes LE) + type "JSON" (4 bytes) + data
    let data = match fs::read(path) {
        Ok(d) => d,
        Err(_) => return (None, None, None),
    };
    if data.len() < 20 {
        return (None, None, None);
    }
    let json_len = u32::from_le_bytes([data[12], data[13], data[14], data[15]]) as usize;
    let json_start = 20usize;
    if json_start + json_len > data.len() {
        return (None, None, None);
    }
    if let Ok(json) =
        serde_json::from_slice::<serde_json::Value>(&data[json_start..json_start + json_len])
    {
        return gltf_json_counts(&json);
    }
    (None, None, None)
}

fn gltf_json_counts(json: &serde_json::Value) -> (Option<u64>, Option<u64>, Option<u32>) {
    // Count accessors as a proxy for data (not exact vertex/face count without full parsing)
    // Use meshes→primitives→indices accessor count for faces if available
    let materials = json
        .get("materials")
        .and_then(|m| m.as_array())
        .map(|a| a.len() as u32);

    let meshes = json.get("meshes").and_then(|m| m.as_array());
    let mut total_prims: u64 = 0;
    if let Some(arr) = meshes {
        for mesh in arr {
            if let Some(prims) = mesh.get("primitives").and_then(|p| p.as_array()) {
                total_prims += prims.len() as u64;
            }
        }
    }

    // We can't easily get exact vertex/face counts without loading accessors
    // Return primitive count as a rough face proxy
    (
        None, // vertex count: would require loading binary buffer
        if total_prims > 0 {
            Some(total_prims)
        } else {
            None
        },
        materials,
    )
}

// ── IPC commands ──────────────────────────────────────────────────────────────

#[command]
fn get_initial_file() -> Option<String> {
    std::env::args().skip(1).find(|a| !a.starts_with('-'))
}

#[command]
fn get_file_info(path: String) -> Result<FileInfo, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }
    let meta = fs::metadata(p).map_err(|e| e.to_string())?;
    let name = p
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let ext = p
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
        .to_string();
    let category = category_for_ext(&ext).to_string();
    Ok(FileInfo {
        name,
        ext,
        size: meta.len(),
        category,
        path,
    })
}

#[command]
fn get_metadata(path: String) -> Result<Metadata, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File not found: {path}"));
    }
    let file_size = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
    let ext = p
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
        .to_string();
    let category = category_for_ext(&ext);

    Ok(match category {
        "image" | "image_convert" => metadata_image(p, &ext, file_size),
        "video" => metadata_media(p, file_size, "video"),
        "audio" => metadata_media(p, file_size, "audio"),
        "pdf" => metadata_pdf(p, file_size),
        "model" => metadata_model(p, &ext, file_size),
        _ => Metadata {
            file_size,
            category: category.to_string(),
            width: None,
            height: None,
            color_space: None,
            bit_depth: None,
            exif_camera_make: None,
            exif_camera_model: None,
            exif_lens: None,
            exif_date_taken: None,
            exif_gps_lat: None,
            exif_gps_lng: None,
            codec: None,
            bitrate: None,
            duration_secs: None,
            frame_rate: None,
            audio_tracks: None,
            sample_rate: None,
            channels: None,
            id3_artist: None,
            id3_album: None,
            id3_title: None,
            pdf_page_count: None,
            pdf_author: None,
            pdf_title: None,
            pdf_creation_date: None,
            pdf_version: None,
            vertex_count: None,
            face_count: None,
            material_count: None,
        },
    })
}

#[command]
fn convert_image_to_png(path: String) -> Result<String, String> {
    let tmp = format!("/tmp/librewin-mv-{}.png", tmp_tag());

    let ext = std::path::Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let src = if matches!(
        ext.as_str(),
        "heic" | "heif" | "tiff" | "tif" | "psd" | "psb"
    ) {
        format!("{path}[0]")
    } else {
        path.clone()
    };

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

#[command]
fn open_file_dialog() -> Option<String> {
    rfd::FileDialog::new()
        .add_filter(
            "Media",
            &[
                "png", "jpg", "jpeg", "gif", "webp", "bmp", "ico", "svg", "avif", "heic", "heif",
                "tiff", "tif", "raw", "psd", "mp4", "mkv", "webm", "avi", "mov", "m4v", "mp3",
                "flac", "ogg", "wav", "aac", "m4a", "opus", "pdf", "obj", "gltf", "glb", "stl",
            ],
        )
        .add_filter("All files", &["*"])
        .pick_file()
        .map(|p| p.to_string_lossy().to_string())
}

// ── Recent files ──────────────────────────────────────────────────────────────

fn recent_files_path() -> Option<std::path::PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let dir = std::path::PathBuf::from(&home).join(".config/librewin");
    fs::create_dir_all(&dir).ok()?;
    Some(dir.join("prism-recent.json"))
}

#[command]
fn get_recent_files() -> Vec<String> {
    let path = match recent_files_path() {
        Some(p) => p,
        None => return vec![],
    };
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str::<Vec<String>>(&content).unwrap_or_default()
}

#[command]
fn add_recent_file(path: String) -> Result<(), String> {
    let rpath = recent_files_path().ok_or("Cannot determine config dir")?;
    let content = fs::read_to_string(&rpath).unwrap_or_default();
    let mut list: Vec<String> = serde_json::from_str(&content).unwrap_or_default();

    // Remove existing entry (dedup), then prepend
    list.retain(|p| p != &path);
    list.insert(0, path);
    list.truncate(20);

    let json = serde_json::to_string(&list).map_err(|e| e.to_string())?;
    fs::write(&rpath, json).map_err(|e| e.to_string())
}

// ── Directory listing for prev/next navigation ─────────────────────────────

#[command]
fn list_dir_files(dir: String, category: String) -> Vec<String> {
    let dir_path = Path::new(&dir);
    let entries = match fs::read_dir(dir_path) {
        Ok(e) => e,
        Err(_) => return vec![],
    };

    // Normalize category group: image + image_convert both treated as "image"
    let group = if category == "image_convert" {
        "image"
    } else {
        &category
    };

    let mut files: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let p = e.path();
            if !p.is_file() {
                return None;
            }
            let ext = p.extension()?.to_string_lossy().to_lowercase().to_string();
            let cat = category_for_ext(&ext);
            let cat_group = if cat == "image_convert" { "image" } else { cat };
            if cat_group == group {
                Some(p.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    files.sort();
    files
}

// ── Stream handler ────────────────────────────────────────────────────────────

fn handle_stream(req: tauri::http::Request<Vec<u8>>) -> tauri::http::Response<Vec<u8>> {
    let uri = req.uri().to_string();
    let path_encoded = uri.strip_prefix("mvstream://localhost").unwrap_or_default();
    let path_str = decode_path(path_encoded);
    let path = Path::new(&path_str);

    if !path.exists() {
        return tauri::http::Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body(b"Not found".to_vec())
            .unwrap();
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let mime = mime_for_ext(&ext);

    let file_size = match fs::metadata(path) {
        Ok(m) => m.len(),
        Err(e) => {
            return tauri::http::Response::builder()
                .status(500)
                .body(format!("Metadata error: {e}").into_bytes())
                .unwrap();
        },
    };

    let range = req
        .headers()
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
                end_str
                    .parse::<u64>()
                    .ok()?
                    .min(file_size.saturating_sub(1))
            };
            if start > end {
                return None;
            }
            Some((start, end))
        });

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return tauri::http::Response::builder()
                .status(500)
                .body(format!("Open error: {e}").into_bytes())
                .unwrap();
        },
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
fn get_theme() -> String {
    lw_get_theme()
}

#[tauri::command]
fn get_accent() -> String {
    lw_get_accent()
}

// ── App entry ─────────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("mvstream", |_app, req| handle_stream(req))
        .invoke_handler(tauri::generate_handler![
            get_initial_file,
            get_file_info,
            get_metadata,
            convert_image_to_png,
            open_file_dialog,
            get_recent_files,
            add_recent_file,
            list_dir_files,
            get_theme,
            get_accent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running prism");
}

// ── Tests ─────────────────────────────────────────────────────────────────────

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

    #[test]
    fn format_fraction_fps_common_rates() {
        assert_eq!(format_fraction_fps("30/1"), "30 fps");
        assert_eq!(format_fraction_fps("30000/1001"), "29.97 fps");
        assert_eq!(format_fraction_fps("24/1"), "24 fps");
        assert_eq!(format_fraction_fps("25/1"), "25 fps");
    }

    #[test]
    fn png_dimensions_from_header() {
        // Minimal 1×1 white PNG (valid IHDR at bytes 16-23)
        let fixture = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tests/fixtures/1x1.png"
        ));
        if fixture.exists() {
            let (w, h) = image_dimensions_from_header(fixture, "png");
            assert_eq!(w, Some(1));
            assert_eq!(h, Some(1));
        }
    }

    #[test]
    fn metadata_pdf_page_count() {
        let fixture = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tests/fixtures/minimal.pdf"
        ));
        if fixture.exists() {
            let m = metadata_pdf(fixture, 0);
            assert!(m.pdf_page_count.is_some());
            assert!(m.pdf_page_count.unwrap() >= 1);
        }
    }

    #[test]
    fn metadata_obj_counts() {
        let fixture = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tests/fixtures/triangle.obj"
        ));
        if fixture.exists() {
            let (v, f, _) = parse_obj(fixture);
            assert_eq!(v, Some(3));
            assert_eq!(f, Some(1));
        }
    }

    #[test]
    fn recent_files_roundtrip() {
        // Only run when HOME is set and writable
        if std::env::var("HOME").is_err() {
            return;
        }
        // add + get
        let _ = add_recent_file("/tmp/test-prism-recent.png".to_string());
        let list = get_recent_files();
        assert!(list.contains(&"/tmp/test-prism-recent.png".to_string()));
    }
}
