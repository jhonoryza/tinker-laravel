#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod php;

use php::{execute_laravel_code, run_artisan_command};

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![execute_laravel_code, run_artisan_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
