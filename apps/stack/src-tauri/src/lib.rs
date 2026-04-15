use librewin_common::{get_accent as lw_get_accent, get_theme as lw_get_theme};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ── File I/O ──────────────────────────────────────────────────────────────────

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Failed to read {path}: {e}"))
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    // Ensure parent directory exists
    if let Some(parent) = PathBuf::from(&path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {e}"))?;
    }
    std::fs::write(&path, content).map_err(|e| format!("Failed to write {path}: {e}"))
}

// ── File dialogs (rfd — blocking GTK3, fine on Linux from a command thread) ──

#[derive(Serialize, Deserialize)]
pub struct OpenedFile {
    pub path: String,
    pub content: String,
}

#[tauri::command]
async fn open_file_dialog() -> Result<Option<OpenedFile>, String> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("Open File")
        .add_filter("Text & Markdown", &["md", "markdown", "txt", "text"])
        .add_filter("All Files", &["*"])
        .pick_file()
        .await;

    match path {
        None => Ok(None),
        Some(handle) => {
            let path_str = handle.path().to_string_lossy().into_owned();
            let content = std::fs::read_to_string(&path_str)
                .map_err(|e| format!("Could not read file: {e}"))?;
            Ok(Some(OpenedFile {
                path: path_str,
                content,
            }))
        },
    }
}

#[tauri::command]
async fn save_file_dialog(default_name: Option<String>) -> Result<Option<String>, String> {
    let mut dialog = rfd::AsyncFileDialog::new()
        .set_title("Save File")
        .add_filter("Markdown", &["md"])
        .add_filter("Text", &["txt"])
        .add_filter("HTML", &["html"])
        .add_filter("All Files", &["*"]);

    if let Some(name) = default_name {
        dialog = dialog.set_file_name(&name);
    }

    let path = dialog.save_file().await;
    Ok(path.map(|h| h.path().to_string_lossy().into_owned()))
}

// ── HTML export via pulldown-cmark ────────────────────────────────────────────

#[tauri::command]
fn export_html(content: String) -> Result<String, String> {
    use pulldown_cmark::{html, Options, Parser};

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(&content, opts);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Wrap in a minimal styled document
    let document = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
<title>Export</title>
<style>
  body {{ max-width: 720px; margin: 40px auto; padding: 0 24px;
         font-family: system-ui, sans-serif; line-height: 1.65;
         color: #111827; background: #fff; }}
  h1,h2,h3,h4,h5,h6 {{ color: #0066cc; margin-top: 1.5em; }}
  pre {{ background: #f7f7f8; border-radius: 6px; padding: 12px; overflow-x: auto; }}
  code {{ font-family: monospace; font-size: 0.9em; }}
  blockquote {{ border-left: 3px solid #0066cc; margin-left: 0; padding-left: 16px; color: #6b7280; }}
  table {{ border-collapse: collapse; width: 100%; }}
  th, td {{ border: 1px solid #e2e4e8; padding: 8px 12px; text-align: left; }}
  th {{ background: #f7f7f8; font-weight: 600; }}
  a {{ color: #0066cc; }}
  img {{ max-width: 100%; }}
</style>
</head>
<body>
{html_output}
</body>
</html>"#
    );

    Ok(document)
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
        .invoke_handler(tauri::generate_handler![
            read_file,
            write_file,
            open_file_dialog,
            save_file_dialog,
            export_html,
            librewin_common::window::close_window,
            librewin_common::window::minimize_window,
            librewin_common::window::toggle_maximize,
            get_theme,
            get_accent,
        ])
        .run(tauri::generate_context!())
        .expect("error while running stack");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_html_wraps_in_doctype() {
        let result = export_html("hello".to_string()).unwrap();
        assert!(result.contains("<!DOCTYPE html>"));
        assert!(result.contains("<body>"));
        assert!(result.contains("</body>"));
    }

    #[test]
    fn export_html_renders_heading() {
        let result = export_html("# Hello World".to_string()).unwrap();
        assert!(result.contains("<h1>Hello World</h1>"));
    }

    #[test]
    fn export_html_renders_paragraph() {
        let result = export_html("Just a paragraph.".to_string()).unwrap();
        assert!(result.contains("<p>Just a paragraph.</p>"));
    }

    #[test]
    fn export_html_renders_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let result = export_html(md.to_string()).unwrap();
        assert!(result.contains("<table>"));
        assert!(result.contains("<th>A</th>") || result.contains(">A<"));
    }

    #[test]
    fn export_html_empty_input_produces_valid_document() {
        let result = export_html(String::new()).unwrap();
        assert!(result.contains("<!DOCTYPE html>"));
        assert!(result.contains("<html"));
        assert!(result.contains("</html>"));
    }
}
