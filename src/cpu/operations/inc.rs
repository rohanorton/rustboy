use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::ArithmeticTarget8Bit;

pub struct Inc {
    target: ArithmeticTarget8Bit,
    cycles: u8,
}

impl Inc {
    pub fn new(target: ArithmeticTarget8Bit, cycles: u8) -> Self {
        Inc { target, cycles }
    }
}

impl Operation for Inc {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let value = self.target.value(cpu);
        let new_value = value.wrapping_add(1);

        cpu.registers.set_z_flag(new_value == 0);
        cpu.registers.set_n_flag(false);
        cpu.registers
            .set_h_flag(((value & 0xF) + (1 & 0xF)) & 0x10 != 0);

        self.target.set_value(cpu, new_value);

        self.cycles
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

    use super::ArithmeticTarget8Bit;
    use super::Cpu;
    use super::Inc;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn increments_register() {
        let mut cpu = empty();
        cpu.registers.set_c(0x01);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert_eq!(cpu.registers.c(), 0x02);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.registers.set_c(0xFF);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_c(0x01);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.registers.set_c(0x04);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn does_not_change_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.registers.set_cy_flag(false);

        cpu.registers.set_c(0xFF);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn does_not_change_carry_flag_no_overflow() {
        let mut cpu = empty();
        cpu.registers.set_c(0xF1);
        cpu.registers.set_cy_flag(false);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_c(0x0F);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(cpu.registers.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_c(0x01);

        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Inc::new(ArithmeticTarget8Bit::C, CYCLE_COUNT);
        assert_eq!(format!("{op}"), "INC C");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = FFh,
        cpu.registers.set_a(0xFF);

        // INC A
        let op = Inc::new(ArithmeticTarget8Bit::A, CYCLE_COUNT);

        op.execute(&mut cpu);

        // A←0,Z←1,H←1,N←0
        assert_eq!(cpu.registers.a(), 0);
        assert!(cpu.registers.z_flag());
        assert!(cpu.registers.h_flag());
        assert!(!cpu.registers.n_flag());
    }
}
