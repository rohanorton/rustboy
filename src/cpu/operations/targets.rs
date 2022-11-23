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

pub enum ArithmeticTarget16Bit {
    BC,
    DE,
    HL,
    SP,
}

impl ArithmeticTarget16Bit {
    pub fn value(&self, cpu: &mut Cpu) -> u16 {
        match self {
            Self::BC => cpu.registers.bc(),
            Self::DE => cpu.registers.de(),
            Self::HL => cpu.registers.hl(),
            Self::SP => cpu.registers.sp(),
        }
    }
    pub fn set_value(&self, cpu: &mut Cpu, val: u16) {
        match self {
            Self::BC => cpu.registers.set_bc(val),
            Self::DE => cpu.registers.set_de(val),
            Self::HL => cpu.registers.set_hl(val),
            Self::SP => cpu.registers.set_sp(val),
        };
    }
}

impl fmt::Display for ArithmeticTarget16Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BC => "BC",
                Self::DE => "DE",
                Self::HL => "HL",
                Self::SP => "SP",
            }
        )
    }
}

pub enum LdTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BCAddr,
    DEAddr,
    HLAddr,
    HLIAddr,
    HLDAddr,
    D8,
    A16,
    CAddr,
    A8,
}

impl LdTarget {
    pub fn value(&self, cpu: &mut Cpu) -> u8 {
        match self {
            Self::A => cpu.registers.a(),
            Self::B => cpu.registers.b(),
            Self::C => cpu.registers.c(),
            Self::D => cpu.registers.d(),
            Self::E => cpu.registers.e(),
            Self::H => cpu.registers.h(),
            Self::L => cpu.registers.l(),
            Self::BCAddr => cpu.mmu.get_byte(cpu.registers.bc()),
            Self::DEAddr => cpu.mmu.get_byte(cpu.registers.de()),
            Self::HLAddr => cpu.mmu.get_byte(cpu.registers.hl()),
            Self::HLIAddr => {
                let hl = cpu.registers.hl();
                cpu.registers.incr_hl();
                cpu.mmu.get_byte(hl)
            }
            Self::HLDAddr => {
                let hl = cpu.registers.hl();
                cpu.registers.decr_hl();
                cpu.mmu.get_byte(hl)
            }
            Self::D8 => cpu.read_u8(),
            Self::A16 => {
                let addr = cpu.read_u16();
                cpu.mmu.get_byte(addr)
            }
            Self::CAddr => {
                let c = cpu.registers.c() as u16;
                cpu.mmu.get_byte(c + 0xFF00)
            }
            Self::A8 => {
                let x = cpu.read_u8() as u16;
                cpu.mmu.get_byte(x + 0xFF00)
            }
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
            Self::BCAddr => cpu.mmu.set_byte(cpu.registers.bc(), val),
            Self::DEAddr => cpu.mmu.set_byte(cpu.registers.de(), val),
            Self::HLAddr => cpu.mmu.set_byte(cpu.registers.hl(), val),
            Self::HLIAddr => {
                let hl = cpu.registers.hl();
                cpu.registers.incr_hl();
                cpu.mmu.set_byte(hl, val);
            }
            Self::HLDAddr => {
                let hl = cpu.registers.hl();
                cpu.registers.decr_hl();
                cpu.mmu.set_byte(hl, val);
            }
            Self::D8 => panic!("Illegal Operation. Cannot set value."),
            Self::A16 => {
                let addr = cpu.read_u16();
                cpu.mmu.set_byte(addr, val);
            }
            Self::CAddr => {
                let c = cpu.registers.c() as u16;
                cpu.mmu.set_byte(c + 0xFF00, val);
            }
            Self::A8 => {
                let x = cpu.read_u8() as u16;
                cpu.mmu.set_byte(x + 0xFF00, val);
            }
        };
    }
}

impl fmt::Display for LdTarget {
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
                Self::BCAddr => "(BC)",
                Self::DEAddr => "(DE)",
                Self::HLAddr => "(HL)",
                Self::HLIAddr => "(HL+)",
                Self::HLDAddr => "(HL-)",
                Self::D8 => "d8",
                Self::A16 => "(a16)",
                Self::A8 => "($FF00+a8)",
                Self::CAddr => "($FF00+C)",
            }
        )
    }
}
