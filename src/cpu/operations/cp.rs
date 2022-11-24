use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Cp {
    target: ArithmeticTarget8Bit,
}

/// Compares the contents of operand s and register A and sets the flag if they are equal.
impl Cp {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Cp { target }
    }
}

impl Operation for Cp {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let (new_value, did_overflow) = cpu.reg.a().overflowing_sub(value);

        cpu.reg.set_z_flag(new_value == 0);
        cpu.reg.set_n_flag(true);
        cpu.reg.set_cy_flag(did_overflow);
        cpu.reg
            .set_h_flag(((cpu.reg.a() & 0xF).wrapping_sub(value & 0xF)) & 0x10 != 0);

        // DON'T set result in accumulator
    }
}

impl fmt::Display for Cp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CP {}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::ArithmeticTarget8Bit;
    use super::Cp;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn does_not_alter_accumulator() {
        let mut cpu = empty();
        cpu.reg.set_a(0x11);
        cpu.reg.set_c(0x0F);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.reg.a(), 0x11);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x04);
        cpu.reg.set_c(0x04);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x03);
        cpu.reg.set_c(0x01);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.z_flag());
    }

    #[test]
    fn sets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_a(0x02);
        cpu.reg.set_c(0x04);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.n_flag());
    }

    #[test]
    fn sets_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x01);
        cpu.reg.set_c(0x44);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.cy_flag());
    }

    #[test]
    fn unsets_carry_flag_when_no_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0xFE);
        cpu.reg.set_c(0x01);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x10);
        cpu.reg.set_c(0x01);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x1E);
        cpu.reg.set_c(0x01);
        Cp::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Cp::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "CP C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 3Ch, B = 2Fh
        cpu.reg.set_a(0x3C);
        cpu.reg.set_b(0x2F);

        // CP B
        Cp::new(ArithmeticTarget8Bit::B).run(&mut cpu);

        // Z←0,H←1,N←1,CY←0
        assert!(!cpu.reg.z_flag());
        assert!(cpu.reg.h_flag());
        assert!(cpu.reg.n_flag());
        assert!(!cpu.reg.cy_flag());
    }
}
