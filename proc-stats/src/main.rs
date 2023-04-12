use serde_json::Map;

mod common;
mod cpu;
mod io;
mod network;
use network::network_interface_stats;
mod memory;
use memory::{mem_info, mem_pressure};
mod sys;

use crate::cpu::{cpu_info, cpu_pressure};
use crate::io::io_pressure;
use crate::sys::uptime;

// /proc/cgroups?
// /proc/crypto? lists available ciphers
// /proc/devices?
// /proc/diskstats?
// /proc/driver/*? e.g. rtc
// /proc/ioports?
// /proc/modules?

fn main() {
    let cpu_info_json = serde_json::to_string(&cpu_info()).unwrap();
    let cpu_pressure_json = serde_json::to_string(&cpu_pressure()).unwrap();
    let io_pressure_json = serde_json::to_string(&io_pressure()).unwrap();
    let mem_info_json = serde_json::to_string(&mem_info()).unwrap();
    let mem_pressure_json = serde_json::to_string(&mem_pressure()).unwrap();
    let network_interface_stats_json = serde_json::to_string(&network_interface_stats()).unwrap();
    let sys_uptime_json = serde_json::to_string(&uptime()).unwrap();

    let mut output = serde_json::Value::Object(Map::new());
    output.as_object_mut().map(|output| {
        output.insert(
            String::from("cpu_info"),
            serde_json::from_str(&cpu_info_json).unwrap(),
        );
        output.insert(
            String::from("cpu_pressure"),
            serde_json::from_str(&cpu_pressure_json).unwrap(),
        );
        output.insert(
            String::from("io_pressure"),
            serde_json::from_str(&io_pressure_json).unwrap(),
        );
        output.insert(
            String::from("mem_info"),
            serde_json::from_str(&mem_info_json).unwrap(),
        );
        output.insert(
            String::from("mem_pressure"),
            serde_json::from_str(&mem_pressure_json).unwrap(),
        );
        output.insert(
            String::from("network_interface_stats"),
            serde_json::from_str(&network_interface_stats_json).unwrap(),
        );
        output.insert(
            String::from("sys_uptime"),
            serde_json::from_str(&sys_uptime_json).unwrap(),
        );
        output
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
