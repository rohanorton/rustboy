use std::fmt;

use super::super::cpu::Cpu;
use super::operation::Operation;

// Call
pub struct Call;

impl Operation for Call {
    fn run(&self, cpu: &mut Cpu) {
        // In memory, push the program counter PC value corresponding to the address
        // following the CALL instruction to the 2 bytes following the byte specified
        // by the current stack pointer SP. Then load the 16-bit immediate operand
        // a16 into PC.
        //
        // The subroutine is placed after the location specified by the new PC value.
        // When the subroutine finishes, control is returned to the source program
        // using a return instruction and by popping the starting address of the next
        // instruction (which was just pushed) and moving it to the PC.
        //
        // With the push, the current value of SP is decremented by 1, and the
        // higher-order byte of PC is loaded in the memory address specified by the new
        // SP value. The value of SP is then decremented by 1 again, and the lower-order
        // byte of PC is loaded in the memory address specified by that value of SP.
        //
        // The lower-order byte of a16 is placed in byte 2 of the object code, and the
        // higher-order byte is placed in byte 3.
        let a16 = cpu.read_u16();
        let pc = cpu.reg.pc();
        let sp = cpu.reg.sp();
        let [pc_low, pc_high] = pc.to_be_bytes();
        cpu.mmu.set_byte(sp - 2, pc_high);
        cpu.mmu.set_byte(sp - 1, pc_low);
        cpu.reg.set_pc(a16);
        cpu.reg.set_sp(sp - 2);
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CALL a16")
    }
}

#[cfg(test)]
mod test {
    use crate::memory::address_space::AddressSpace;
    use crate::memory::ram::Ram;

    use super::Call;
    use super::Cpu;
    use super::Operation;

    fn with_ram(data: Vec<u8>) -> Cpu {
        let mut ram = Ram::new(0, data.len() as u16);
        for (i, n) in data.iter().enumerate() {
            ram.set_byte(i as u16, *n);
        }
        Cpu::new(ram)
    }

    #[test]
    fn display_trait() {
        let op = Call;
        assert_eq!(format!("{op}"), "CALL a16");
    }

    #[test]
    fn example_from_gameboy_programming_manual() {
        let mut cpu = with_ram(vec![0x00; 0xFFFF]);

        // Examples: When PC = 8000h and SP = FFFEh
        cpu.reg.set_pc(0x8000);

        // Increment PC by 1 (We read a byte in order get op_code)
        cpu.reg.incr_pc();

        cpu.reg.set_sp(0xFFFE);
        cpu.mmu.set_byte(0x8001, 0x34);
        cpu.mmu.set_byte(0x8002, 0x12);

        Call.run(&mut cpu);

        // Jumps to address 1234h
        let pc = cpu.reg.pc();
        assert_eq!(
            pc, 0x1234,
            "PC should store new address 0x1234, but instead is set to {pc:#06x}"
        );
        // (FFFDH) ← 80H
        assert_eq!(
            cpu.mmu.get_byte(0xFFFD),
            0x80,
            "(0xFFFD) should be set to 0x80"
        );
        // (FFFCH) ← 03H
        assert_eq!(
            cpu.mmu.get_byte(0xFFFC),
            0x03,
            "(0xFFFC) should be set to 0x03"
        );
        // SP ← FFFCH
        assert_eq!(
            cpu.reg.sp(),
            0xFFFC,
            "SP should be set to address where return address stored"
        );
    }
}
