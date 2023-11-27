// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod dictdecode;

#[tauri::command(rename_all = "snake_case")]
fn start(file_path: &str, cmp_hash: &str) {
    dictdecode::dictdecode::decode(file_path, cmp_hash);
}

fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
              let window = app.get_window("main").unwrap();
              window.open_devtools();
              window.close_devtools();
            }
            Ok(())
          })
        .invoke_handler(tauri::generate_handler![start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
