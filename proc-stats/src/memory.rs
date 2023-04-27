use std::{collections::HashMap, fs};

use crate::common::{SysPressure, SysResource};

pub fn mem_info() -> HashMap<String, String> {
    fs::read_to_string("/proc/meminfo")
        .expect("Failed to read /proc/meminfo")
        .trim()
        .split('\n')
        .filter_map(|line| {
            // Example line:
            // MemTotal:         985768 kB
            line.split_once(':')
                .map(|(name, value)| (name.to_string(), value.trim().to_string()))
        })
        .collect()
}

pub fn mem_pressure() -> SysPressure {
    SysPressure::try_from(SysResource::Memory).unwrap()
}
