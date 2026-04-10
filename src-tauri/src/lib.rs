// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    Manager,
    tray::TrayIconBuilder,
    menu::{Menu, MenuItem},
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;
            let _tray = TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "show" => {
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
                "hide" => {
                let window = app.get_webview_window("main").unwrap();
                window.hide().unwrap();
            }
                "quit" => app.exit(0),
                _ => {}
            })
            .build(app)?;
            let window = app.get_webview_window("main").unwrap();
            window.hide().unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
