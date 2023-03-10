use std::fmt;

use super::targets::ArithmeticTarget8Bit;
use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Inc {
    target: ArithmeticTarget8Bit,
}

impl Inc {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Inc { target }
    }
}

impl Operation for Inc {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let new_value = value.wrapping_add(1);

        cpu.reg.set_z_flag(new_value == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(((value & 0xF) + (1 & 0xF)) & 0x10 != 0);

        self.target.set_value(cpu, new_value);
    }
}

impl fmt::Display for Inc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC {}", self.target)
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
    fn increments_register() {
        let mut cpu = empty();
        cpu.reg.set_c(0x01);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.reg.c(), 0x02);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.reg.set_c(0xFF);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.reg.set_c(0x01);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_c(0x04);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.n_flag());
    }

    #[test]
    fn does_not_change_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.reg.set_cy_flag(false);
        cpu.reg.set_c(0xFF);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn does_not_change_carry_flag_no_overflow() {
        let mut cpu = empty();
        cpu.reg.set_c(0xF1);
        cpu.reg.set_cy_flag(false);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_c(0x0F);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_c(0x01);
        Inc::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Inc::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "INC C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = FFh,
        cpu.reg.set_a(0xFF);

        // INC A
        Inc::new(ArithmeticTarget8Bit::A).run(&mut cpu);

        // A???0,Z???1,H???1,N???0
        assert_eq!(cpu.reg.a(), 0);
        assert!(cpu.reg.z_flag());
        assert!(cpu.reg.h_flag());
        assert!(!cpu.reg.n_flag());
    }
}
