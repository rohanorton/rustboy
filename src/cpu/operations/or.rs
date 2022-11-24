use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Or {
    target: ArithmeticTarget8Bit,
}

impl Or {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Or { target }
    }
}

impl Operation for Or {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let new_value = cpu.registers.a() | value;

        cpu.registers.set_z_flag(new_value == 0);
        cpu.registers.set_n_flag(false);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_cy_flag(false);

        // Set result in accumulator
        cpu.registers.set_a(new_value);
    }
}

impl fmt::Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OR {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;
    use super::Or;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn ors_register_with_accumulator() {
        let mut cpu = empty();
        cpu.registers.set_a(0x05);
        cpu.registers.set_c(0x03);
        Or::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.registers.a(), 0x07);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x00);
        Or::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x01);
        Or::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        Or::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn unsets_carry_flag() {
        let mut cpu = empty();
        Or::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();
        Or::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Or::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "OR C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 5Ah
        cpu.registers.set_a(0x5A);

        // OR A
        Or::new(ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←5Ah,Z←0
        assert_eq!(cpu.registers.a(), 0x5A);
        assert!(!cpu.registers.z_flag());
        assert!(!cpu.registers.h_flag());
        assert!(!cpu.registers.n_flag());
        assert!(!cpu.registers.cy_flag());
    }
}
