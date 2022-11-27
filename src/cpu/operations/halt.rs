use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

/// Halt
pub struct Halt;

impl Operation for Halt {
    fn run(&self, cpu: &mut Cpu) {
        cpu.is_halted = true;
    }
}

impl fmt::Display for Halt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HALT")
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;
    use crate::memory::void::Void;

    use super::*;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Halt;
        assert_eq!(format!("{op}"), "HALT");
    }

    #[test]
    fn sets_interrupt_master_enabled_flag() {
        let mut cpu = empty();
        cpu.is_halted = false;
        Halt.run(&mut cpu);
        assert!(cpu.is_halted, "Halted should be set");
    }
}
