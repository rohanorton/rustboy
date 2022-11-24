use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Add {
    target: ArithmeticTarget8Bit,
}

impl Add {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Add { target }
    }
}

impl Operation for Add {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let (new_value, did_overflow) = cpu.reg.a().overflowing_add(value);

        cpu.reg.set_z_flag(new_value == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_cy_flag(did_overflow);
        cpu.reg
            .set_h_flag(((cpu.reg.a() & 0xF) + (value & 0xF)) & 0x10 != 0);

        // Set result in accumulator
        cpu.reg.set_a(new_value);
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD A,{}", self.target)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Add;
    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn adds_register_to_accumulator() {
        let mut cpu = empty();
        cpu.reg.set_a(0x01);
        cpu.reg.set_c(0x02);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert_eq!(cpu.reg.a(), 0x03);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x00);
        cpu.reg.set_c(0x00);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.reg.set_a(0x00);
        cpu.reg.set_c(0x01);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_a(0x02);
        cpu.reg.set_c(0x04);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.n_flag());
    }

    #[test]
    fn sets_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0xFF);
        cpu.reg.set_c(0x01);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.cy_flag());
    }

    #[test]
    fn unsets_carry_flag_when_no_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0xFE);
        cpu.reg.set_c(0x01);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x0F);
        cpu.reg.set_c(0x01);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(cpu.reg.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.reg.set_a(0x0E);
        cpu.reg.set_c(0x01);
        Add::new(ArithmeticTarget8Bit::C).run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Add::new(ArithmeticTarget8Bit::C);
        assert_eq!(format!("{op}"), "ADD A,C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();
        // When A = 0x3A and B = 0xC6,
        cpu.reg.set_a(0x3A);
        cpu.reg.set_b(0xC6);

        // ADD A, B
        Add::new(ArithmeticTarget8Bit::B).run(&mut cpu);

        // A←0,Z←1,H←1,N←0,CY←1
        assert_eq!(cpu.reg.a(), 0);
        assert!(cpu.reg.z_flag());
        assert!(cpu.reg.h_flag());
        assert!(!cpu.reg.n_flag());
        assert!(cpu.reg.cy_flag());
    }
}
