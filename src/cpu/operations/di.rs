use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

/// Disable Interrupt
pub struct Di;

impl Operation for Di {
    fn run(&self, cpu: &mut Cpu) {
        cpu.ime = false;
    }
}

impl fmt::Display for Di {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DI")
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
        let op = Di;
        assert_eq!(format!("{op}"), "DI");
    }

    #[test]
    fn unsets_interrupt_master_enabled_flag() {
        let mut cpu = empty();
        cpu.ime = true;
        Di.run(&mut cpu);
        assert!(!cpu.ime, "IME should not be set");
    }
}
