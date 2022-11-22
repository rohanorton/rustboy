use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Ccf {
    cycles: u8,
}

impl Ccf {
    pub fn new(cycles: u8) -> Self {
        Ccf { cycles }
    }
}

impl Operation for Ccf {
    fn execute(&self, cpu: &mut Cpu) -> u8 {
        cpu.registers.set_cy_flag(!cpu.registers.cy_flag());
        cpu.registers.set_h_flag(false);
        cpu.registers.set_n_flag(false);
        self.cycles
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

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        let op = Ccf::new(CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn flips_carry_flag() {
        let mut cpu = empty();
        cpu.registers.set_cy_flag(true);
        let op = Ccf::new(CYCLE_COUNT);
        op.execute(&mut cpu);
        assert!(!cpu.registers.cy_flag());
        op.execute(&mut cpu);
        assert!(cpu.registers.cy_flag());
    }

    #[test]
    fn unsets_sub_flag() {
        let mut cpu = empty();

        cpu.registers.set_n_flag(true);

        let op = Ccf::new(CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.n_flag());
    }

    #[test]
    fn unsets_halfcarry_flag() {
        let mut cpu = empty();

        cpu.registers.set_h_flag(true);

        let op = Ccf::new(CYCLE_COUNT);

        op.execute(&mut cpu);

        assert!(!cpu.registers.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Ccf::new(CYCLE_COUNT);
        assert_eq!(format!("{op}"), "CCF");
    }
}
