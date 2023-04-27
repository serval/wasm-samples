use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
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

#[derive(Serialize, Deserialize)]
pub enum Virtualization {
    Vm(String),
    Container(String),
    None,
}

#[derive(Serialize, Deserialize)]
pub struct Platform {
    virtualization: Virtualization,
}

fn exists_and_contains(file: &str, value: &str) -> bool {
    let path = Path::new(file);
    match fs::read_to_string(path) {
        Ok(s) => return s.contains(value),
        _ => return false,
    }
}
/// Detects platform characteristics.
pub fn detect_platform() -> Platform {
    let device_tree = fs::read_dir(Path::new("/proc/device-tree"));
    match device_tree {
        Ok(dir) => {
            if Path::new("/proc/device-tree/hypervisor/compatible").exists() {
                let hv_compat = fs::read_to_string("/proc/device-tree/hypervisor/compatible")
                    .expect("Failed to read /proc/device-tree/hypervisor/compatible");
                if hv_compat.contains("linux,kvm") {
                    return Platform {
                        virtualization: Virtualization::Vm(String::from("kvm")),
                    };
                }
                if hv_compat.contains("xen") {
                    return Platform {
                        virtualization: Virtualization::Vm(String::from("xen")),
                    };
                }
                if hv_compat.contains("vmware") {
                    return Platform {
                        virtualization: Virtualization::Vm(String::from("vmware")),
                    };
                }
            } else if Path::new("/proc/device-tree/ibm,partition-name").exists()
                && Path::new("/proc/device-tree/hmc-managed?").exists()
                && !Path::new("/proc/device-treee/chosen/qemu,graphic-width").exists()
            {
                return Platform {
                    virtualization: Virtualization::Vm(String::from("powervm")),
                };
            }
            for file in dir {
                match file {
                    Ok(f) => {
                        if f.path().to_string_lossy().contains("fw-cfg") {
                            return Platform {
                                virtualization: Virtualization::Vm(String::from("qemu")),
                            };
                        }
                    }
                    _ => {}
                }
            }
            if exists_and_contains("/proc/device-tree/compatible", "qemu,pseries") {
                return Platform {
                    virtualization: Virtualization::Vm(String::from("qemu")),
                };
            }
        }
        _ => {}
    }
    if Path::new("/proc/xen").exists() {
        return Platform {
            virtualization: Virtualization::Vm(String::from("xen")),
        };
    }

    // OpenVZ
    if Path::new("/proc/vz").exists() && Path::new("/proc/bc").exists() {
        return Platform {
            virtualization: Virtualization::Container(String::from("openvz")),
        };
    }
    let osrelease = fs::read_to_string("/proc/sys/kernel/osrelease")
        .expect("Failed to read /proc/sys/kernel/osrelease");
    // Microsoft WSL
    // Apparently, this is how to detect WSL:
    // https://github.com/Microsoft/WSL/issues/423#issuecomment-221627364
    if osrelease.contains("Microsoft") && osrelease.contains("WSL") {
        return Platform {
            virtualization: Virtualization::Container(String::from("wsl")),
        };
    }

    // Others (tbd)
    // proot? https://github.com/systemd/systemd/blob/fde55f3a327c4eef3bbdeb199b05e7c385d1f331/src/basic/virt.c#L689
    Platform {
        virtualization: Virtualization::None,
    }
}
