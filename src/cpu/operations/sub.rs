use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Sub {
    target: ArithmeticTarget8Bit,
}

impl Sub {
    pub fn new(target: ArithmeticTarget8Bit) -> Self {
        Sub { target }
    }
}

impl Operation for Sub {
    fn execute(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let (new_value, did_overflow) = cpu.registers.a().overflowing_sub(value);

        cpu.registers.set_z_flag(new_value == 0);
        cpu.registers.set_n_flag(true);
        cpu.registers.set_cy_flag(did_overflow);
        cpu.registers
            .set_h_flag(((cpu.registers.a() & 0xF).wrapping_sub(value & 0xF)) & 0x10 != 0);

        // Set result in accumulator
        cpu.registers.set_a(new_value);
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

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Operation;
    use super::Sub;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn subtracts_register_from_accumulator() {
        let mut cpu = empty();
        cpu.registers.set_a(0x03);
        cpu.registers.set_c(0x02);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert_eq!(cpu.registers.a(), 0x01);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x04);
        cpu.registers.set_c(0x04);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x03);
        cpu.registers.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn sets_sub_flag() {
        let mut cpu = empty();
        cpu.registers.set_a(0x02);
        cpu.registers.set_c(0x04);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(cpu.registers.n_flag());
    }

    #[test]
    fn sets_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0x01);
        cpu.registers.set_c(0x44);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_carry_flag_when_no_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0xFE);
        cpu.registers.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0x10);
        cpu.registers.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(cpu.registers.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0x1E);
        cpu.registers.set_c(0x01);
        Sub::new(ArithmeticTarget8Bit::C).execute(&mut cpu);
        assert!(!cpu.registers.h_flag());
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
        cpu.registers.set_a(0x3E);
        cpu.registers.set_e(0x3E);

        // SUB A,E
        Sub::new(ArithmeticTarget8Bit::E).execute(&mut cpu);

        // A←00h,Z←1,H←0,N←1 CY←0
        assert_eq!(cpu.registers.a(), 0);
        assert!(cpu.registers.z_flag());
        assert!(!cpu.registers.h_flag());
        assert!(cpu.registers.n_flag());
        assert!(!cpu.registers.cy_flag());
    }
}
