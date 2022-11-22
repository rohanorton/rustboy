use crate::cpu::cpu::Cpu;
use std::fmt;

pub enum ArithmeticTarget8Bit {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLAddr,
    D8,
}

impl ArithmeticTarget8Bit {
    pub fn value(&self, cpu: &mut Cpu) -> u8 {
        match self {
            Self::A => cpu.registers.a(),
            Self::B => cpu.registers.b(),
            Self::C => cpu.registers.c(),
            Self::D => cpu.registers.d(),
            Self::E => cpu.registers.e(),
            Self::H => cpu.registers.h(),
            Self::L => cpu.registers.l(),
            Self::HLAddr => cpu.mmu.get_byte(cpu.registers.hl()),
            Self::D8 => cpu.read_u8(),
        }
    }
    pub fn set_value(&self, cpu: &mut Cpu, val: u8) {
        match self {
            Self::A => cpu.registers.set_a(val),
            Self::B => cpu.registers.set_b(val),
            Self::C => cpu.registers.set_c(val),
            Self::D => cpu.registers.set_d(val),
            Self::E => cpu.registers.set_e(val),
            Self::H => cpu.registers.set_h(val),
            Self::L => cpu.registers.set_l(val),
            Self::HLAddr => cpu.mmu.set_byte(cpu.registers.hl(), val),
            Self::D8 => panic!("Illegal Operation. Cannot set value."),
        };
    }
}

impl fmt::Display for ArithmeticTarget8Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::A => "A",
                Self::B => "B",
                Self::C => "C",
                Self::D => "D",
                Self::E => "E",
                Self::H => "H",
                Self::L => "L",
                Self::HLAddr => "(HL)",
                Self::D8 => "d8",
            }
        )
    }
}
