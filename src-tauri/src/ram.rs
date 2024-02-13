use serde::{Serialize, Deserialize};
use sysinfo::{System};

#[derive(Serialize, Deserialize)]
pub struct Ram {
    memory: Memory,
    swap: Swap
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    total: u64,
    used: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Swap {
    total: u64,
    used: u64
}

#[tauri::command]
pub fn ram__get_current_state() -> Ram {
    let mut sys = System::new_all();
    sys.refresh_all();

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

    /*
    let mut sys = System::new_all();
    sys.refresh_all();

    let current_ram_state: RamState = RamState {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap()
    };

    return serde_json::to_string(&current_ram_state).unwrap()
    */
}
