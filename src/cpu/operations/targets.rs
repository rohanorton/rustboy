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
            Self::A => cpu.reg.a(),
            Self::B => cpu.reg.b(),
            Self::C => cpu.reg.c(),
            Self::D => cpu.reg.d(),
            Self::E => cpu.reg.e(),
            Self::H => cpu.reg.h(),
            Self::L => cpu.reg.l(),
            Self::HLAddr => cpu.mmu.get_byte(cpu.reg.hl()),
            Self::D8 => cpu.read_u8(),
        }
    }
    pub fn set_value(&self, cpu: &mut Cpu, val: u8) {
        match self {
            Self::A => cpu.reg.set_a(val),
            Self::B => cpu.reg.set_b(val),
            Self::C => cpu.reg.set_c(val),
            Self::D => cpu.reg.set_d(val),
            Self::E => cpu.reg.set_e(val),
            Self::H => cpu.reg.set_h(val),
            Self::L => cpu.reg.set_l(val),
            Self::HLAddr => cpu.mmu.set_byte(cpu.reg.hl(), val),
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
            Self::BC => cpu.reg.bc(),
            Self::DE => cpu.reg.de(),
            Self::HL => cpu.reg.hl(),
            Self::SP => cpu.reg.sp(),
        }
    }
    pub fn set_value(&self, cpu: &mut Cpu, val: u16) {
        match self {
            Self::BC => cpu.reg.set_bc(val),
            Self::DE => cpu.reg.set_de(val),
            Self::HL => cpu.reg.set_hl(val),
            Self::SP => cpu.reg.set_sp(val),
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
            Self::A => cpu.reg.a(),
            Self::B => cpu.reg.b(),
            Self::C => cpu.reg.c(),
            Self::D => cpu.reg.d(),
            Self::E => cpu.reg.e(),
            Self::H => cpu.reg.h(),
            Self::L => cpu.reg.l(),
            Self::BCAddr => cpu.mmu.get_byte(cpu.reg.bc()),
            Self::DEAddr => cpu.mmu.get_byte(cpu.reg.de()),
            Self::HLAddr => cpu.mmu.get_byte(cpu.reg.hl()),
            Self::HLIAddr => {
                let hl = cpu.reg.hl();
                cpu.reg.incr_hl();
                cpu.mmu.get_byte(hl)
            }
            Self::HLDAddr => {
                let hl = cpu.reg.hl();
                cpu.reg.decr_hl();
                cpu.mmu.get_byte(hl)
            }
            Self::D8 => cpu.read_u8(),
            Self::A16 => {
                let addr = cpu.read_u16();
                cpu.mmu.get_byte(addr)
            }
            Self::CAddr => {
                let c = cpu.reg.c() as u16;
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
            Self::A => cpu.reg.set_a(val),
            Self::B => cpu.reg.set_b(val),
            Self::C => cpu.reg.set_c(val),
            Self::D => cpu.reg.set_d(val),
            Self::E => cpu.reg.set_e(val),
            Self::H => cpu.reg.set_h(val),
            Self::L => cpu.reg.set_l(val),
            Self::BCAddr => cpu.mmu.set_byte(cpu.reg.bc(), val),
            Self::DEAddr => cpu.mmu.set_byte(cpu.reg.de(), val),
            Self::HLAddr => cpu.mmu.set_byte(cpu.reg.hl(), val),
            Self::HLIAddr => {
                let hl = cpu.reg.hl();
                cpu.reg.incr_hl();
                cpu.mmu.set_byte(hl, val);
            }
            Self::HLDAddr => {
                let hl = cpu.reg.hl();
                cpu.reg.decr_hl();
                cpu.mmu.set_byte(hl, val);
            }
            Self::D8 => panic!("Illegal Operation. Cannot set value."),
            Self::A16 => {
                let addr = cpu.read_u16();
                cpu.mmu.set_byte(addr, val);
            }
            Self::CAddr => {
                let c = cpu.reg.c() as u16;
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

pub enum Ld16Target {
    BC,
    DE,
    HL,
    SP,
    D16,
    A16,
}

impl Ld16Target {
    pub fn value(&self, cpu: &mut Cpu) -> u16 {
        match self {
            Self::BC => cpu.reg.bc(),
            Self::DE => cpu.reg.de(),
            Self::HL => cpu.reg.hl(),
            Self::SP => cpu.reg.sp(),
            Self::D16 => cpu.read_u16(),
            Self::A16 => panic!("Cannot read u16 from address"),
        }
    }

    pub fn set_value(&self, cpu: &mut Cpu, val: u16) {
        match self {
            Self::BC => cpu.reg.set_bc(val),
            Self::DE => cpu.reg.set_de(val),
            Self::HL => cpu.reg.set_hl(val),
            Self::SP => cpu.reg.set_sp(val),
            Self::D16 => panic!("Cannot write to address"),
            Self::A16 => {
                // Stores the lower byte at address nn specified by the 16-bit
                // immediate operand nn and the upper byte at address nn + 1.
                let nn = cpu.read_u16();
                let upper = (val >> 8) as u8;
                let lower = (val & 0x00FF) as u8;
                cpu.mmu.set_byte(nn, lower);
                cpu.mmu.set_byte(nn + 1, upper);
            }
        };
    }
}

impl fmt::Display for Ld16Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BC => "BC",
                Self::DE => "DE",
                Self::HL => "HL",
                Self::SP => "SP",
                Self::D16 => "d16",
                Self::A16 => "(a16)",
            }
        )
    }
}

pub enum PushPopTarget {
    BC,
    DE,
    HL,
    AF,
}

impl PushPopTarget {
    pub fn value(&self, cpu: &mut Cpu) -> u16 {
        match self {
            Self::BC => cpu.reg.bc(),
            Self::DE => cpu.reg.de(),
            Self::HL => cpu.reg.hl(),
            Self::AF => cpu.reg.af(),
        }
    }
    pub fn set_value(&self, cpu: &mut Cpu, val: u16) {
        match self {
            Self::BC => cpu.reg.set_bc(val),
            Self::DE => cpu.reg.set_de(val),
            Self::HL => cpu.reg.set_hl(val),
            Self::AF => cpu.reg.set_af(val),
        }
    }
}

impl fmt::Display for PushPopTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BC => "BC",
                Self::DE => "DE",
                Self::HL => "HL",
                Self::AF => "AF",
            }
        )
    }
}

#[derive(Clone, Copy)]
pub enum AddressTarget {
    A16,
    HLAddr,
}

impl AddressTarget {
    pub fn value(&self, cpu: &mut Cpu) -> u16 {
        match self {
            Self::A16 => cpu.read_u16(),
            Self::HLAddr => cpu.reg.hl(),
        }
    }
}

impl fmt::Display for AddressTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::A16 => "a16",
                Self::HLAddr => "(HL)",
            }
        )
    }
}
