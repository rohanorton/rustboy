use crate::cpu::Cpu;
use std::fmt;

pub enum Condition {
    C,
    NC,
    Z,
    NZ,
}

impl Condition {
    pub fn check(&self, cpu: &mut Cpu) -> bool {
        match self {
            Self::C => cpu.reg.cy_flag(),
            Self::NC => !cpu.reg.cy_flag(),
            Self::Z => cpu.reg.z_flag(),
            Self::NZ => !cpu.reg.z_flag(),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::C => "C",
                Self::NC => "NC",
                Self::Z => "Z",
                Self::NZ => "NZ",
            }
        )
    }
}
