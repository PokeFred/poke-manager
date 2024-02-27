use serde::{Serialize, Deserialize};
use sysinfo::Disks;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemDisk {
    name: String,
    mount_point: String,
    file_system: String,
    available_space: u64,
    total_space: u64,
    is_removable: bool,
    kind: String
}

#[tauri::command]
pub fn get_disks() -> Vec<SystemDisk> {
    let mut disks: Vec<SystemDisk> = vec![];

    for disk in &Disks::new_with_refreshed_list() {
        let element: SystemDisk = SystemDisk {
            name: disk.name().to_string_lossy().to_string(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            available_space: disk.available_space(),
            total_space: disk.total_space(),
            is_removable: disk.is_removable(),
            kind: disk.kind().to_string()
        };

        disks.push(element);
    }

    return disks;
}
