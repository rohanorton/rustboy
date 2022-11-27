mod clock;
mod operations;
mod registers;
mod run_extended_operation;
mod run_operation;

use crate::memory::address_space::AddressSpace;
use operations::Operation;
use registers::Registers;

use clock::Clock;
use run_extended_operation::run_extended_operation;
use run_operation::run_operation;

pub struct Cpu {
    reg: Registers,
    mmu: Box<dyn AddressSpace>,
    clock: Clock,
    remaining_cycles: u8,
    ime: bool,
    is_halted: bool,
}

impl Cpu {
    pub fn new<Space: AddressSpace + 'static>(mmu: Space) -> Self {
        Cpu {
            reg: Registers::new(),
            mmu: Box::new(mmu),
            clock: Clock::default(),
            remaining_cycles: 0,
            ime: true,
            is_halted: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
            self.clock.sleep_until_next_cycle();
        }
    }

    fn tick(&mut self) {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
            return;
        }
        let op_code = self.read_u8();
        self.execute(op_code);
    }

    fn run_operation(&mut self, op: impl Operation, cycles: u8) {
        let pc = self.reg.pc();
        log::trace!("({pc:#06X}): {op}");
        self.remaining_cycles += cycles;
        op.run(self);
    }

    fn read_u8(&mut self) -> u8 {
        let res = self.mmu.get_byte(self.reg.pc());
        self.reg.incr_pc();
        res
    }

    fn read_u16(&mut self) -> u16 {
        let l = self.read_u8() as u16;
        let h = self.read_u8() as u16;
        (h << 8) | l
    }

    fn execute(&mut self, op_code: u8) {
        run_operation(self, op_code);
    }

    fn execute_extended(&mut self, op_code: u8) {
        run_extended_operation(self, op_code);
    }
}
