use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Set {
    bit_number: u8,
    operand: ArithmeticTarget8Bit,
}

// Sets to 1 the specified bit in specified register r.
impl Set {
    pub fn new(bit_number: u8, operand: ArithmeticTarget8Bit) -> Self {
        Set {
            bit_number,
            operand,
        }
    }
}

impl Operation for Set {
    fn run(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let mask = 1 << self.bit_number;
        self.operand.set_value(cpu, x | mask);
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SET {},{}", self.bit_number, self.operand)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;
    use super::Set;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Set::new(0, ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "SET 0,C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 80h
        cpu.registers.set_a(0x80);

        // NOTE: This example seems to be wrong in the manual, giving the operation SET 3,A.
        // SET 2,A
        Set::new(2, ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←0x84
        assert_eq!(cpu.registers.a(), 0x84);
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When L = 3Bh
        cpu.registers.set_l(0x3B);

        // SET 7, L
        Set::new(7, ArithmeticTarget8Bit::L).run(&mut cpu);

        // L←0xBB
        assert_eq!(cpu.registers.l(), 0xBB);
    }
}
