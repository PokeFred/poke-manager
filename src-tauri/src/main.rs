#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ram;

use ram::get_ram;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ram])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
