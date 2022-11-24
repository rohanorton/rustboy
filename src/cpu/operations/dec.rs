use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Dec {
    target: ArithmeticTarget8Bit,
}

impl Dec {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Dec { target }
    }
}

impl Operation for Dec {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let new_value = value.wrapping_sub(1);

        cpu.reg.set_z_flag(new_value == 0);
        cpu.reg.set_n_flag(true);
        cpu.reg
            .set_h_flag(((value & 0xF).wrapping_sub(1 & 0xF)) & 0x10 != 0);

        self.target.set_value(cpu, new_value);
    }
}

impl fmt::Display for Dec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DEC {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Dec;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn decrements_register() {
        let mut cpu = empty();
        cpu.reg.set_c(0x02);
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.reg.c(), 0x01);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.reg.set_c(0x01);
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.reg.set_c(0x04);
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.z_flag());
    }

    #[test]
    fn sets_sub_flag() {
        let mut cpu = empty();
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.n_flag());
    }

    #[test]
    fn does_not_change_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.reg.set_c(0x00);
        cpu.reg.set_cy_flag(false);
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn does_not_change_carry_flag_no_overflow() {
        let mut cpu = empty();
        cpu.reg.set_c(0xF4);
        cpu.reg.set_cy_flag(true);
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_c(0x10);
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_c(0xF1);
        Dec::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Dec::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "DEC C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When L = 01h,
        cpu.reg.set_l(0x01);
        cpu.reg.set_e(0x3E);

        // DEC L
        Dec::new(ArithmeticTarget8Bit::L).run(&mut cpu);

        // L←0,Z←1,H←0,N←1
        assert_eq!(cpu.reg.l(), 0);
        assert!(cpu.reg.z_flag());
        assert!(!cpu.reg.h_flag());
        assert!(cpu.reg.n_flag());
    }
}
