use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

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
    use crate::cpu::cpu::Cpu;
    use crate::memory::void::Void;

    use super::Di;
    use super::Operation;

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
