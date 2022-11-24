use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

/// Rotates the contents of register A to the left.
pub struct Rlca;

impl Operation for Rlca {
    fn run(&self, cpu: &mut Cpu) {
        // That is, the contents of bit 0 are copied to bit 1 and the previous
        // contents of bit 1 (the contents before the copy operation) are copied
        // to bit 2. The same operation is repeated in sequence for the rest of
        // the register. The contents of bit 7 are placed in both CY and bit 0
        // of register A..
        let a = cpu.reg.a();
        let rot_a = a.rotate_left(1);
        cpu.reg.set_a(rot_a);
        cpu.reg.set_cy_flag(a >> 7 != 0);
        cpu.reg.set_z_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Rlca {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RLCA")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Cpu;
    use super::Operation;
    use super::Rlca;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Rlca;
        assert_eq!(format!("{op}"), "RLCA");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // Example: When A = 85h and CY = 0,
        cpu.reg.set_a(0x85);
        cpu.reg.set_cy_flag(false);

        // RLCA
        Rlca.run(&mut cpu);

        // A←0Ah,CY←1,Z←0,H←0,N←0
        // NOTE: The documentation says, A=0x0A, but this doesn't make sense according to the
        // description of the operation. Assuming the example is wrong until proven otherwise.
        assert_eq!(cpu.reg.a(), 0x0B);
        assert!(cpu.reg.cy_flag(), "Carry flag should be set");
        assert!(!cpu.reg.z_flag(), "Zero flag should not be set");
        assert!(!cpu.reg.h_flag(), "Half-Carry flag should not be set");
        assert!(!cpu.reg.n_flag(), "Subtract flag should not be set");
    }
}
