use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use crate::byte::lower_nibble;

// Decimal adjust register A.
// This instruction adjusts register A so that the correct representation
// of Binary Coded Decimal (BCD) is obtained.
pub struct Daa {
    cycles: u8,
}

impl Daa {
    pub fn new(cycles: u8) -> Self {
        Daa { cycles }
    }
}

impl Operation for Daa {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        let a = cpu.registers.a();
        let is_addition = !cpu.registers.n_flag();
        let has_half_carried = cpu.registers.h_flag();
        let has_carried = cpu.registers.cy_flag();

        let mut adjustment = 0;
        if has_half_carried {
            adjustment |= 0x06;
        }
        if is_addition && lower_nibble(a) > 9 {
            adjustment |= 0x06;
        }
        if has_carried {
            adjustment |= 0x60;
        }
        if is_addition && a > 0x99 {
            adjustment |= 0x60;
        }

        let bcd_a = if is_addition {
            a.wrapping_add(adjustment)
        } else {
            a.wrapping_sub(adjustment)
        };

        cpu.registers.set_a(bcd_a);
        cpu.registers.set_cy_flag(adjustment >= 0x60);
        cpu.registers.set_z_flag(bcd_a == 0);
        cpu.registers.set_h_flag(false);

        self.cycles
    }
}

impl fmt::Display for Daa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DAA")
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::operations::add::Add;
    use crate::cpu::operations::sub::Sub;
    use crate::cpu::operations::targets::ArithmeticTarget8Bit;
    use crate::memory::void::Void;

    use super::Cpu;
    use super::Daa;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();
        let op = Daa::new(CYCLE_COUNT);
        let res = op.execute(&mut cpu);
        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn sets_zero_flag_if_a_reg_eq_0_at_start() {
        let mut cpu = empty();
        cpu.registers.set_f(0);
        cpu.registers.set_a(0);
        let op = Daa::new(CYCLE_COUNT);
        op.execute(&mut cpu);
        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn sets_zero_flag_if_a_reg_eq_0_at_when_converted() {
        let mut cpu = empty();
        cpu.registers.set_f(0);
        cpu.registers.set_a(0x9A);
        let op = Daa::new(CYCLE_COUNT);
        op.execute(&mut cpu);
        assert_eq!(cpu.registers.a(), 0);
        assert!(cpu.registers.z_flag());
    }

    #[test]
    fn unsets_zero_flag_if_ne_0() {
        let mut cpu = empty();
        cpu.registers.set_a(0x04);
        let op = Daa::new(CYCLE_COUNT);
        op.execute(&mut cpu);
        assert!(!cpu.registers.z_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();
        cpu.registers.set_h_flag(true);
        let op = Daa::new(CYCLE_COUNT);
        op.execute(&mut cpu);
        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Daa::new(CYCLE_COUNT);
        assert_eq!(format!("{op}"), "DAA");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        let op = Daa::new(CYCLE_COUNT);

        // When A = 45h and B = 38h
        cpu.registers.set_a(0x45);
        cpu.registers.set_b(0x38);

        // ADD A,B
        let add_op = Add::new(ArithmeticTarget8Bit::B, CYCLE_COUNT);
        add_op.execute(&mut cpu);
        // A←7Dh,N←0
        assert_eq!(cpu.registers.a(), 0x7D);

        // DAA
        op.execute(&mut cpu);
        // A←7Dh+06h(83h),CY←0
        assert_eq!(cpu.registers.a(), 0x83);
        assert!(!cpu.registers.cy_flag());

        // SUB A,B
        let sub_op = Sub::new(ArithmeticTarget8Bit::B, CYCLE_COUNT);
        sub_op.execute(&mut cpu);
        // A←83h–38h(4Bh),N←1
        assert_eq!(cpu.registers.a(), 0x4B);

        // DAA
        op.execute(&mut cpu);
        // A←4Bh+FAh(45h)
        assert_eq!(cpu.registers.a(), 0x45);
    }
}
