use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Xor {
    target: ArithmeticTarget8Bit,
    cycles: u8,
}

impl Xor {
    pub fn new(target: ArithmeticTarget8Bit, cycles: u8) -> Self {
        Xor { target, cycles }
    }
}

impl Operation for Xor {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let value = self.target.value(cpu);
        let new_value = cpu.registers.a() ^ value;

        cpu.registers.set_z_flag(new_value == 0);
        cpu.registers.set_n_flag(false);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_cy_flag(false);

        // Set result in accumulator
        cpu.registers.set_a(new_value);

        self.cycles
    }
}

impl fmt::Display for Xor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XOR {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;
    use super::Xor;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn xors_register_with_accumulator() {
        let mut cpu = empty();
        cpu.registers.set_a(0x09);
        cpu.registers.set_c(0x0e);

        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert_eq!(cpu.registers.a(), 0x07);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x00);

        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x01);

        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();

        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn unsets_carry_flag() {
        let mut cpu = empty();

        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();

        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Xor::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);
        assert_eq!(format!("{op}"), "XOR C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = FFh
        cpu.registers.set_a(0xFF);

        // XOR A
        let op = Xor::new(ArithmeticTarget8Bit::A, CYCLE_COUNT);

        op.execute(&mut cpu);

        // A←00h,Z←1
        assert_eq!(cpu.registers.a(), 0x00);
        assert!(cpu.registers.z_flag());
        assert!(!cpu.registers.h_flag());
        assert!(!cpu.registers.n_flag());
        assert!(!cpu.registers.cy_flag());
    }
}
