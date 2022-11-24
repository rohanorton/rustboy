use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

/// Enable Interrupt
pub struct Ei;

impl Operation for Ei {
    fn run(&self, cpu: &mut Cpu) {
        cpu.ime = true;
    }
}

impl fmt::Display for Ei {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EI")
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::cpu::Cpu;
    use crate::memory::void::Void;

    use super::Ei;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn display_trait() {
        let op = Ei;
        assert_eq!(format!("{op}"), "EI");
    }

    #[test]
    fn sets_interrupt_master_enabled_flag() {
        let mut cpu = empty();
        cpu.ime = false;
        Ei.run(&mut cpu);
        assert!(cpu.ime, "IME should be set");
    }
}
