use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Ccf;

impl Operation for Ccf {
    fn run(&self, cpu: &mut Cpu) {
        cpu.registers.set_cy_flag(!cpu.registers.cy_flag());
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
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

    use super::Ccf;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn flips_carry_flag() {
        let mut cpu = empty();
        cpu.registers.set_cy_flag(true);
        Ccf.run(&mut cpu);
        assert!(!cpu.registers.cy_flag());
        Ccf.run(&mut cpu);
        assert!(cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();
        cpu.registers.set_n_flag(true);
        Ccf.run(&mut cpu);
        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();
        cpu.registers.set_h_flag(true);
        Ccf.run(&mut cpu);
        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Ccf;
        assert_eq!(format!("{op}"), "CCF");
    }
}
