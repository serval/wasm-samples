use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs, path::Path, str::FromStr};

#[derive(Serialize, Deserialize)]
pub enum SysResource {
    Cpu,
    Memory,
    Io,
}
impl fmt::Display for SysResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SysResource::Cpu => write!(f, "cpu"),
            SysResource::Memory => write!(f, "memory"),
            SysResource::Io => write!(f, "io"),
        }
    }
}
impl SysResource {
    pub fn pressure_file(&self) -> &Path {
        match self {
            SysResource::Cpu => Path::new("/proc/pressure/cpu"),
            SysResource::Memory => Path::new("/proc/pressure/memory"),
            SysResource::Io => Path::new("/proc/pressure/io"),
        }
    }
}
#[derive(Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum SysPressureCategory {
    Some,
    Full,
}
#[derive(Debug)]
pub struct SysPressureParseError;

impl FromStr for SysPressureCategory {
    type Err = SysPressureParseError;
    fn from_str(input: &str) -> Result<SysPressureCategory, SysPressureParseError> {
        match input {
            "some" => Ok(SysPressureCategory::Some),
            "full" => Ok(SysPressureCategory::Full),
            _ => Err(SysPressureParseError),
        }
    }
}
impl fmt::Display for SysPressureCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SysPressureCategory::Full => write!(f, "full"),
            SysPressureCategory::Some => write!(f, "some"),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct SysPressureData {
    avg10: f64,
    avg60: f64,
    avg300: f64,
    total: u64,
}
#[derive(Serialize, Deserialize)]
pub struct SysPressure {
    resource: SysResource,
    pressure: HashMap<SysPressureCategory, SysPressureData>,
}

impl SysPressure {
    pub fn try_from(resource: SysResource) -> Result<Self> {
        //let mut path = PathBuf::from("/proc/pressure/");
        //path.push(resource.to_string());
        let pressure: HashMap<SysPressureCategory, SysPressureData> =
            fs::read_to_string(resource.pressure_file())
                .expect("Failed to read {path}")
                .trim()
                .split('\n')
                .filter_map(|line| {
                    // Example line:
                    // some avg10=0.04 avg60=0.08 avg300=0.12 total=10739245730
                    let re = Regex::new(
                        r"(?x)
                        (some|full)+\s                      # $1 Should map to a SysPressureCategory
                        avg10=([0-9]*\.[0-9]+|[0-9]+)\s     # $2 a f64 for the 10 second average
                        avg60=([0-9]*\.[0-9]+|[0-9]+)\s     # $3 a f64 for the 60 second average
                        avg300=([0-9]*\.[0-9]+|[0-9]+)\s    # $4 a f64 for the 300 second average
                        total=([0-9]+)                      # $5 a u64 for the total
                        ",
                    )
                    .unwrap();
                    let cap = re.captures(line);
                    match cap {
                        Some(c) => Some({
                            let cat =
                                SysPressureCategory::from_str(c.get(1).map_or("", |m| m.as_str()))
                                    .expect("Failed to read pressure category");
                            let data = SysPressureData {
                                avg10: c
                                    .get(2)
                                    .map_or(0.00, |m| m.as_str().parse::<f64>().unwrap()),
                                avg60: c
                                    .get(3)
                                    .map_or(0.00, |m| m.as_str().parse::<f64>().unwrap()),
                                avg300: c
                                    .get(4)
                                    .map_or(0.00, |m| m.as_str().parse::<f64>().unwrap()),
                                total: c
                                    .get(5)
                                    .map_or(0_u64, |m| m.as_str().parse::<u64>().unwrap()),
                            };
                            (cat, data)
                        }),
                        None => todo!(),
                    }
                })
                .collect();
        assert_eq!(pressure.len(), 2);
        Ok(SysPressure { resource, pressure })
    }
}
