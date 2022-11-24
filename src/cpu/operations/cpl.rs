use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Cpl;

impl Operation for Cpl {
    fn run(&self, cpu: &mut Cpu) {
        // Takes the one’s complement of the contents of register A.
        let a = cpu.reg.a();
        cpu.reg.set_a(!a);

        cpu.reg.set_h_flag(true);
        cpu.reg.set_n_flag(true);
    }
}

impl fmt::Display for Cpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CPL")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Cpl;
    use super::Cpu;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    #[test]
    fn sets_sub_flag() {
        let mut cpu = empty();
        cpu.reg.set_n_flag(true);
        Cpl.run(&mut cpu);
        assert!(cpu.reg.n_flag());
    }

    #[test]
    fn sets_halfcarry_flag() {
        let mut cpu = empty();
        Cpl.run(&mut cpu);
        assert!(cpu.reg.h_flag());
    }

    #[test]
    fn display_trait() {
        let op = Cpl;
        assert_eq!(format!("{op}"), "CPL");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When A = 35h,
        cpu.reg.set_a(0x35);

        // CPL
        Cpl.run(&mut cpu);

        // A ← CAh
        assert_eq!(cpu.reg.a(), 0xCA);
    }
}
