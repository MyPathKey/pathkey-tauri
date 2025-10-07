#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager};
use tauri::Url;
use tauri::WebviewWindow;
use tauri::*;

#[tauri::command]
fn window_action(webview: tauri::Webview, action: String) {
  let win = webview.window();
  match action.as_str() {
    "min" => { let _ = win.minimize(); }
    "max" => {
      if win.is_maximized().unwrap_or(false) { let _ = win.unmaximize(); }
      else { let _ = win.maximize(); }
    }
    "close" => { let _ = win.close(); }
    _ => {}
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![window_action])
    .setup(|app| {
      let url = Url::parse("about:blank").unwrap(); // or "https://mypathkey.com"
      app.get_webview_window("main").unwrap().navigate(url)?;
      Ok(())
    })
    .on_page_load(|webview, _| {
      let js = r#"
        (function () {
          if (window.__emeraldChrome) return; window.__emeraldChrome = true;

          // ===== Styles (emerald glass + GOLD text/icons) =====
          const css = `
            :root {
              --bg-deep: #081a12;
              --glass: rgba(12,40,28,.92);
              --edge: #14924e;          /* border stroke */
              --glow: rgba(20,146,78,.45);
              --gold: #f2c66e;
            }

            html, body {
              margin: 0; height: 100%; width: 100%;
              background: var(--bg-deep);
              color: var(--gold);
              overflow: hidden;
            }

            /* Full, visible border on ALL SIDES with rounded corners */
            .border-ring {
              position: fixed; inset: 0;
              pointer-events: none;
              border-radius: 18px;
              box-shadow:
                0 0 0 2px var(--edge) inset,
                0 0 22px 3px var(--glow);
              /* Draw a faint outer ring too so edges aren't only at corners */
              outline: 2px solid rgba(0,0,0,.35);
              outline-offset: -2px;
              z-index: 2147483646;
            }

            /* Glass container */
            .frame {
              position: fixed; inset: 0;
              border-radius: 16px;
              background: var(--glass);
              -webkit-backdrop-filter: blur(12px) saturate(1.2);
                      backdrop-filter: blur(12px) saturate(1.2);
              display: flex; flex-direction: column;
              overflow: hidden;
              box-shadow: inset 0 1px 0 rgba(255,255,255,.1);
            }

            /* TRUE draggable titlebar across the full width */
            .titlebar {
              height: 46px;
              display: grid;
              grid-template-columns: auto 1fr auto;
              align-items: center;
              padding: 6px 10px;
              -webkit-app-region: drag;         /* drag anywhere here... */
              border-bottom: 1px solid rgba(255,255,255,.15);
              box-shadow: 0 6px 18px rgba(0,0,0,.35), inset 0 1px 0 rgba(255,255,255,.08);
              color: var(--gold);
              z-index: 2;
            }
            .l, .r { display: flex; gap: 8px; align-items: center; }
            .l, .r, .btn { -webkit-app-region: no-drag !important; } /* ...but NOT on buttons */

            .title {
              justify-self: center;
              font-weight: 700;
              letter-spacing: .35px;
              text-shadow: 0 1px 0 rgba(0,0,0,.5);
            }

            .btn {
              width: 34px; height: 30px;
              display: inline-flex; align-items: center; justify-content: center;
              border-radius: 9px;
              background: rgba(255,255,255,.08);
              border: 1px solid rgba(255,255,255,.12);
              color: var(--gold);
              cursor: pointer;
              transition: background .12s, transform .08s;
              box-shadow: inset 0 1px 0 rgba(255,255,255,.18), inset 0 -1px 0 rgba(0,0,0,.45);
            }
            .btn:hover { background: rgba(255,255,255,.16); }
            .btn:active { transform: translateY(1px); }
            .close:hover { background: rgba(255,60,60,.55); color: #fff; }
            .btn svg { width: 16px; height: 16px; fill: currentColor; filter: drop-shadow(0 1px 0 rgba(0,0,0,.4)); }

            /* Content area (holds the remote site) */
            .content { flex: 1; position: relative; }
            .content iframe {
              position: absolute; inset: 0; width: 100%; height: 100%;
              border: 0; background: transparent;
            }
          `;

          const style = document.createElement('style');
          style.textContent = css;
          document.head.appendChild(style);

          // ===== DOM
          document.body.innerHTML = `
            <div class="frame" id="frame">
              <div class="titlebar">
                <div class="l">
                  <div class="btn" id="nav-back" title="Back"><svg viewBox="0 0 24 24"><path d="M15.5 4.5a1 1 0 0 1 0 1.4L10.4 11l5.1 5.1a1 1 0 0 1-1.4 1.4l-6-6a1 1 0 0 1 0-1.4l6-6a1 1 0 0 1 1.4 0z"/></svg></div>
                  <div class="btn" id="nav-forward" title="Forward"><svg viewBox="0 0 24 24"><path d="M8.5 19.5a1 1 0 0 1 0-1.4L13.6 13 8.5 7.9a1 1 0 1 1 1.4-1.4l6 6a1 1 0 0 1 0 1.4l-6 6a1 1 0 0 1-1.4 0z"/></svg></div>
                  <div class="btn" id="nav-reload" title="Reload"><svg viewBox="0 0 24 24"><path d="M17.65 6.35A7.95 7.95 0 0 0 12 4a8 8 0 1 0 7.64 5.5h-2.04A6 6 0 1 1 12 6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg></div>
                </div>
                <div class="title">Path Key</div>
                <div class="r">
                  <div class="btn" id="win-min" title="Minimize"><svg viewBox="0 0 24 24"><path d="M5 12h14v1H5z"/></svg></div>
                  <div class="btn" id="win-max" title="Maximize/Restore"><svg viewBox="0 0 24 24"><path d="M5 5h14v14H5z"/></svg></div>
                  <div class="btn close" id="win-close" title="Close"><svg viewBox="0 0 24 24"><path d="M6.4 5l5.6 5.6L17.6 5l1.4 1.4L13.4 12l5.6 5.6-1.4 1.4L12 13.4 6.4 19l-1.4-1.4L10.6 12 5 6.4z"/></svg></div>
                </div>
              </div>
              <div class="content">
                <iframe id="site" src="https://mypathkey.com" allow="clipboard-read; clipboard-write; fullscreen; geolocation; microphone; camera"></iframe>
              </div>
            </div>
            <div class="border-ring" aria-hidden="true"></div>
          `;

          // ===== Wiring (explicit; not affected by drag region)
          const site = document.getElementById('site');

          document.getElementById('nav-back').onclick    = () => site.contentWindow?.history.back();
          document.getElementById('nav-forward').onclick = () => site.contentWindow?.history.forward();
          document.getElementById('nav-reload').onclick  = () => site.contentWindow?.location.reload();

          document.getElementById('win-min').onclick   = () => window.__TAURI__.invoke('window_action', { action: 'min' });
          document.getElementById('win-max').onclick   = () => window.__TAURI__.invoke('window_action', { action: 'max' });
          document.getElementById('win-close').onclick = () => window.__TAURI__.invoke('window_action', { action: 'close' });
        })();
      "#;

      let _ = webview.eval(js);
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri app");
}
