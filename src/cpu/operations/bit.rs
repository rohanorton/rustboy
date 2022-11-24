use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Bit {
    number: u8,
    operand: ArithmeticTarget8Bit,
}

// Copies the complement of the contents of the specified bit in register r to the Z flag of the program status word (PSW).
impl Bit {
    pub fn new(number: u8, operand: ArithmeticTarget8Bit) -> Self {
        Bit { number, operand }
    }
}

impl Operation for Bit {
    fn run(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let bit = x >> self.number & 1 == 0;
        cpu.registers.set_z_flag(bit);
        cpu.registers.set_h_flag(true);
        cpu.registers.set_n_flag(false);
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BIT {},{}", self.number, self.operand)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Bit;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Bit::new(0, ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "BIT 0,C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 80h
        cpu.registers.set_a(0x80);

        // BIT 7, A
        Bit::new(7, ArithmeticTarget8Bit::A).run(&mut cpu);

        // Z←0,H←1,N←0
        assert!(!cpu.registers.z_flag(), "Zero flag should not be set");
        assert!(cpu.registers.h_flag(), "Half-Carry flag should be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When L = EFh
        cpu.registers.set_l(0xEF);

        // BIT 4, L
        Bit::new(4, ArithmeticTarget8Bit::L).run(&mut cpu);

        // Z←1,H←1,N←0
        assert!(cpu.registers.z_flag(), "Zero flag should be set");
        assert!(cpu.registers.h_flag(), "Half-Carry flag should be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }
}
