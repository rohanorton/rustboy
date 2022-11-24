use std::fmt;

use super::super::cpu::Cpu;
use super::ei::Ei;
use super::operation::Operation;
use super::ret::Ret;

pub struct Reti;

impl Operation for Reti {
    fn run(&self, cpu: &mut Cpu) {
        Ret.run(cpu);
        Ei.run(cpu);
    }
}

impl fmt::Display for Reti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RETI")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Cpu;
    use super::Operation;
    use super::Reti;

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = Reti;
        assert_eq!(format!("{op}"), "RETI");
    }

    #[test]
    fn returns_to_address_stored() {
        let mut cpu = with_ram(vec![0x00; 0xffff]);

        cpu.reg.set_pc(0x5000);
        cpu.reg.set_sp(0xfffc);
        cpu.mmu.set_byte(0xfffc, 0x03);
        cpu.mmu.set_byte(0xfffd, 0x80);

        Reti.run(&mut cpu);

        assert_eq!(cpu.reg.pc(), 0x8003, "pc should return to previous address");
    }

    #[test]
    fn sets_interrupt_master_enabled_flag() {
        let mut cpu = with_ram(vec![0x00; 0xffff]);
        cpu.reg.set_sp(0xfffc);
        cpu.ime = false;
        Reti.run(&mut cpu);
        assert!(cpu.ime, "IME should be set");
    }
}
