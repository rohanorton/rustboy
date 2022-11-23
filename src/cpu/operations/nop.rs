use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

pub struct Nop;

impl Operation for Nop {
    fn execute(&self, _cpu: &mut Cpu) {}
}

impl fmt::Display for Nop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NOP")
    }
}

#[cfg(test)]
mod test {
    use super::Nop;

    #[test]
    fn display_trait() {
        let op = Nop;
        assert_eq!(format!("{op}"), "NOP");
    }
}
