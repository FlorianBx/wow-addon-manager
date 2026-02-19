mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::fetch_manifest,
            commands::get_installed_addons,
            commands::install_addon,
            commands::uninstall_addon,
            commands::get_wow_path,
            commands::set_wow_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
