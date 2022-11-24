use core::fmt;

use crate::cpu::cpu::Cpu;

pub trait Operation: fmt::Display {
    fn run(&self, cpu: &mut Cpu);
}
