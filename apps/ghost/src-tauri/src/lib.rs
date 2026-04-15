use librewin_common::{get_accent as lw_get_accent, get_theme as lw_get_theme};
use std::collections::HashSet;
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc,
};
use tauri::webview::DownloadEvent;
use tauri::Emitter;

// ── Blocklist ─────────────────────────────────────────────────────────────────

const BLOCKLIST_JSON: &str = include_str!("../assets/blocklist.json");

fn load_blocklist() -> HashSet<String> {
    serde_json::from_str::<Vec<String>>(BLOCKLIST_JSON)
        .unwrap_or_default()
        .into_iter()
        .collect()
}

// ── Privacy state ─────────────────────────────────────────────────────────────

pub struct PrivacyStats {
    pub trackers_blocked: AtomicU32,
    pub cookies_blocked: AtomicU32,
}

pub struct AppState {
    pub stats: Arc<PrivacyStats>,
    pub tracker_blocking_enabled: AtomicBool,
    pub blocklist: HashSet<String>,
}

impl AppState {
    fn new(blocklist: HashSet<String>) -> Self {
        Self {
            stats: Arc::new(PrivacyStats {
                trackers_blocked: AtomicU32::new(0),
                cookies_blocked: AtomicU32::new(0),
            }),
            tracker_blocking_enabled: AtomicBool::new(true),
            blocklist,
        }
    }
}

// ── Privacy init script — runs before page JS on every external page ──────────
// Intercepts fetch() and XHR to block known tracker domains.
// Reads the blocklist from window.__LW_GHOST_BLOCKLIST__ (injected at startup).
const PRIVACY_INIT_SCRIPT: &str = r#"
;(function () {
  if (location.protocol === 'tauri:') return;
  var BLOCKED_HOSTS = window.__LW_GHOST_BLOCKLIST__ || [];
  if (!BLOCKED_HOSTS.length) return;
  var blocked = new Set(BLOCKED_HOSTS);

  function isBlocked(url) {
    try {
      var host = new URL(url, location.href).hostname.replace(/^www\./, '');
      return blocked.has(host) || Array.from(blocked).some(function (d) { return host.endsWith('.' + d); });
    } catch (e) { return false; }
  }

  // Block fetch()
  var origFetch = window.fetch;
  window.fetch = function (url, opts) {
    if (typeof url === 'string' && isBlocked(url)) {
      window.__lw_ghost_tracker_blocked && window.__lw_ghost_tracker_blocked();
      return Promise.reject(new Error('Blocked by Ghost'));
    }
    return origFetch.apply(this, arguments);
  };

  // Block XMLHttpRequest
  var origOpen = window.XMLHttpRequest.prototype.open;
  window.XMLHttpRequest.prototype.open = function (method, url) {
    if (typeof url === 'string' && isBlocked(url)) {
      window.__lw_ghost_tracker_blocked && window.__lw_ghost_tracker_blocked();
      this.__lw_blocked = true;
    }
    return origOpen.apply(this, arguments);
  };
  var origSend = window.XMLHttpRequest.prototype.send;
  window.XMLHttpRequest.prototype.send = function () {
    if (this.__lw_blocked) return;
    return origSend.apply(this, arguments);
  };
})();
"#;

