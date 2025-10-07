#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, WebviewWindow};

fn main() {
  tauri::Builder::default()
    .on_page_load(|window, _| {
      // Runs on every navigation
      let js = r#"
        // Kill right-click menu
        addEventListener('contextmenu', e => e.preventDefault(), true);

        // Block common DevTools keys
        addEventListener('keydown', e => {
          const k = e.key.toUpperCase();
          if (
            k === 'F12' ||
            (e.ctrlKey && e.shiftKey && (k === 'I' || k === 'J' || k === 'C')) || // Ctrl+Shift+I/J/C
            (e.ctrlKey && k === 'U') // Ctrl+U (view source)
          ) {
            e.preventDefault();
            e.stopPropagation();
          }
        }, true);
      "#;
      // Fire-and-forget is fine here
      let _ = window.eval(js);
    })
    .run(tauri::generate_context!())
    .expect("error while running Path Key Desktop");
}
