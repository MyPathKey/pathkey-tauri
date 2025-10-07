#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::window::{ProgressBarState, ProgressBarStatus};

#[tauri::command]
fn set_progress(webview: tauri::Webview, indeterminate: bool) {
  let win = webview.window();
  let _ = if indeterminate {
    win.set_progress_bar(ProgressBarState {
      status: Some(ProgressBarStatus::Indeterminate),
      progress: None,
    })
  } else {
    win.set_progress_bar(ProgressBarState {
      status: Some(ProgressBarStatus::None),
      progress: None,
    })
  };
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![set_progress])
    .on_page_load(|webview, _| {
      let js = r#"
        (function () {
          if (window.__pkToolbarInjected) return;
          window.__pkToolbarInjected = true;

          const css = `
            :root{
              --glass-bg: rgba(12,33,24,.55);            /* deep emerald */
              --glass-grad1: rgba(18,52,38,.65);
              --glass-grad2: rgba(8,24,18,.45);
              --glass-border: rgba(255,214,110,.35);     /* soft gold edge */
              --gold: #f2c66e;
              --gold-weak: rgba(242,198,110,.35);
              --ink: #e6f1ea;                            /* mint-ivory text */
              --ink-weak: rgba(230,241,234,.7);
              --btn-bg: rgba(255,255,255,.08);
              --btn-bg-hover: rgba(255,255,255,.14);
              --btn-bg-active: rgba(255,255,255,.22);
              --btn-border: rgba(255,255,255,.18);
              --pill-bg: rgba(20,48,36,.55);
              --pill-border: rgba(255,255,255,.16);
              --shadow-outer: 0 14px 38px rgba(0,0,0,.45), 0 1px 0 rgba(255,255,255,.06) inset;
              --shadow-inner: inset 0 1px 0 rgba(255,255,255,.18), inset 0 -1px 0 rgba(0,0,0,.35);
              --glow-bottom: 0 2px 0 rgba(255, 215, 130, .35);
            }

            .pk-toolbar, .pk-loader {
              font: 13px/1.2 ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Ubuntu, Cantarell, Noto Sans, sans-serif;
              z-index: 2147483647;
              color: var(--ink);
              text-shadow: 0 1px 0 rgba(0,0,0,.4);
            }

            /* ===== Toolbar (glass emerald + gold accents) ===== */
            .pk-toolbar {
              position: fixed; inset: 0 0 auto 0; height: 46px;
              display: grid;
              grid-template-columns: 1fr auto 1fr;       /* center address pill */
              align-items: center;
              padding: 6px 10px;
              background:
                linear-gradient(180deg, var(--glass-grad1), var(--glass-grad2)),
                var(--glass-bg);
              -webkit-backdrop-filter: blur(12px) saturate(1.25);
                      backdrop-filter: blur(12px) saturate(1.25);
              border-bottom: 1px solid var(--glass-border);
              box-shadow: var(--shadow-outer), var(--glow-bottom);
              user-select: none;
              -webkit-app-region: drag;
            }

            .pk-left, .pk-right { display:flex; gap:8px; align-items:center; -webkit-app-region: no-drag; }
            .pk-left { justify-content: flex-start; }
            .pk-right { justify-content: flex-end; }

            .pk-btn {
              -webkit-app-region: no-drag;
              display:inline-flex; align-items:center; justify-content:center;
              width:34px; height:30px;
              border-radius: 10px;
              background: var(--btn-bg);
              border: 1px solid var(--btn-border);
              box-shadow: var(--shadow-inner);
              color: var(--ink);
              cursor: pointer;
              transition: transform .08s ease, background .12s ease, box-shadow .12s ease;
            }
            .pk-btn:hover { background: var(--btn-bg-hover); }
            .pk-btn:active { background: var(--btn-bg-active); transform: translateY(1px); }
            .pk-btn[disabled]{ opacity:.45; cursor:default; filter:grayscale(.2); }

            .pk-btn svg { width:16px; height:16px; fill: currentColor; filter: drop-shadow(0 1px 0 rgba(0,0,0,.35)); }

            /* Center pill address bar with beveled 3D depth */
            .pk-center {
              display:flex; justify-content:center; pointer-events:none;  /* keep bar draggable except inner input */
            }
            .pk-url {
              pointer-events:auto;
              min-width: 40vw; max-width: 64vw; height: 32px;
              display:flex; align-items:center; gap:10px; padding:0 12px;
              background: var(--pill-bg);
              border-radius: 12px;
              border: 1px solid var(--pill-border);
              box-shadow: inset 0 1px 0 rgba(255,255,255,.22), inset 0 -1px 0 rgba(0,0,0,.45), 0 6px 16px rgba(0,0,0,.25);
              color: var(--ink);
              overflow:hidden; white-space:nowrap; text-overflow:ellipsis;
            }
            .pk-url .dot {
              width:10px; height:10px; border-radius:50%;
              background: radial-gradient(circle at 30% 30%, var(--gold), rgba(242,198,110,.35) 60%, rgba(242,198,110,.1) 100%);
              box-shadow: 0 0 10px var(--gold-weak);
              border: 1px solid rgba(0,0,0,.4);
            }
            .pk-url .text { opacity:.95 }

            /* Push page down so it isn't covered */
            html:not(.pk-has-margin) body { margin-top: 46px !important; }

            /* ===== Loader (glass chip) ===== */
            .pk-loader {
              position: fixed; top: 56px; right: 14px;
              background: linear-gradient(180deg, rgba(18,52,38,.85), rgba(10,28,21,.82));
              -webkit-backdrop-filter: blur(10px) saturate(1.2);
                      backdrop-filter: blur(10px) saturate(1.2);
              color: var(--ink);
              border: 1px solid var(--glass-border);
              border-radius: 12px; padding: 10px 12px;
              display: none; align-items: center; gap: 10px;
              box-shadow: 0 12px 28px rgba(0,0,0,.45);
            }
            .pk-loader.show { display: flex; }
            .pk-spinner {
              width: 16px; height:16px; border-radius: 50%;
              border: 2px solid rgba(255,255,255,.22);
              border-top-color: var(--gold);
              animation: pkspin 1s linear infinite;
              box-shadow: 0 0 10px rgba(242,198,110,.35);
            }
            @keyframes pkspin { to { transform: rotate(360deg) } }

            /* Fallback if backdrop-filter unsupported */
            @supports not ((backdrop-filter: blur(10px)) or (-webkit-backdrop-filter: blur(10px))) {
              .pk-toolbar, .pk-loader { background-color: rgba(12,33,24,.92); }
            }
          `;

          const style = document.createElement('style');
          style.textContent = css;
          document.documentElement.appendChild(style);
          document.documentElement.classList.add('pk-has-margin');

          // ---- Toolbar DOM ----
          const bar = document.createElement('div');
          bar.className = 'pk-toolbar';
          bar.innerHTML = `
            <div class="pk-left">
              <button class="pk-btn" id="pk-back" title="Back" aria-label="Back">
                <svg viewBox="0 0 24 24"><path d="M15.5 4.5a1 1 0 0 1 0 1.4L10.4 11l5.1 5.1a1 1 0 0 1-1.4 1.4l-6-6a1 1 0 0 1 0-1.4l6-6a1 1 0 0 1 1.4 0z"/></svg>
              </button>
              <button class="pk-btn" id="pk-forward" title="Forward" aria-label="Forward">
                <svg viewBox="0 0 24 24"><path d="M8.5 19.5a1 1 0 0 1 0-1.4L13.6 13 8.5 7.9a1 1 0 1 1 1.4-1.4l6 6a1 1 0 0 1 0 1.4l-6 6a1 1 0 0 1-1.4 0z"/></svg>
              </button>
              <button class="pk-btn" id="pk-reload" title="Reload" aria-label="Reload">
                <svg viewBox="0 0 24 24" aria-hidden="true">
                  <path d="M17.65 6.35A7.95 7.95 0 0 0 12 4a8 8 0 1 0 7.64 5.5h-2.04A6 6 0 1 1 12 6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                </svg>
              </button>
            </div>

            <div class="pk-center">
              <div class="pk-url" id="pk-url">
                <span class="dot" aria-hidden="true"></span>
                <span class="text" id="pk-url-text"></span>
              </div>
            </div>

            <div class="pk-right">
              <!-- reserved for future (e.g., settings/help) -->
            </div>
          `;
          document.body.appendChild(bar);

          const backBtn = bar.querySelector('#pk-back');
          const fwdBtn  = bar.querySelector('#pk-forward');
          const relBtn  = bar.querySelector('#pk-reload');
          const urlText = bar.querySelector('#pk-url-text');

          backBtn.addEventListener('click', () => history.back());
          fwdBtn.addEventListener('click', () => history.forward());
          relBtn.addEventListener('click', () => location.reload());

          function updateUrlBox(){ urlText.textContent = location.href; }
          function updateNavButtons(){
            backBtn.disabled = (performance?.navigation?.type === 0 && document.referrer === '') && history.length <= 1;
            fwdBtn.disabled = false; /* cannot reliably detect forward availability cross-origin */
          }

          // ---- Loader ----
          const loader = document.createElement('div');
          loader.className = 'pk-loader';
          loader.innerHTML = `<div class="pk-spinner"></div><div>Loading…</div>`;
          document.body.appendChild(loader);

          async function nativeProgress(on){
            try { await window.__TAURI__.invoke('set_progress', { indeterminate: !!on }); } catch {}
          }
          function flashLoader(){
            loader.classList.add('show');
            nativeProgress(true);
            setTimeout(() => { loader.classList.remove('show'); nativeProgress(false); }, 720);
            updateUrlBox(); updateNavButtons();
          }

          // Trigger on navigations
          window.addEventListener('beforeunload', () => { loader.classList.add('show'); nativeProgress(true); });

          const _ps = history.pushState, _rs = history.replaceState;
          history.pushState = function(){ _ps.apply(this, arguments); flashLoader(); };
          history.replaceState = function(){ _rs.apply(this, arguments); flashLoader(); };
          window.addEventListener('popstate', flashLoader);

          // Initial
          updateUrlBox(); updateNavButtons();
          window.addEventListener('load', () => { loader.classList.remove('show'); nativeProgress(false); });
        })();
      "#;

      let _ = webview.eval(js);

      // Clear native progress if any
      let win = webview.window();
      let _ = win.set_progress_bar(ProgressBarState { status: Some(ProgressBarStatus::None), progress: None });
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri app");
}
