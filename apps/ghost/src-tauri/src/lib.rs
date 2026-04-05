use tauri::Emitter;
use tauri::webview::DownloadEvent;

#[tauri::command]
fn close_window(window: tauri::Window) {
    let _ = window.close();
}

#[tauri::command]
fn minimize_window(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn toggle_maximize(window: tauri::Window) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

/// Launch a system browser by short name and detach immediately.
/// The frontend calls this when the user clicks Chrome / Firefox / Brave
/// in the new-tab page, then closes Ghost so the handoff feels instant.
#[tauri::command]
fn launch_app(app: String) {
    let binary: &str = match app.as_str() {
        "chrome"  => "google-chrome-stable",
        "firefox" => "firefox",
        "brave"   => "brave-browser",
        _         => return,
    };
    let _ = std::process::Command::new(binary)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

// Toolbar JS injected into every page the WebView loads.
// On the app's own new-tab page (tauri:// protocol) we skip injection —
// Svelte handles the chrome there.  On any external page we inject a
// minimal nav toolbar: Windows-style controls (─ □ ×) on the RIGHT,
// Ghost icon + name on the LEFT, back/forward/reload + URL bar in between.
const TOOLBAR_INIT_SCRIPT: &str = r#"
;(function () {
  if (location.protocol === 'tauri:') return; // new-tab page handles its own chrome
  if (document.getElementById('__lw_ghost_tb')) return; // already injected

  var H = 46;
  var ACCENT = '#0066cc';

  var style = document.createElement('style');
  style.textContent = [
    '#__lw_ghost_tb{position:fixed;top:0;left:0;right:0;height:'+H+'px;',
    'background:#f8fafc;border-bottom:1px solid #e2e8f0;',
    'display:flex;align-items:center;gap:0;padding:0;',
    'z-index:2147483647;-webkit-app-region:drag;box-sizing:border-box;',
    'font-family:system-ui,sans-serif;}',
    '#__lw_ghost_tb *{-webkit-app-region:no-drag;box-sizing:border-box;}',
    /* Brand label on left */
    '#__lw_ghost_brand{display:flex;align-items:center;gap:6px;padding:0 8px 0 12px;',
    'flex-shrink:0;-webkit-app-region:drag;}',
    '#__lw_ghost_brand span{font-size:13px;font-weight:500;color:#374151;',
    '-webkit-app-region:drag;}',
    /* Nav + address bar area */
    '#__lw_ghost_nav{display:flex;align-items:center;gap:2px;flex:1;min-width:0;padding:0 4px;}',
    '#__lw_ghost_tb .nb{border:none;background:none;cursor:pointer;border-radius:6px;',
    'padding:4px 6px;font-size:15px;color:#6b7280;line-height:1;}',
    '#__lw_ghost_tb .nb:hover{background:#e5e7eb;color:#111827;}',
    '#__lw_url_bar{flex:1;height:28px;border:1px solid #d1d5db;border-radius:14px;',
    'padding:0 12px;font-size:13px;background:white;color:#111827;outline:none;min-width:0;margin:0 4px;}',
    '#__lw_url_bar:focus{border-color:'+ACCENT+';box-shadow:0 0 0 3px rgba(0,102,204,0.15);}',
    '#__lw_ghost_go{flex-shrink:0;height:28px;padding:0 10px;background:'+ACCENT+';',
    'color:white;font-size:12px;font-weight:500;border:none;border-radius:14px;cursor:pointer;}',
    '#__lw_ghost_go:hover{opacity:0.9;}',
    /* Windows-style window controls on right */
    '#__lw_ghost_wc{display:flex;align-items:center;flex-shrink:0;height:100%;}',
    '#__lw_ghost_tb .wc{width:44px;height:100%;border:none;background:none;cursor:pointer;',
    'font-size:14px;color:#6b7280;display:flex;align-items:center;justify-content:center;}',
    '#__lw_ghost_tb .wc:hover{background:#e5e7eb;color:#111827;}',
    '#__lw_ghost_cls:hover{background:#ef4444!important;color:white!important;}',
    'body{padding-top:'+H+'px!important;margin-top:0!important;}'
  ].join('');
  document.head.appendChild(style);

  /* Ghost icon SVG (14×14) */
  var ghostSvg = '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="'+ACCENT+'" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" style="-webkit-app-region:drag;flex-shrink:0">'
    + '<path d="M12 3a7 7 0 0 0-7 7v9l2.5-2 2.5 2 2.5-2 2.5 2 2.5-2V10a7 7 0 0 0-7-7z"/>'
    + '<circle cx="9.5" cy="11" r="1" fill="'+ACCENT+'" stroke="none"/>'
    + '<circle cx="14.5" cy="11" r="1" fill="'+ACCENT+'" stroke="none"/>'
    + '</svg>';

  var tb = document.createElement('div');
  tb.id = '__lw_ghost_tb';
  tb.innerHTML =
    /* Left: brand */
    '<div id="__lw_ghost_brand">' + ghostSvg + '<span>Ghost</span></div>' +
    /* Center: nav + address */
    '<div id="__lw_ghost_nav">' +
      '<button class="nb" id="__lw_ghost_back" title="Back">\u2039</button>' +
      '<button class="nb" id="__lw_ghost_fwd" title="Forward">\u203a</button>' +
      '<button class="nb" id="__lw_ghost_rld" title="Reload">\u21ba</button>' +
      '<input id="__lw_url_bar" type="text" value="' + location.href + '" placeholder="Search or enter address\u2026"/>' +
      '<button id="__lw_ghost_go">Go</button>' +
    '</div>' +
    /* Right: window controls — ─ □ × */
    '<div id="__lw_ghost_wc">' +
      '<button class="wc" id="__lw_ghost_min" title="Minimize">\u2500</button>' +
      '<button class="wc" id="__lw_ghost_max" title="Maximize">\u25a1</button>' +
      '<button class="wc" id="__lw_ghost_cls" title="Close">\u00d7</button>' +
    '</div>';

  if (document.body) {
    document.body.insertBefore(tb, document.body.firstChild);
  } else {
    document.addEventListener('DOMContentLoaded', function () {
      document.body.insertBefore(tb, document.body.firstChild);
    });
  }

  /* Window controls */
  document.getElementById('__lw_ghost_cls').onclick = function () {
    if (window.__TAURI__ && window.__TAURI__.core) window.__TAURI__.core.invoke('close_window');
  };
  document.getElementById('__lw_ghost_min').onclick = function () {
    if (window.__TAURI__ && window.__TAURI__.core) window.__TAURI__.core.invoke('minimize_window');
  };
  document.getElementById('__lw_ghost_max').onclick = function () {
    if (window.__TAURI__ && window.__TAURI__.core) window.__TAURI__.core.invoke('toggle_maximize');
  };

  /* Navigation */
  document.getElementById('__lw_ghost_back').onclick = function () { history.back(); };
  document.getElementById('__lw_ghost_fwd').onclick = function () { history.forward(); };
  document.getElementById('__lw_ghost_rld').onclick = function () { location.reload(); };

  var urlBar = document.getElementById('__lw_url_bar');
  document.getElementById('__lw_ghost_go').onclick = function () { go(); };
  urlBar.addEventListener('focus', function () { urlBar.select(); });
  urlBar.addEventListener('keydown', function (e) {
    if (e.key === 'Enter') go();
    if (e.key === 'Escape') urlBar.blur();
  });

  function go() {
    var u = urlBar.value.trim();
    if (!u) return;
    if (/^javascript:/i.test(u) || /^data:/i.test(u) || /^vbscript:/i.test(u)) return;
    if (!u.includes('.') || u.includes(' ')) {
      u = 'https://duckduckgo.com/?q=' + encodeURIComponent(u);
    } else if (!/^https?:\/\//i.test(u)) {
      u = 'https://' + u;
    }
    location.href = u;
  }

  window.addEventListener('popstate', function () {
    var h = location.href;
    if (/^javascript:/i.test(h) || /^data:/i.test(h)) h = '';
    urlBar.value = h;
  });
})();
"#;

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
        .invoke_handler(tauri::generate_handler![
            close_window,
            minimize_window,
            toggle_maximize,
            launch_app,
            get_theme,
            get_accent,
        ])
        .setup(|app| {
            let config = app.config();
            let win_config = config.app.windows.first().cloned()
                .unwrap_or_else(|| tauri::utils::config::WindowConfig {
                    label: "main".into(),
                    title: "Ghost".into(),
                    width: 1280.0,
                    height: 800.0,
                    decorations: false,
                    resizable: true,
                    center: true,
                    ..Default::default()
                });

            tauri::WebviewWindowBuilder::from_config(app, &win_config)?
                .initialization_script(TOOLBAR_INIT_SCRIPT)
                .on_download(|webview, event| {
                    match event {
                        DownloadEvent::Requested { url: _, destination } => {
                            // Route all downloads to ~/Downloads, preserving the suggested filename
                            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                            let filename = destination
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("download")
                                .to_string();
                            *destination = std::path::PathBuf::from(&home)
                                .join("Downloads")
                                .join(&filename);
                            true // allow the download
                        }
                        DownloadEvent::Finished { url: _, path, success } => {
                            if success {
                                let filename = path
                                    .as_ref()
                                    .and_then(|p| p.file_name())
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("file")
                                    .to_string();
                                // Emit to the Ghost frontend so the toast appears
                                let _ = webview.emit("download-complete", filename);
                            }
                            false
                        }
                        _ => true,
                    }
                })
                .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running ghost");
}
