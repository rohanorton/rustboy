use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

/// Rotates the contents of register A to the right.
pub struct Rra;

impl Operation for Rra {
    fn run(&self, cpu: &mut Cpu) {
        let a = cpu.reg.a();
        let carry_bit = cpu.reg.cy_flag() as u8;
        let rot_a = a >> 1 | carry_bit << 7;
        cpu.reg.set_a(rot_a);
        cpu.reg.set_cy_flag(a & 1 != 0);
        cpu.reg.set_z_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Rra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRA")
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
    fn display_trait() {
        let op = Rra;
        assert_eq!(format!("{op}"), "RRA");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 81h and CY = 0,
        cpu.reg.set_a(0x81);
        cpu.reg.set_cy_flag(false);

        // RRA
        Rra.run(&mut cpu);

        // A←40h,CY←1,Z←0,H←0,N←0
        assert_eq!(cpu.reg.a(), 0x40);
        assert!(cpu.reg.cy_flag(), "Carry flag should be set");
        assert!(!cpu.reg.z_flag(), "Zero flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }
}
