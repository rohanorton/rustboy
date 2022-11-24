use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Scf;

impl Operation for Scf {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg.set_cy_flag(true);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Scf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SCF")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Cpu;
    use super::Operation;
    use super::Scf;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn sets_carry_flag() {
        let mut cpu = empty();
        cpu.reg.set_cy_flag(false);
        Scf.run(&mut cpu);
        assert!(cpu.reg.cy_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_n_flag(true);
        Scf.run(&mut cpu);
        assert!(!cpu.reg.n_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();
        cpu.reg.set_h_flag(true);
        Scf.run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Scf;
        assert_eq!(format!("{op}"), "SCF");
    }
}
