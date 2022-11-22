use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::Target;

pub struct Add {
    target: Target,
    cycles: u8,
}

impl Add {
    pub fn new(target: Target, cycles: u8) -> Self {
        Add { target, cycles }
    }
}

impl Operation for Add {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let value = self.target.value(cpu);
        let (new_value, did_overflow) = cpu.registers.a().overflowing_add(value);

        cpu.registers.set_z_flag(new_value == 0);
        cpu.registers.set_n_flag(false);
        cpu.registers.set_cy_flag(did_overflow);
        cpu.registers
            .set_h_flag(((cpu.registers.a() & 0xF) + (value & 0xF)) & 0x10 != 0);

        // Set result in accumulator
        cpu.registers.set_a(new_value);

        self.cycles
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
    use super::Cpu;
    use super::Operation;
    use super::Target;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        const CYCLE_COUNT: u8 = 4;

        let op = Add::new(Target::C, CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn adds_register_to_accumulator() {
        let mut cpu = empty();
        cpu.registers.set_a(0x01);
        cpu.registers.set_c(0x02);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert_eq!(cpu.registers.a(), 0x03);
    }

    #[test]
    fn sets_zero_flag_when_result_eq_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x00);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_when_result_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x00);
        cpu.registers.set_c(0x01);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.registers.set_a(0x02);
        cpu.registers.set_c(0x04);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn sets_carry_flag_on_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0xFF);
        cpu.registers.set_c(0x01);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert!(cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_carry_flag_when_no_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0xFE);
        cpu.registers.set_c(0x01);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert!(!cpu.registers.cy_flag());
    }

    #[test]
    fn sets_halfcarry_flag_on_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0x0F);
        cpu.registers.set_c(0x01);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert!(cpu.registers.h_flag());
    }

    #[test]
    fn unsets_halfcarry_flag_when_no_lower_nibble_overflow() {
        let mut cpu = empty();
        cpu.registers.set_a(0x0E);
        cpu.registers.set_c(0x01);

        let op = Add::new(Target::C, 4);

        op.execute(&mut cpu);

        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Add::new(Target::C, 4);
        assert_eq!(format!("{op}"), "ADD A,C");
    }
}
