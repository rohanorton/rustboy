use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Ccf;

impl Operation for Ccf {
    fn run(&self, cpu: &mut Cpu) {
        cpu.reg.set_cy_flag(!cpu.reg.cy_flag());
        cpu.reg.set_h_flag(false);
        cpu.reg.set_n_flag(false);
    }
}

impl fmt::Display for Ccf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CCF")
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
    fn flips_carry_flag() {
        let mut cpu = empty();
        cpu.reg.set_cy_flag(true);
        Ccf.run(&mut cpu);
        assert!(!cpu.reg.cy_flag());
        Ccf.run(&mut cpu);
        assert!(cpu.reg.cy_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_n_flag(true);
        Ccf.run(&mut cpu);
        assert!(!cpu.reg.n_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();
        cpu.reg.set_h_flag(true);
        Ccf.run(&mut cpu);
        assert!(!cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Ccf;
        assert_eq!(format!("{op}"), "CCF");
    }
}
