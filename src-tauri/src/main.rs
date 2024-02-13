#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ram;

use ram::ram__get_current_state;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ram__get_current_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
