#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ram;

use ram::get_ram;

fn main() {
    /*
    println!("System name:             {:?}", sysinfo::System::name());
    println!("System kernel version:   {:?}", sysinfo::System::kernel_version());
    println!("System OS version:       {:?}", sysinfo::System::os_version());
    println!("System host name:        {:?}", sysinfo::System::host_name());

    let disks = sysinfo::Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{:?}: {:?}", disk.name(), disk);
    }
    */

    /*
    let disks = sysinfo::Disks::new_with_refreshed_list();
    for disk in disks.list() {
        println!("{:?}: {:?}, {:?}, {:?}", disk.name(), disk.kind(), disk.available_space(), disk.total_space());
    }

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
        .invoke_handler(tauri::generate_handler![get_ram])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
