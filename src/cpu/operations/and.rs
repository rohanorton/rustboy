use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct And {
    target: ArithmeticTarget8Bit,
}

impl And {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        And { target }
    }
}

impl Operation for And {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let new_value = cpu.reg.a() & value;

        cpu.reg.set_z_flag(new_value == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(true);
        cpu.reg.set_cy_flag(false);

        // Set result in accumulator
        cpu.reg.set_a(new_value);
    }
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AND {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::And;
    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn ands_register_with_accumulator() {
        let mut cpu = empty();
        cpu.reg.set_a(0x11);
        cpu.reg.set_c(0x01);
        And::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.reg.a(), 0x01);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x00);
        cpu.reg.set_c(0x00);
        And::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x01);
        cpu.reg.set_c(0x01);
        And::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_a(0x02);
        cpu.reg.set_c(0x04);
        And::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.n_flag());
    }

    #[test]
    fn unsets_carry_flag() {
        let mut cpu = empty();
        cpu.reg.set_a(0xFE);
        cpu.reg.set_c(0x01);
        And::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag() {
        let mut cpu = empty();
        cpu.reg.set_a(0x0F);
        cpu.reg.set_c(0x01);
        And::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = And::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "AND C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 5Ah, L = 3Fh
        cpu.reg.set_a(0x5A);
        cpu.reg.set_l(0x3F);

        // AND L
        And::new(ArithmeticTarget8Bit::L).run(&mut cpu);

        // A←1Ah,Z←0,H←1,N←0 CY←0
        assert_eq!(cpu.reg.a(), 0x1A);
        assert!(!cpu.reg.z_flag());
        assert!(cpu.reg.h_flag());
        assert!(!cpu.reg.n_flag());
        assert!(!cpu.reg.cy_flag());
    }
}
