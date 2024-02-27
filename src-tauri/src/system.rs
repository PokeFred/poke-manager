use serde::{Serialize, Deserialize};
use sysinfo::{System};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInformation {
    name: String,
    host_name: String,
    os_version: String,
    kernel_version: String
}

#[tauri::command]
pub fn get_system_information() -> SystemInformation {
    return SystemInformation {
        name: System::name().unwrap(),
        host_name: System::host_name().unwrap(),
        os_version: System::os_version().unwrap(),
        kernel_version: System::kernel_version().unwrap()
    };
}