// ── Toolbar init script — injected into every external page ──────────────────
// Provides nav toolbar, window controls, HTTP warning, and tracker callback.
const TOOLBAR_INIT_SCRIPT: &str = r#"
;(function () {
  if (location.protocol === 'tauri:') return; // new-tab page handles its own chrome
  if (document.getElementById('__lw_ghost_tb')) return; // already injected

  var H = 46;
  var ACCENT = '#0066cc';

  // Relay tracker blocks to Rust counter
  window.__lw_ghost_tracker_blocked = function () {
    if (window.__TAURI__ && window.__TAURI__.core) {
      window.__TAURI__.core.invoke('increment_tracker_count');
    }
  };

  var style = document.createElement('style');
  style.textContent = [
    '#__lw_ghost_tb{position:fixed;top:0;left:0;right:0;height:'+H+'px;',
    'background:#f8fafc;border-bottom:1px solid #e2e8f0;',
    'display:flex;align-items:center;gap:0;padding:0;',
    'z-index:2147483647;-webkit-app-region:drag;box-sizing:border-box;',
    'font-family:system-ui,sans-serif;}',
    '#__lw_ghost_tb *{-webkit-app-region:no-drag;box-sizing:border-box;}',
    '#__lw_ghost_brand{display:flex;align-items:center;gap:6px;padding:0 8px 0 12px;',
    'flex-shrink:0;-webkit-app-region:drag;}',
    '#__lw_ghost_brand span{font-size:13px;font-weight:500;color:#374151;',
    '-webkit-app-region:drag;}',
    '#__lw_ghost_nav{display:flex;align-items:center;gap:2px;flex:1;min-width:0;padding:0 4px;}',
    '#__lw_ghost_tb .nb{border:none;background:none;cursor:pointer;border-radius:6px;',
    'padding:4px 6px;font-size:15px;color:#6b7280;line-height:1;}',
    '#__lw_ghost_tb .nb:hover{background:#e5e7eb;color:#111827;}',
    '#__lw_url_bar{flex:1;height:28px;border:1px solid #d1d5db;border-radius:14px;',
    'padding:0 12px;font-size:13px;background:white;color:#111827;outline:none;min-width:0;margin:0 4px;}',
    '#__lw_url_bar:focus{border-color:'+ACCENT+';box-shadow:0 0 0 3px rgba(0,102,204,0.15);}',
    '#__lw_ghost_proto{flex-shrink:0;font-size:11px;font-weight:600;padding:2px 6px;',
    'border-radius:4px;margin-right:2px;}',
    '#__lw_ghost_go{flex-shrink:0;height:28px;padding:0 10px;background:'+ACCENT+';',
    'color:white;font-size:12px;font-weight:500;border:none;border-radius:14px;cursor:pointer;}',
    '#__lw_ghost_go:hover{opacity:0.9;}',
    '#__lw_ghost_wc{display:flex;align-items:center;flex-shrink:0;height:100%;}',
    '#__lw_ghost_tb .wc{width:44px;height:100%;border:none;background:none;cursor:pointer;',
    'font-size:14px;color:#6b7280;display:flex;align-items:center;justify-content:center;}',
    '#__lw_ghost_tb .wc:hover{background:#e5e7eb;color:#111827;}',
    '#__lw_ghost_cls:hover{background:#ef4444!important;color:white!important;}',
    'body{padding-top:'+H+'px!important;margin-top:0!important;}'
  ].join('');
  document.head.appendChild(style);

  var ghostSvg = '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="'+ACCENT+'" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" style="-webkit-app-region:drag;flex-shrink:0">'
    + '<path d="M12 3a7 7 0 0 0-7 7v9l2.5-2 2.5 2 2.5-2 2.5 2 2.5-2V10a7 7 0 0 0-7-7z"/>'
    + '<circle cx="9.5" cy="11" r="1" fill="'+ACCENT+'" stroke="none"/>'
    + '<circle cx="14.5" cy="11" r="1" fill="'+ACCENT+'" stroke="none"/>'
    + '</svg>';

  // Build toolbar via createElement to avoid reflecting location.href into innerHTML
  var tb = document.createElement('div');
  tb.id = '__lw_ghost_tb';

  // Left: brand
  var brand = document.createElement('div');
  brand.id = '__lw_ghost_brand';
  brand.innerHTML = ghostSvg + '<span>Ghost</span>';
  tb.appendChild(brand);

  // Center: nav + address
  var nav = document.createElement('div');
  nav.id = '__lw_ghost_nav';

  var btnBack = document.createElement('button');
  btnBack.className = 'nb'; btnBack.id = '__lw_ghost_back'; btnBack.title = 'Back'; btnBack.textContent = '\u2039';
  var btnFwd = document.createElement('button');
  btnFwd.className = 'nb'; btnFwd.id = '__lw_ghost_fwd'; btnFwd.title = 'Forward'; btnFwd.textContent = '\u203a';
  var btnRld = document.createElement('button');
  btnRld.className = 'nb'; btnRld.id = '__lw_ghost_rld'; btnRld.title = 'Reload'; btnRld.textContent = '\u21ba';

  // Protocol indicator
  var proto = document.createElement('span');
  proto.id = '__lw_ghost_proto';
  updateProto(proto);

  // URL bar — value set as DOM property, never via innerHTML
  var urlBar = document.createElement('input');
  urlBar.id = '__lw_url_bar';
  urlBar.type = 'text';
  urlBar.placeholder = 'Search or enter address\u2026';
  urlBar.value = location.href;

  var btnGo = document.createElement('button');
  btnGo.id = '__lw_ghost_go'; btnGo.textContent = 'Go';

  nav.appendChild(btnBack); nav.appendChild(btnFwd); nav.appendChild(btnRld);
  nav.appendChild(proto); nav.appendChild(urlBar); nav.appendChild(btnGo);
  tb.appendChild(nav);

  // Right: window controls
  var wc = document.createElement('div');
  wc.id = '__lw_ghost_wc';
  ['min\u2500', 'max\u25a1', 'cls\u00d7'].forEach(function (s) {
    var btn = document.createElement('button');
    btn.className = 'wc'; btn.id = '__lw_ghost_' + s.slice(0,3); btn.textContent = s.slice(3);
    btn.title = { min: 'Minimize', max: 'Maximize', cls: 'Close' }[s.slice(0,3)];
    wc.appendChild(btn);
  });
  tb.appendChild(wc);

  if (document.body) {
    document.body.insertBefore(tb, document.body.firstChild);
  } else {
    document.addEventListener('DOMContentLoaded', function () {
      document.body.insertBefore(tb, document.body.firstChild);
    });
  }

  // HTTP warning banner
  if (location.protocol === 'http:' && location.hostname !== 'localhost') {
    var warn = document.createElement('div');
    warn.id = '__lw_ghost_http_warn';
    warn.style.cssText = 'position:fixed;top:'+H+'px;left:0;right:0;z-index:2147483646;'
      + 'background:#fef3c7;border-bottom:1px solid #f59e0b;padding:6px 16px;'
      + 'font-family:system-ui,sans-serif;font-size:12px;color:#92400e;'
      + 'display:flex;align-items:center;gap:8px;-webkit-app-region:no-drag;';
    warn.innerHTML = '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>'
      + '<span>This page is <strong>not secure</strong> (HTTP). Your connection is unencrypted.</span>'
      + '<button onclick="this.parentNode.remove()" style="margin-left:auto;background:none;border:none;cursor:pointer;font-size:16px;color:#92400e;padding:0 4px;">\u00d7</button>';
    document.body.style.setProperty('padding-top', (H + 36) + 'px', 'important');
    document.body.appendChild(warn);
  }

  // Window controls
  document.getElementById('__lw_ghost_cls').onclick = function () {
    if (window.__TAURI__ && window.__TAURI__.core) window.__TAURI__.core.invoke('close_window');
  };
  document.getElementById('__lw_ghost_min').onclick = function () {
    if (window.__TAURI__ && window.__TAURI__.core) window.__TAURI__.core.invoke('minimize_window');
  };
  document.getElementById('__lw_ghost_max').onclick = function () {
    if (window.__TAURI__ && window.__TAURI__.core) window.__TAURI__.core.invoke('toggle_maximize');
  };

  // Navigation
  document.getElementById('__lw_ghost_back').onclick = function () { history.back(); };
  document.getElementById('__lw_ghost_fwd').onclick = function () { history.forward(); };
  document.getElementById('__lw_ghost_rld').onclick = function () { location.reload(); };

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

  function updateProto(el) {
    var isHttps = location.protocol === 'https:';
    el.textContent = isHttps ? '\uD83D\uDD12' : '\u26A0\uFE0F';
    el.style.color = isHttps ? '#16a34a' : '#d97706';
    el.title = isHttps ? 'Secure (HTTPS)' : 'Not secure (HTTP)';
  }

  window.addEventListener('popstate', function () {
    var h = location.href;
    if (/^javascript:/i.test(h) || /^data:/i.test(h)) h = '';
    urlBar.value = h;
    updateProto(document.getElementById('__lw_ghost_proto'));
  });
})();
"#;

