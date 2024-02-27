use serde::{Serialize, Deserialize};
use sysinfo::{System};

#[derive(Serialize, Deserialize)]
pub struct Ram {
    memory: Memory,
    swap: Swap
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    total: u64,
    used: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Swap {
    total: u64,
    used: u64
}

#[tauri::command]
pub fn get_ram() -> Ram {
    let mut sys: System = System::new();
    sys.refresh_memory();

    return Ram {
        memory: Memory {
            total: sys.total_memory(),
            used: sys.used_memory()
        },
        swap: Swap {
            total: sys.total_swap(),
            used: sys.used_swap()
        }
    };
}
