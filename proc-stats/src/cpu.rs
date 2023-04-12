use std::{collections::HashMap, fs};

use crate::common::{SysPressure, SysResource};

pub fn cpu_info() -> HashMap<String, String> {
    fs::read_to_string("/proc/cpuinfo")
        .expect("Failed to read /proc/cpuinfo")
        .trim()
        .split('\n')
        .filter_map(|line| {
            // Example line:
            // vendor_id	: GenuineIntel
            line.split_once(':')
                .map(|(name, value)| (name.trim().to_string(), value.trim().to_string()))
        })
        .collect()
}

pub fn cpu_pressure() -> SysPressure {
    SysPressure::try_from(SysResource::Cpu).unwrap()
}
