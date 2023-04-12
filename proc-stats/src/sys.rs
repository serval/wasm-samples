use serde::{Deserialize, Serialize};
use std::{
    fs,
    time::{Duration, SystemTime},
};

#[derive(Serialize, Deserialize)]
pub struct SysUptime {
    up_since: SystemTime,
    uptime: Duration,
    seconds_idle: Duration,
}
impl SysUptime {
    pub fn new() -> SysUptime {
        let read_uptime = fs::read_to_string("/proc/uptime")
            .expect("Failed to read {path}")
            .trim()
            .split_whitespace()
            .filter_map(|item| item.parse::<f64>().ok())
            .collect::<Vec<f64>>();
        let uptime = Duration::from_secs_f64(read_uptime[0]);
        let seconds_idle = Duration::from_secs_f64(read_uptime[1]);
        SysUptime {
            up_since: SystemTime::now() - uptime,
            uptime,
            seconds_idle,
        }
    }
}
pub fn uptime() -> SysUptime {
    SysUptime::new()
}
