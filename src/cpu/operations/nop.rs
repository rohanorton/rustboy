use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Nop;

impl Operation for Nop {
    fn run(&self, _cpu: &mut Cpu) {}
}

impl fmt::Display for Nop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NOP")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_trait() {
        let op = Nop;
        assert_eq!(format!("{op}"), "NOP");
    }
}
