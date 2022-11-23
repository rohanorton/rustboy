use core::fmt;

use crate::cpu::cpu::Cpu;

pub trait Operation: fmt::Display {
    fn execute(&self, cpu: &mut Cpu);
}
