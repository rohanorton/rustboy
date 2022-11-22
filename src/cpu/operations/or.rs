use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Or {
    target: ArithmeticTarget8Bit,
    cycles: u8,
}

impl Or {
    pub fn new(target: ArithmeticTarget8Bit, cycles: u8) -> Self {
        Or { target, cycles }
    }
}

impl Operation for Or {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let value = self.target.value(cpu);
        let new_value = cpu.registers.a() | value;

        cpu.registers.set_z_flag(new_value == 0);
        cpu.registers.set_n_flag(false);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_cy_flag(false);

        // Set result in accumulator
        cpu.registers.set_a(new_value);

        self.cycles
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

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn ors_register_with_accumulator() {
        let mut cpu = empty();
        cpu.registers.set_a(0x05);
        cpu.registers.set_c(0x03);

        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert_eq!(cpu.registers.a(), 0x07);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x00);

        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x01);

        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();

        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn unsets_carry_flag() {
        let mut cpu = empty();

        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();

        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Or::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);
        assert_eq!(format!("{op}"), "OR C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 5Ah
        cpu.registers.set_a(0x5A);

        // OR A
        let op = Or::new(ArithmeticTarget8Bit::A, CYCLE_COUNT);

        op.execute(&mut cpu);

        // A←5Ah,Z←0
        assert_eq!(cpu.registers.a(), 0x5A);
        assert!(!cpu.registers.z_flag());
        assert!(!cpu.registers.h_flag());
        assert!(!cpu.registers.n_flag());
        assert!(!cpu.registers.cy_flag());
    }
}
