use std::fmt;

use super::targets::ArithmeticTarget8Bit;
use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Sub {
    target: ArithmeticTarget8Bit,
}

impl Sub {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Sub { target }
    }
}

impl Operation for Sub {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let (new_value, did_overflow) = cpu.reg.a().overflowing_sub(value);

        cpu.reg.set_z_flag(new_value == 0);
        cpu.reg.set_n_flag(true);
        cpu.reg.set_cy_flag(did_overflow);
        cpu.reg
            .set_h_flag(((cpu.reg.a() & 0xF).wrapping_sub(value & 0xF)) & 0x10 != 0);

        // Set result in accumulator
        cpu.reg.set_a(new_value);
    }
}

impl fmt::Display for Sub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SUB {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::*;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn subtracts_register_from_accumulator() {
        let mut cpu = empty();
        cpu.reg.set_a(0x03);
        cpu.reg.set_c(0x02);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.reg.a(), 0x01);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x04);
        cpu.reg.set_c(0x04);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x03);
        cpu.reg.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.z_flag());
    }

    #[test]
    fn sets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_a(0x02);
        cpu.reg.set_c(0x04);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.n_flag());
    }

    #[test]
    fn sets_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x01);
        cpu.reg.set_c(0x44);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.cy_flag());
    }

    #[test]
    fn unsets_carry_flag_when_no_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0xFE);
        cpu.reg.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x10);
        cpu.reg.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x1E);
        cpu.reg.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Sub::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "SUB C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 3Eh, E = 3Eh
        cpu.reg.set_a(0x3E);
        cpu.reg.set_e(0x3E);

        // SUB A,E
        Sub::new(ArithmeticTarget8Bit::E).run(&mut cpu);

        // A←00h,Z←1,H←0,N←1 CY←0
        assert_eq!(cpu.reg.a(), 0);
        assert!(cpu.reg.z_flag());
        assert!(!cpu.reg.h_flag());
        assert!(cpu.reg.n_flag());
        assert!(!cpu.reg.cy_flag());
    }
}
