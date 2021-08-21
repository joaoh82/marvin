use serde::{Deserialize, Serialize};
use sysinfo::{System as SystemInfo, SystemExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub used_memory: u64,
    pub total_memory: u64,
    pub used_swap: u64,
    pub total_swap: u64,
}

/// read_metric reads the memory metrics from the system
pub fn read_metric(sys : &SystemInfo) -> Result<String, std::io::Error> {
    let memory = Memory {
        used_memory: sys.used_memory(),
        total_memory: sys.total_memory(),
        used_swap: sys.used_swap(),
        total_swap: sys.total_swap(),
    };
    let out = serde_json::to_string(&memory).unwrap();
    Ok(out)
}