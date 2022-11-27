use std::fmt;

use super::targets::ArithmeticTarget8Bit;
use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

// Rotates the contents of operand to the left.
pub struct Swap {
    operand: ArithmeticTarget8Bit,
}

impl Swap {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Swap { operand }
    }
}

impl Operation for Swap {
    fn run(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let swapped = x << 4 | x >> 4;
        self.operand.set_value(cpu, swapped);
        cpu.reg.set_z_flag(swapped == 0);
        cpu.reg.set_cy_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Swap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SWAP {}", self.operand)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;
    use crate::memory::void::Void;

    use super::*;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = Swap::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "SWAP C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 00h
        cpu.reg.set_a(0x00);

        // SWAP A
        Swap::new(ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←00h,Z←1,H←0,N←0,CY←0
        assert_eq!(cpu.reg.a(), 0x00);
        assert!(cpu.reg.z_flag(), "Zero flag should be set");
        assert!(!cpu.reg.cy_flag(), "Carry flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = F0h
        cpu.mmu.set_byte(cpu.reg.hl(), 0xF0);

        // SWAP(HL)
        Swap::new(ArithmeticTarget8Bit::HLAddr).run(&mut cpu);

        // (HL)←0Fh,Z←0,H←0,N←0,CY←0
        assert_eq!(cpu.mmu.get_byte(cpu.reg.hl()), 0x0F);
        assert!(!cpu.reg.z_flag(), "Zero flag should not be set");
        assert!(!cpu.reg.cy_flag(), "Carry flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }
}
