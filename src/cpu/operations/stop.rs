use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Stop;

impl Operation for Stop {
    fn run(&self, _cpu: &mut Cpu) {
        // TODO:
        panic!("STOP operation not implemented")
    }
}

impl fmt::Display for Stop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STOP 0")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_trait() {
        let op = Stop;
        assert_eq!(format!("{op}"), "STOP 0");
    }
}
