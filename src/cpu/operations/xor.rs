use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Xor {
    target: ArithmeticTarget8Bit,
}

impl Xor {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Xor { target }
    }
}

impl Operation for Xor {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let new_value = cpu.reg.a() ^ value;

        cpu.reg.set_z_flag(new_value == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_cy_flag(false);

        // Set result in accumulator
        cpu.reg.set_a(new_value);
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

    #[test]
    fn xors_register_with_accumulator() {
        let mut cpu = empty();
        cpu.reg.set_a(0x09);
        cpu.reg.set_c(0x0e);
        Xor::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.reg.a(), 0x07);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x00);
        cpu.reg.set_c(0x00);
        Xor::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x00);
        cpu.reg.set_c(0x01);
        Xor::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        Xor::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.n_flag());
    }

    #[test]
    fn unsets_carry_flag() {
        let mut cpu = empty();
        Xor::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();
        Xor::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Xor::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "XOR C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = FFh
        cpu.reg.set_a(0xFF);

        // XOR A
        Xor::new(ArithmeticTarget8Bit::A).run(&mut cpu);

        // A←00h,Z←1
        assert_eq!(cpu.reg.a(), 0x00);
        assert!(cpu.reg.z_flag());
        assert!(!cpu.reg.h_flag());
        assert!(!cpu.reg.n_flag());
        assert!(!cpu.reg.cy_flag());
    }
}
