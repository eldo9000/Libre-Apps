#[tauri::command]
fn save_theme(
    accent: String,
    accent_hover: String,
    font_family: String,
    font_size: u32,
) -> Result<(), String> {
    let tokens_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| "could not resolve tokens path".to_string())?
        .join("common-js/src/tokens.css");

    let content = std::fs::read_to_string(&tokens_path).map_err(|e| e.to_string())?;

    let mut in_root = false;
    let patched: Vec<String> = content
        .lines()
        .map(|line| {
            let t = line.trim();
            if t == ":root {" {
                in_root = true;
            } else if in_root && t == "}" {
                in_root = false;
            }
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            if t.starts_with("--accent:") && !t.starts_with("--accent-hover") {
                format!("{}--accent:          {};", indent, accent)
            } else if t.starts_with("--accent-hover:") {
                format!("{}--accent-hover:    {};", indent, accent_hover)
            } else if in_root && t.starts_with("font-family:") {
                format!("{}font-family: {};", indent, font_family)
            } else if in_root && t.starts_with("font-size:") {
                format!("{}font-size: {}px;", indent, font_size)
            } else {
                line.to_string()
            }
        })
        .collect();

    std::fs::write(&tokens_path, patched.join("\n")).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_preset(path: String, content: String) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_preset(path: String) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_theme() -> String {
    librewin_common::get_theme()
}

#[tauri::command]
fn get_accent() -> String {
    librewin_common::get_accent()
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            save_theme,
            write_preset,
            read_preset,
            get_theme,
            get_accent,
            librewin_common::window::close_window,
            librewin_common::window::minimize_window,
            librewin_common::window::toggle_maximize,
        ])
        .run(tauri::generate_context!())
        .expect("error while running gallery");
}
