use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

/// Rotates the contents of register A to the right.
pub struct Rrca;

impl Operation for Rrca {
    fn run(&self, cpu: &mut Cpu) {
        let a = cpu.reg.a();
        let rot_a = a.rotate_right(1);
        cpu.reg.set_a(rot_a);
        cpu.reg.set_cy_flag(a & 1 != 0);
        cpu.reg.set_z_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Rrca {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RRCA")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Cpu;
    use super::Operation;
    use super::Rrca;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Rrca;
        assert_eq!(format!("{op}"), "RRCA");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 3Bh and CY = 0,
        cpu.reg.set_a(0x3B);
        cpu.reg.set_cy_flag(false);

        // RRCA
        Rrca.run(&mut cpu);

        // A←9Dh,CY←1,Z←0,H←0,N←0
        assert_eq!(cpu.reg.a(), 0x9D);
        assert!(cpu.reg.cy_flag(), "Carry flag should be set");
        assert!(!cpu.reg.z_flag(), "Zero flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }
}
