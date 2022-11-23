use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;
use super::targets::LdTarget;

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
    fn execute(&self, cpu: &mut Cpu) {
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

    use super::Cpu;
    use super::Ld;
    use super::LdTarget;
    use super::Operation;

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
        cpu.registers.set_a(0x00);
        cpu.registers.set_b(0x23);
        Ld::new(LdTarget::A, LdTarget::B).execute(&mut cpu);
        assert_eq!(cpu.registers.a(), 0x23);
    }

    #[test]
    fn loads_value_from_ram_into_register() {
        let mut cpu = with_ram(vec![0x19]);
        cpu.registers.set_hl(0x00);
        cpu.registers.set_a(0x00);
        Ld::new(LdTarget::A, LdTarget::HLAddr).execute(&mut cpu);
        assert_eq!(cpu.registers.a(), 0x19);
    }

    #[test]
    fn loads_value_from_register_into_ram() {
        let mut cpu = with_ram(vec![0x00]);
        cpu.registers.set_hl(0x00);
        cpu.registers.set_a(0x29);
        Ld::new(LdTarget::HLAddr, LdTarget::A).execute(&mut cpu);
        assert_eq!(cpu.mmu.get_byte(cpu.registers.hl()), 0x29);
    }

    #[test]
    fn loads_value_from_register_into_ram_incrementing_pointer_for_hli_target() {
        let mut cpu = empty();
        cpu.registers.set_hl(0x00);
        cpu.registers.set_a(0x29);
        Ld::new(LdTarget::HLIAddr, LdTarget::A).execute(&mut cpu);
        assert_eq!(cpu.registers.hl(), 0x01);
    }

    #[test]
    fn loads_value_from_register_into_ram_decrementing_pointer_for_hld_target() {
        let mut cpu = empty();
        cpu.registers.set_hl(0x25);
        cpu.registers.set_a(0x29);
        Ld::new(LdTarget::HLDAddr, LdTarget::A).execute(&mut cpu);
        assert_eq!(cpu.registers.hl(), 0x24);
    }

    #[test]
    fn display_trait() {
        let op = Ld::new(LdTarget::A, LdTarget::B);
        assert_eq!(format!("{op}"), "LD A,B");
    }
}
