use crate::common::{SysPressure, SysResource};

pub fn io_pressure() -> SysPressure {
    SysPressure::try_from(SysResource::Io).unwrap()
}
