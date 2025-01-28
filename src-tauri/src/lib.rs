mod php;

use php::{execute_laravel_code, run_artisan_command};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            execute_laravel_code,
            run_artisan_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
