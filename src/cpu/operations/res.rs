use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Res {
    bit_number: u8,
    operand: ArithmeticTarget8Bit,
}

// Resets to 0 the specified bit in the specified register r.
impl Res {
    pub fn new(bit_number: u8, operand: ArithmeticTarget8Bit) -> Self {
        Res {
            bit_number,
            operand,
        }
    }
}

impl Operation for Res {
    fn run(&self, cpu: &mut Cpu) {
        let x = self.operand.value(cpu);
        let mask = 1 << self.bit_number;
        self.operand.set_value(cpu, x & !mask);
    }
}

impl fmt::Display for Res {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RES {},{}", self.bit_number, self.operand)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;
    use super::Res;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Res::new(0, ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "RES 0,C");
    }

    #[test]
    fn example_1_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 80h
        cpu.registers.set_a(0x80);

        // RES 7,A
        Res::new(7, ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←00h
        assert_eq!(cpu.registers.a(), 0x00);
    }

    #[test]
    fn example_2_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When L = 3Bh
        cpu.registers.set_l(0x3B);

        // RES 1, L
        Res::new(1, ArithmeticTarget8Bit::L).run(&mut cpu);

        // L ← 39h
        assert_eq!(cpu.registers.l(), 0x39);
    }
}
