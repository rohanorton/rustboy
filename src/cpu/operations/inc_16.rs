use std::fmt;

use super::targets::ArithmeticTarget16Bit;
use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Inc16 {
    target: ArithmeticTarget16Bit,
}

impl Inc16 {
    pub fn new(target: ArithmeticTarget16Bit) -> Self {
        Inc16 { target }
    }
}

impl Operation for Inc16 {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.target.value(cpu);
        let new_value = value.wrapping_add(1);
        self.target.set_value(cpu, new_value);
    }
}

impl fmt::Display for Inc16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INC {}", self.target)
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
    fn increments_register() {
        let mut cpu = empty();
        cpu.reg.set_bc(0x0FFF);
        Inc16::new(ArithmeticTarget16Bit::BC).run(&mut cpu);
        assert_eq!(cpu.reg.bc(), 0x1000);
    }

    #[test]
    fn display_trait() {
        let op = Inc16::new(ArithmeticTarget16Bit::BC);
        assert_eq!(format!("{op}"), "INC BC");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = empty();

        // When DE = 235Fh,
        cpu.reg.set_de(0x235F);

        // INC DE
        Inc16::new(ArithmeticTarget16Bit::DE).run(&mut cpu);

        // DE ← 2360h
        assert_eq!(cpu.reg.de(), 0x2360);
    }
}
