// Copyright 2025 Anthony Lenk (owner, Path Key), Nicholas Chiaravalle (author)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use tauri::{Listener, Manager, Url, Webview};

// ---- Commands --------------------------------------------------------------

#[tauri::command]
fn window_action(webview: Webview, action: String) {
  let window = webview.window();
  match action.as_str() {
    "min" => { let _ = window.minimize(); }
    "max" => {
      if window.is_maximized().unwrap_or(false) { let _ = window.unmaximize(); }
      else { let _ = window.maximize(); }
    }
    "close" => { let _ = window.close(); }
    _ => {}
  }
}

#[tauri::command]
fn nav_action(webview: Webview, action: String) {
  let _ = webview.eval("document.getElementById('__pk_spinner')?.classList.add('__pk_show')");
  let js = match action.as_str() {
    "back" => "history.back()",
    "forward" => "history.forward()",
    "reload" => "location.reload()",
    _ => "void 0",
  };
  let _ = webview.eval(js);
}

// ---- App entry -------------------------------------------------------------

fn main() {
  tauri::Builder::default()
    // Start Plugins
    // .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_updater::Builder::new().build())
    // End Plugins
    .invoke_handler(tauri::generate_handler![window_action, nav_action])
    .setup(|app| {

      // Event path — when the splash finishes
      let app_handle_for_event = app.handle().clone();
      app.handle().listen("splash-finished", move |_evt| {
        if let Some(s) = app_handle_for_event.get_webview_window("splash") {
          let _ = s.close();
        }
        if let Some(m) = app_handle_for_event.get_webview_window("main") {
          let _ = m.navigate(Url::parse("https://mypathkey.com").unwrap());
          let _ = m.show();
          let _ = m.set_focus();
        }
      });

      // Fallback timer — in case the event doesn't fire
      let app_handle_for_timer = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(10_500)).await;
        if app_handle_for_timer.get_webview_window("splash").is_some() {
          if let Some(s) = app_handle_for_timer.get_webview_window("splash") {
            let _ = s.close();
          }
          if let Some(m) = app_handle_for_timer.get_webview_window("main") {
            let _ = m.navigate(Url::parse("https://mypathkey.com").unwrap());
            let _ = m.show();
            let _ = m.set_focus();
          }
        }
      });

      Ok(())
    })
    .on_page_load(|webview, _| {
      // Custom titlebar injection (safe to run on any page)
      let js = r#"
      (function(){
        if(document.getElementById('__pk_titlebar')) return;

        const css = `
          :root{--gold:#f2c66e;--edge:#14924e;--glow:rgba(20,146,78,.45);}
          html,body{margin:0;height:100%;overflow:hidden;color:var(--gold);}
          #__pk_titlebar{
            position:fixed;top:0;left:0;right:0;height:34px;z-index:2147483647;
            display:grid;grid-template-columns:auto 1fr auto;align-items:center;
            padding:2px 6px;background:rgba(12,40,28,.92);
            backdrop-filter:blur(10px) saturate(1.15);
            -webkit-backdrop-filter:blur(10px) saturate(1.15);
            border-bottom:1px solid rgba(255,255,255,.12);
            box-shadow:0 4px 14px rgba(0,0,0,.35),inset 0 1px 0 rgba(255,255,255,.08);
            color:var(--gold);-webkit-app-region:drag;border-top-left-radius:14px;border-top-right-radius:14px;
          }
          #__pk_l,#__pk_r{display:flex;gap:6px;align-items:center}
          #__pk_title{justify-self:center;font-weight:700;letter-spacing:.2px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
          .__pk_btn{
            -webkit-app-region:no-drag;width:28px;height:26px;border-radius:8px;
            display:inline-flex;align-items:center;justify-content:center;
            background:rgba(255,255,255,.06);border:1px solid rgba(255,255,255,.12);
            color:var(--gold);cursor:pointer;transition:background .12s,transform .06s;
            box-shadow:inset 0 1px 0 rgba(255,255,255,.18),inset 0 -1px 0 rgba(0,0,0,.5);
          }
          .__pk_btn:hover{background:rgba(255,255,255,.14)}.__pk_btn:active{transform:translateY(1px)}
          .__pk_btn.__pk_close:hover{background:rgba(255,60,60,.55);color:#fff}
          .__pk_btn svg{width:14px;height:14px;fill:currentColor;filter:drop-shadow(0 1px 0 rgba(0,0,0,.4))}
          #__pk_ring{position:fixed;inset:0;pointer-events:none;z-index:2147483599;border-radius:16px;
            box-shadow:0 0 0 2px var(--edge) inset,0 0 18px 2px var(--glow);
            outline:1px solid rgba(0,0,0,.35);outline-offset:-1px;}
          #__pk_spinner{
            position:fixed;top:34px;left:0;right:0;bottom:0;display:flex;align-items:center;justify-content:center;
            pointer-events:none;opacity:0;transition:opacity .18s ease;z-index:2147483647;
          }
          #__pk_spinner.__pk_show{opacity:1}
          #__pk_spinner .dot{width:10px;height:10px;border-radius:50%;background:var(--gold);
            box-shadow:0 0 12px rgba(242,198,110,.6);animation:__pk_b 0.9s infinite alternate}
          #__pk_spinner .dot:nth-child(2){animation-delay:.15s;margin:0 8px}
          #__pk_spinner .dot:nth-child(3){animation-delay:.30s}
          @keyframes __pk_b{from{transform:translateY(0);opacity:.6}to{transform:translateY(-9px);opacity:1}}
        `;
        const s=document.createElement('style');s.textContent=css;document.head.appendChild(s);

        const bar=document.createElement('div');
        bar.id='__pk_titlebar';bar.setAttribute('data-tauri-drag-region','');
        bar.innerHTML=`
          <div id="__pk_l" data-tauri-drag-region="no-drag">
            <button class="__pk_btn" id="__pk_back" title="Back"><svg viewBox="0 0 24 24"><path d="M15.5 4.5a1 1 0 0 1 0 1.4L10.4 11l5.1 5.1a1 1 0 0 1-1.4 1.4l-6-6a1 1 0 0 1 0-1.4l6-6a1 1 0 0 1 1.4 0z"/></svg></button>
            <button class="__pk_btn" id="__pk_forward" title="Forward"><svg viewBox="0 0 24 24"><path d="M8.5 19.5a1 1 0 0 1 0-1.4L13.6 13 8.5 7.9a1 1 0 1 1 1.4-1.4l6 6a1 1 0 0 1 0 1.4l-6 6a1 1 0 0 1-1.4 0z"/></svg></button>
            <button class="__pk_btn" id="__pk_reload" title="Reload"><svg viewBox="0 0 24 24"><path d="M17.65 6.35A7.95 7.95 0 0 0 12 4a8 8 0 1 0 7.64 5.5h-2.04A6 6 0 1 1 12 6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/></svg></button>
          </div>
          <div id="__pk_title" data-tauri-drag-region="no-drag">Path&nbsp;Key</div>
          <div id="__pk_r" data-tauri-drag-region="no-drag">
            <button class="__pk_btn" id="__pk_min" title="Minimize"><svg viewBox="0 0 24 24"><path d="M5 12h14v1H5z"/></svg></button>
            <button class="__pk_btn" id="__pk_max" title="Maximize/Restore"><svg viewBox="0 0 24 24"><path d="M5 5h14v14H5z"/></svg></button>
            <button class="__pk_btn __pk_close" id="__pk_close" title="Close"><svg viewBox="0 0 24 24"><path d="M6.4 5l5.6 5.6L17.6 5l1.4 1.4L13.4 12l5.6 5.6-1.4 1.4L12 13.4 6.4 19l-1.4-1.4L10.6 12 5 6.4z"/></svg></button>
          </div>
        `;
        document.body.appendChild(bar);
        const ring=document.createElement('div');ring.id='__pk_ring';document.body.appendChild(ring);
        const spinner=document.createElement('div');spinner.id='__pk_spinner';
        spinner.innerHTML='<div class="dot"></div><div class="dot"></div><div class="dot"></div>';document.body.appendChild(spinner);

        const updateTitle=()=>{const el=document.getElementById('__pk_title');if(el)el.textContent=document.title||'Path Key';};
        updateTitle();new MutationObserver(updateTitle).observe(document.querySelector('title')||document.head,{childList:true,subtree:true});

        const hide=()=>spinner.classList.remove('__pk_show');window.addEventListener('load',hide);

        const invoke=(c,p)=>(window.__TAURI__?.core?.invoke||window.__TAURI__?.invoke)?.(c,p);
        document.getElementById('__pk_back').onclick=()=>invoke('nav_action',{action:'back'});
        document.getElementById('__pk_forward').onclick=()=>invoke('nav_action',{action:'forward'});
        document.getElementById('__pk_reload').onclick=()=>invoke('nav_action',{action:'reload'});
        document.getElementById('__pk_min').onclick=()=>invoke('window_action',{action:'min'});
        document.getElementById('__pk_max').onclick=()=>invoke('window_action',{action:'max'});
        document.getElementById('__pk_close').onclick=()=>invoke('window_action',{action:'close'});
      })();
      "#;
      let _ = webview.eval(js);
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri app");
}
