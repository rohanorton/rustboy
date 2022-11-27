use std::fmt;

use super::targets::LdTarget;
use crate::cpu::operations::Operation;
use crate::cpu::Cpu;

pub struct Ld {
    dest: LdTarget,
    src: LdTarget,
}

impl Ld {
    pub fn new(dest: LdTarget, src: LdTarget) -> Self {
        Ld { dest, src }
    }
}

impl Operation for Ld {
    fn run(&self, cpu: &mut Cpu) {
        let value = self.src.value(cpu);
        self.dest.set_value(cpu, value);
    }
}

impl fmt::Display for Ld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD {},{}", self.dest, self.src)
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;
    use crate::memory::void::Void;

    use super::*;

    fn empty() -> Cpu {
        Cpu::new(Void)
    }

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn loads_value_from_one_register_into_another() {
        let mut cpu = empty();
        cpu.reg.set_a(0x00);
        cpu.reg.set_b(0x23);
        Ld::new(LdTarget::A, LdTarget::B).run(&mut cpu);
        assert_eq!(cpu.reg.a(), 0x23);
    }

    #[test]
    fn loads_value_from_ram_into_register() {
        let mut cpu = with_ram(vec![0x19]);
        cpu.reg.set_hl(0x00);
        cpu.reg.set_a(0x00);
        Ld::new(LdTarget::A, LdTarget::HLAddr).run(&mut cpu);
        assert_eq!(cpu.reg.a(), 0x19);
    }

    #[test]
    fn loads_value_from_register_into_ram() {
        let mut cpu = with_ram(vec![0x00]);
        cpu.reg.set_hl(0x00);
        cpu.reg.set_a(0x29);
        Ld::new(LdTarget::HLAddr, LdTarget::A).run(&mut cpu);
        assert_eq!(cpu.mmu.get_byte(cpu.reg.hl()), 0x29);
    }

    #[test]
    fn loads_value_from_register_into_ram_incrementing_pointer_for_hli_target() {
        let mut cpu = empty();
        cpu.reg.set_hl(0x00);
        cpu.reg.set_a(0x29);
        Ld::new(LdTarget::HLIAddr, LdTarget::A).run(&mut cpu);
        assert_eq!(cpu.reg.hl(), 0x01);
    }

    #[test]
    fn loads_value_from_register_into_ram_decrementing_pointer_for_hld_target() {
        let mut cpu = empty();
        cpu.reg.set_hl(0x25);
        cpu.reg.set_a(0x29);
        Ld::new(LdTarget::HLDAddr, LdTarget::A).run(&mut cpu);
        assert_eq!(cpu.reg.hl(), 0x24);
    }

    #[test]
    fn display_trait() {
        let op = Ld::new(LdTarget::A, LdTarget::B);
        assert_eq!(format!("{op}"), "LD A,B");
    }
}
