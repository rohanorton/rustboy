use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

/// Rotates the contents of register A to the right.
pub struct Rrca;

impl Operation for Rrca {
    fn run(&self, cpu: &mut Cpu) {
        let a = cpu.registers.a();
        let rot_a = a.rotate_right(1);
        cpu.registers.set_a(rot_a);
        cpu.registers.set_cy_flag(a & 1 != 0);
        cpu.registers.set_z_flag(false);
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
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
        cpu.registers.set_a(0x3B);
        cpu.registers.set_cy_flag(false);

        // RRCA
        Rrca.run(&mut cpu);

        // A←9Dh,CY←1,Z←0,H←0,N←0
        assert_eq!(cpu.registers.a(), 0x9D);
        assert!(cpu.registers.cy_flag(), "Carry flag should be set");
        assert!(!cpu.registers.z_flag(), "Zero flag should not be set");
        assert!(!cpu.registers.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.registers.n_flag(), "Subtract flag should not be set");
    }
}
