use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

// Rotates the contents of operand to the left.
pub struct Rlc {
    operand: ArithmeticTarget8Bit,
}

impl Rlc {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Rlc { operand }
    }
}

impl Operation for Rlc {
    fn run(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let rot_x = x.rotate_left(1);
        self.operand.set_value(cpu, rot_x);
        cpu.reg.set_cy_flag(x >> 7 != 0);
        cpu.reg.set_z_flag(rot_x == 0);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Rlc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RLC {}", self.operand)
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
    use super::Rlc;

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
        let op = Rlc::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "RLC C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When B = 85h and CY = 0,
        cpu.reg.set_b(0x85);
        cpu.reg.set_cy_flag(false);

        // RLC B
        Rlc::new(ArithmeticTarget8Bit::B).run(&mut cpu);

        // B←0Bh,CY←1,Z←0,H←0,N←0
        assert_eq!(cpu.reg.b(), 0x0B);
        assert!(cpu.reg.cy_flag(), "Carry flag should be set");
        assert!(!cpu.reg.z_flag(), "Zero flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = 0, and CY = 0,
        cpu.mmu.set_byte(cpu.reg.hl(), 0);
        cpu.reg.set_cy_flag(false);

        // RLC (HL)
        Rlc::new(ArithmeticTarget8Bit::HLAddr).run(&mut cpu);

        // (HL)←00h,CY←0,Z←1,H←0,N←0
        assert_eq!(cpu.mmu.get_byte(cpu.reg.hl()), 0x00);
        assert!(cpu.reg.z_flag(), "Zero flag should be set");
        assert!(!cpu.reg.cy_flag(), "Carry flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }
}
