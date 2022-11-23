use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Nop {
    cycles: u8,
}

impl Nop {
    pub fn new(cycles: u8) -> Self {
        Nop { cycles }
    }
}

impl Operation for Nop {
    fn execute(&self, _cpu: &mut Cpu) -> u8 {
        self.cycles
    }
}

impl fmt::Display for Nop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NOP")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::void::Void;

    use super::Cpu;
    use super::Nop;
    use super::Operation;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    const CYCLE_COUNT: u8 = 4;

    #[test]
    fn returns_cycle_count() {
        let mut cpu = empty();

        let op = Nop::new(CYCLE_COUNT);

        let res = op.execute(&mut cpu);

        assert_eq!(
            res, CYCLE_COUNT,
            "Returned value should match cycle count passed to constructor"
        );
    }

    #[test]
    fn display_trait() {
        let op = Nop::new(CYCLE_COUNT);
        assert_eq!(format!("{op}"), "NOP");
    }
}
