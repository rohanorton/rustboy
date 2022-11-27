use std::fmt;

use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct PrefixCB;

impl Operation for PrefixCB {
    fn run(&self, cpu: &mut Cpu) {
        let op_code = cpu.read_u8();
        cpu.execute_extended(op_code);
    }
}

impl fmt::Display for PrefixCB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PREFIX CB")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_trait() {
        let op = PrefixCB;
        assert_eq!(format!("{op}"), "PREFIX CB");
    }
}
