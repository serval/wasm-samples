use crate::common::{SysPressure, SysResource};

pub fn cpu_pressure() -> SysPressure {
    SysPressure::try_from(SysResource::Cpu).unwrap()
}
