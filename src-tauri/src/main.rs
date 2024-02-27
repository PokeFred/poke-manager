#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ram;
mod disk;
mod system;

use ram::get_ram;
use disk::get_disks;
use system::get_system_information;

fn main() {
    println!("{:?}", get_disks());
    println!("{:?}", get_system_information());

    /*
    println!("");
    let networks = sysinfo::Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!("[{interface_name}]: {network:?}");
    }
    println!("");
    let networks = sysinfo::Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!("[{interface_name}] {network:?}");
    }
    */

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ram, get_disks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
