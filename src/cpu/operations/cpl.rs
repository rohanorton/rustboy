use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Cpl;

impl Operation for Cpl {
    fn execute(&self, cpu: &mut Cpu) {
        // Takes the one’s complement of the contents of register A.
        let a = cpu.registers.a();
        cpu.registers.set_a(!a);

        cpu.registers.set_h_flag(true);
        cpu.registers.set_n_flag(true);
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
        cpu.registers.set_n_flag(true);
        Cpl.execute(&mut cpu);
        assert!(cpu.registers.n_flag());
    }

    #[test]
    fn sets_halfcarry_flag() {
        let mut cpu = empty();
        Cpl.execute(&mut cpu);
        assert!(cpu.registers.h_flag());
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
        cpu.registers.set_a(0x35);

        // CPL
        Cpl.execute(&mut cpu);

        // A ← CAh
        assert_eq!(cpu.registers.a(), 0xCA);
    }
}
