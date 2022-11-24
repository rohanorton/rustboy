use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

// Shifts the contents of operand to the right.
pub struct Sra {
    operand: ArithmeticTarget8Bit,
}

impl Sra {
    pub fn new(operand: ArithmeticTarget8Bit) -> Self {
        Sra { operand }
    }
}

impl Operation for Sra {
    fn run(&self, cpu: &mut Cpu) {
        // Shifts the contents of operand m to the right. That is, the contents
        // of bit 7 are copied to bit 6 and the previous contents of bit 6 (the
        // contents before the copy operation) are copied to bit 5. The same
        // operation is repeated in sequence for the rest of the operand.
        // The contents of bit 0 are copied to CY, and the content of bit 7 is unchanged.
        let x = self.operand.value(cpu);
        let shift_x = (x & 128) | (x >> 1);
        self.operand.set_value(cpu, shift_x);
        cpu.registers.set_cy_flag(x & 1 != 0);
        cpu.registers.set_z_flag(shift_x == 0);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
    }
}

impl fmt::Display for Sra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SRA {}", self.operand)
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
    use super::Sra;

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
        let op = Sra::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "SRA C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 8Ah, and CY = 0,
        cpu.registers.set_a(0x8A);
        cpu.registers.set_cy_flag(false);

        // SRA A
        Sra::new(ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←C5h,CY←0,Z←0,H←0,N←0
        assert_eq!(cpu.registers.a(), 0xC5);
        assert!(!cpu.registers.cy_flag(), "Carry flag should not be set");
        assert!(!cpu.registers.z_flag(), "Zero flag should not be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // When (HL) = 01h, and CY = 0,
        cpu.mmu.set_byte(cpu.registers.hl(), 0x01);
        cpu.registers.set_cy_flag(false);

        // SRA (HL)
        Sra::new(ArithmeticTarget8Bit::HLAddr).run(&mut cpu);

        //(HL)←00h,CY←1,Z←1,H←0,N←0
        assert_eq!(cpu.mmu.get_byte(cpu.registers.hl()), 0x00);
        assert!(cpu.registers.z_flag(), "Zero flag should be set");
        assert!(cpu.registers.cy_flag(), "Carry flag should be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }
}
