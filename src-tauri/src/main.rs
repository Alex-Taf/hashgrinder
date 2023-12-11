// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hashgrinder::lib_dictdecode;
use hashgrinder::lib_hashdecode;
use tauri::{Manager, App, AppHandle};

#[tauri::command(rename_all = "snake_case")]
fn dict(app: AppHandle, file_path: &str, cmp_hash: &str) {
    let _ = lib_dictdecode::decode(app, file_path, cmp_hash);
}

#[tauri::command(rename_all = "snake_case")]
fn hash_dict(app: AppHandle, wordlist_file_path: &str, hashlist_file_path: &str) {
    let _ = lib_hashdecode::decode(app, wordlist_file_path, hashlist_file_path);
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
        .invoke_handler(tauri::generate_handler![dict, hash_dict])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
