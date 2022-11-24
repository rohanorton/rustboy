use crate::cpu::cpu::Cpu;
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
            Self::C => cpu.registers.cy_flag(),
            Self::NC => !cpu.registers.cy_flag(),
            Self::Z => cpu.registers.z_flag(),
            Self::NZ => !cpu.registers.z_flag(),
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