// ── System browser launcher ────────────────────────────────────────────────────

#[tauri::command]
fn launch_app(app: String) {
    let binary: &str = match app.as_str() {
        "chrome" => "google-chrome-stable",
        "firefox" => "firefox",
        "brave" => "brave-browser",
        _ => return,
    };
    let _ = std::process::Command::new(binary)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
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

// ── Privacy IPC commands ───────────────────────────────────────────────────────

#[tauri::command]
fn get_privacy_stats(state: tauri::State<'_, AppState>) -> serde_json::Value {
    serde_json::json!({
        "trackers_blocked": state.stats.trackers_blocked.load(Ordering::Relaxed),
        "cookies_blocked":  state.stats.cookies_blocked.load(Ordering::Relaxed),
    })
}

#[tauri::command]
fn set_tracker_blocking(enabled: bool, state: tauri::State<'_, AppState>) {
    state
        .tracker_blocking_enabled
        .store(enabled, Ordering::Relaxed);
}

#[tauri::command]
fn reset_privacy_stats(state: tauri::State<'_, AppState>) {
    state.stats.trackers_blocked.store(0, Ordering::Relaxed);
    state.stats.cookies_blocked.store(0, Ordering::Relaxed);
}

#[tauri::command]
fn increment_tracker_count(state: tauri::State<'_, AppState>) {
    state.stats.trackers_blocked.fetch_add(1, Ordering::Relaxed);
}

#[tauri::command]
async fn clear_browsing_data(window: tauri::WebviewWindow) -> Result<(), String> {
    window
        .eval("try { sessionStorage.clear(); localStorage.clear(); } catch(e) {}")
        .map_err(|e| e.to_string())
}

// ── Entry point ────────────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Build the combined init script with the embedded blocklist.
    let blocklist = load_blocklist();
    let blocklist_json = {
        let domains: Vec<&str> = blocklist.iter().map(String::as_str).collect();
        serde_json::to_string(&domains).unwrap_or_else(|_| "[]".to_string())
    };
    let combined_init = format!(
        "window.__LW_GHOST_BLOCKLIST__ = {};\n{}\n{}",
        blocklist_json, PRIVACY_INIT_SCRIPT, TOOLBAR_INIT_SCRIPT
    );

    let session_dir = std::env::temp_dir().join(format!("ghost-session-{}", std::process::id()));

    tauri::Builder::default()
        .manage(AppState::new(blocklist))
        .invoke_handler(tauri::generate_handler![
            librewin_common::window::close_window,
            librewin_common::window::minimize_window,
            librewin_common::window::toggle_maximize,
            launch_app,
            get_theme,
            get_accent,
            get_privacy_stats,
            set_tracker_blocking,
            reset_privacy_stats,
            increment_tracker_count,
            clear_browsing_data,
        ])
        .setup(move |app| {
            std::fs::create_dir_all(&session_dir).expect("failed to create ghost session dir");

            let config = app.config();
            let win_config = config.app.windows.first().cloned().unwrap_or_else(|| {
                tauri::utils::config::WindowConfig {
                    label: "main".into(),
                    title: "Ghost".into(),
                    width: 1280.0,
                    height: 800.0,
                    decorations: false,
                    resizable: true,
                    center: true,
                    ..Default::default()
                }
            });

            let window = tauri::WebviewWindowBuilder::from_config(app, &win_config)?
                .initialization_script(&combined_init)
                .data_directory(session_dir.clone())
                .on_download(|webview, event| match event {
                    DownloadEvent::Requested {
                        url: _,
                        destination,
                    } => {
                        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                        let filename = destination
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("download")
                            .to_string();
                        *destination = std::path::PathBuf::from(&home)
                            .join("Downloads")
                            .join(&filename);
                        true
                    },
                    DownloadEvent::Finished {
                        url: _,
                        path,
                        success,
                    } => {
                        if success {
                            let filename = path
                                .as_ref()
                                .and_then(|p| p.file_name())
                                .and_then(|n| n.to_str())
                                .unwrap_or("file")
                                .to_string();
                            let _ = webview.emit("download-complete", filename);
                        }
                        false
                    },
                    _ => true,
                })
                .build()?;

            // Apply WebKit2GTK-specific privacy settings on Linux.
            #[cfg(target_os = "linux")]
            {
                use webkit2gtk::prelude::*;
                window
                    .with_webview(|wv| {
                        let webview = wv.inner();

                        // Disable tracking/fingerprinting features
                        if let Some(settings) = webview.settings() {
                            settings.set_enable_hyperlink_auditing(false);
                            settings.set_allow_modal_dialogs(false);
                        }

                        // Block third-party cookies
                        if let Some(ctx) = webview.context() {
                            if let Some(cookie_mgr) = ctx.cookie_manager() {
                                cookie_mgr.set_accept_policy(
                                    webkit2gtk::CookieAcceptPolicy::NoThirdParty,
                                );
                            }
                        }
                    })
                    .map_err(|e| eprintln!("with_webview error: {e:?}"))
                    .ok();
            }

            // Suppress unused-variable warning on non-Linux.
            #[cfg(not(target_os = "linux"))]
            {
                let _ = &window;
                eprintln!("Ghost: WebKit privacy settings only available on Linux");
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error building ghost")
        .run(|_app, event| {
            if let tauri::RunEvent::Exit = event {
                let pid = std::process::id();
                let session_dir = std::env::temp_dir().join(format!("ghost-session-{}", pid));
                std::fs::remove_dir_all(&session_dir).ok();

                #[cfg(target_os = "linux")]
                {
                    if let Ok(home) = std::env::var("HOME") {
                        std::fs::remove_dir_all(format!("{home}/.cache/ghost-session-{pid}")).ok();
                        std::fs::remove_dir_all(format!("{home}/.local/share/ghost-session-{pid}"))
                            .ok();
                    }
                }
            }
        });
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolbar_script_blocks_javascript_scheme() {
        assert!(TOOLBAR_INIT_SCRIPT.contains("/^javascript:/i.test(u)"));
    }

    #[test]
    fn toolbar_script_blocks_data_scheme() {
        assert!(TOOLBAR_INIT_SCRIPT.contains("/^data:/i.test(u)"));
    }

    #[test]
    fn toolbar_script_blocks_vbscript_scheme() {
        assert!(TOOLBAR_INIT_SCRIPT.contains("/^vbscript:/i.test(u)"));
    }

    #[test]
    fn toolbar_script_skips_new_tab_page() {
        assert!(TOOLBAR_INIT_SCRIPT.contains("location.protocol === 'tauri:'"));
    }

    #[test]
    fn toolbar_script_sets_url_bar_as_dom_property() {
        // Regression guard: must not concatenate location.href into innerHTML
        assert!(!TOOLBAR_INIT_SCRIPT.contains("value=\"' + location.href"));
        // And must set it safely via DOM property
        assert!(TOOLBAR_INIT_SCRIPT.contains("urlBar.value = location.href"));
    }

    #[test]
    fn init_script_injects_privacy_script() {
        assert!(PRIVACY_INIT_SCRIPT.contains("__lw_ghost_tracker_blocked"));
    }

    #[test]
    fn http_warning_triggers_on_http() {
        assert!(TOOLBAR_INIT_SCRIPT.contains("location.protocol === 'http:'"));
    }

    #[test]
    fn blocklist_loads_from_json() {
        let list = load_blocklist();
        assert!(
            list.len() >= 10,
            "blocklist should have at least 10 entries"
        );
        assert!(
            list.contains("google-analytics.com"),
            "google-analytics.com must be in blocklist"
        );
    }
}
