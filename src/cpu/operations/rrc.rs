use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

/// Rotates the contents of operand to the right.
pub struct Rrc {
    operand: ArithmeticTarget8Bit,
}

impl Rrc {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Rrc { operand }
    }
}

impl Operation for Rrc {
    fn run(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let rot_x = x.rotate_right(1);
        self.operand.set_value(cpu, rot_x);
        cpu.registers.set_cy_flag(x & 1 != 0);
        cpu.registers.set_z_flag(rot_x == 0);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
    }
}

impl fmt::Display for Rrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRC {}", self.operand)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;
    use super::Rrc;

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
        let op = Rrc::new(ArithmeticTarget8Bit::D);
        assert_eq!(format!("{op}"), "RRC D");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When C = 1h, CY = 0,
        cpu.registers.set_c(0x01);
        cpu.registers.set_cy_flag(false);

        // RRC C
        Rrc::new(ArithmeticTarget8Bit::C).run(&mut cpu);

        // C←80h,CY←1,Z←0,H←0,N←0
        assert_eq!(cpu.registers.c(), 0x80);
        assert!(cpu.registers.cy_flag(), "Carry flag should be set");
        assert!(!cpu.registers.z_flag(), "Zero flag should not be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = 0, and CY = 0,
        cpu.mmu.set_byte(cpu.registers.hl(), 0);
        cpu.registers.set_cy_flag(false);

        // RRC (HL)
        Rrc::new(ArithmeticTarget8Bit::HLAddr).run(&mut cpu);

        // (HL)←00h,CY←0,Z←1,H←0,N←0
        assert_eq!(cpu.mmu.get_byte(cpu.registers.hl()), 0x00);
        assert!(cpu.registers.z_flag(), "Zero flag should be set");
        assert!(!cpu.registers.cy_flag(), "Carry flag should not be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }
}
