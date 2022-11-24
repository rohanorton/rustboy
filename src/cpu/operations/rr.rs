use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

// Rotates the contents of operand to the right.
pub struct Rr {
    operand: ArithmeticTarget8Bit,
}

impl Rr {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Rr { operand }
    }
}

impl Operation for Rr {
    fn run(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let carry_bit = cpu.registers.cy_flag() as u8;
        let rot_x = x >> 1 | carry_bit << 7;
        self.operand.set_value(cpu, rot_x);
        cpu.registers.set_cy_flag(x & 1 != 0);
        cpu.registers.set_z_flag(rot_x == 0);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
    }
}

impl fmt::Display for Rr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RR {}", self.operand)
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
    use super::Rr;

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
        let op = Rr::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "RR C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 1h, CY = 0,
        cpu.registers.set_a(0x01);
        cpu.registers.set_cy_flag(false);

        // RR A
        Rr::new(ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←00h,CY←1,Z←1,H←0,N←0
        assert_eq!(cpu.registers.a(), 0x00);
        assert!(cpu.registers.cy_flag(), "Carry flag should be set");
        assert!(cpu.registers.z_flag(), "Zero flag should be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = 8Ah, CY = 0,
        cpu.mmu.set_byte(cpu.registers.hl(), 0x8A);
        cpu.registers.set_cy_flag(false);

        // RR (HL)
        Rr::new(ArithmeticTarget8Bit::HLAddr).run(&mut cpu);

        // (HL)←45h,CY←0,Z←0,H←0,N←0
        assert_eq!(cpu.mmu.get_byte(cpu.registers.hl()), 0x45);
        assert!(!cpu.registers.z_flag(), "Zero flag should not be set");
        assert!(!cpu.registers.cy_flag(), "Carry flag should not be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }
}
