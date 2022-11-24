use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

/// Rotates the contents of register A to the left.
pub struct Rla;

impl Operation for Rla {
    fn run(&self, cpu: &mut Cpu) {
        let a = cpu.reg.a();
        let carry_bit = cpu.reg.cy_flag() as u8;
        let rot_a = a << 1 | carry_bit;
        cpu.reg.set_a(rot_a);
        cpu.reg.set_cy_flag(a >> 7 != 0);
        cpu.reg.set_z_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Rla {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RLA")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Cpu;
    use super::Operation;
    use super::Rla;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Rla;
        assert_eq!(format!("{op}"), "RLA");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 95h and CY = 1,
        cpu.reg.set_a(0x95);
        cpu.reg.set_cy_flag(true);

        // RLA
        Rla.run(&mut cpu);

        // A ← 2Bh,C←1,Z←0,H←0,N←0
        assert_eq!(cpu.reg.a(), 0x2B);
        assert!(cpu.reg.cy_flag(), "Carry flag should be set");
        assert!(!cpu.reg.z_flag(), "Zero flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }
}
