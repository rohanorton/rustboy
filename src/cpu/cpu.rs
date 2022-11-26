use super::operations::operation::Operation;
use super::registers::Registers;
use crate::memory::address_space::AddressSpace;

use super::operations::condition::Condition;
use super::operations::targets::{
    AddressTarget, ArithmeticTarget16Bit, ArithmeticTarget8Bit, Ld16Target, LdTarget, PushPopTarget,
};

use super::operations::bit::Bit;
use super::operations::res::Res;
use super::operations::rl::Rl;
use super::operations::rlc::Rlc;
use super::operations::rr::Rr;
use super::operations::rrc::Rrc;
use super::operations::set::Set;
use super::operations::sla::Sla;
use super::operations::sra::Sra;
use super::operations::srl::Srl;
use super::operations::swap::Swap;

use super::operations::adc::Adc;
use super::operations::add::Add;
use super::operations::add_hl::AddHl;
use super::operations::add_sp::AddSp;
use super::operations::and::And;
use super::operations::call::Call;
use super::operations::call_cond::ConditionalCall;
use super::operations::ccf::Ccf;
use super::operations::cp::Cp;
use super::operations::cpl::Cpl;
use super::operations::daa::Daa;
use super::operations::dec::Dec;
use super::operations::dec_16::Dec16;
use super::operations::di::Di;
use super::operations::ei::Ei;
use super::operations::halt::Halt;
use super::operations::inc::Inc;
use super::operations::inc_16::Inc16;
use super::operations::jp::Jp;
use super::operations::jp_cond::ConditionalJp;
use super::operations::jr::Jr;
use super::operations::jr_cond::ConditionalJr;
use super::operations::ld::Ld;
use super::operations::ld_16::Ld16;
use super::operations::ld_hl_sp::LdHlSp;
use super::operations::nop::Nop;
use super::operations::or::Or;
use super::operations::pop::Pop;
use super::operations::prefix_cb::PrefixCB;
use super::operations::push::Push;
use super::operations::ret::Ret;
use super::operations::ret_cond::ConditionalRet;
use super::operations::reti::Reti;
use super::operations::rla::Rla;
use super::operations::rlca::Rlca;
use super::operations::rra::Rra;
use super::operations::rrca::Rrca;
use super::operations::rst::Rst;
use super::operations::sbc::Sbc;
use super::operations::scf::Scf;
use super::operations::stop::Stop;
use super::operations::sub::Sub;
use super::operations::xor::Xor;

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Box<dyn AddressSpace>,
    pub remaining_cycles: u8,
    pub ime: bool,
    pub is_halted: bool,
}

impl Cpu {
    pub fn new<Space: AddressSpace + 'static>(mmu: Space) -> Self {
        Cpu {
            reg: Registers::new(),
            mmu: Box::new(mmu),
            remaining_cycles: 0,
            ime: true,
            is_halted: false,
        }
    }

    pub fn run_operation(&mut self, op: impl Operation, cycles: u8) {
        self.remaining_cycles += cycles;
        op.run(self);
    }

    pub fn read_u8(&mut self) -> u8 {
        let res = self.mmu.get_byte(self.reg.pc());
        self.reg.incr_pc();
        res
    }

    pub fn read_u16(&mut self) -> u16 {
        let l = self.read_u8() as u16;
        let h = self.read_u8() as u16;
        (h << 8) | l
    }

    pub fn execute(&mut self, op_code: u8) {
        match op_code {
            0x00 => self.run_operation(Nop, 4),
            0x01 => self.run_operation(Ld16::new(Ld16Target::BC, Ld16Target::D16), 12),
            0x02 => self.run_operation(Ld::new(LdTarget::BCAddr, LdTarget::A), 8),
            0x03 => self.run_operation(Inc16::new(ArithmeticTarget16Bit::BC), 8),
            0x04 => self.run_operation(Inc::new(ArithmeticTarget8Bit::B), 4),
            0x05 => self.run_operation(Dec::new(ArithmeticTarget8Bit::B), 4),
            0x06 => self.run_operation(Ld::new(LdTarget::B, LdTarget::D8), 8),
            0x07 => self.run_operation(Rlca, 4),
            0x08 => self.run_operation(Ld16::new(Ld16Target::A16, Ld16Target::SP), 20),
            0x09 => self.run_operation(AddHl::new(ArithmeticTarget16Bit::BC), 8),
            0x0A => self.run_operation(Ld::new(LdTarget::A, LdTarget::BCAddr), 8),
            0x0B => self.run_operation(Dec16::new(ArithmeticTarget16Bit::BC), 8),
            0x0C => self.run_operation(Inc::new(ArithmeticTarget8Bit::C), 4),
            0x0D => self.run_operation(Dec::new(ArithmeticTarget8Bit::C), 4),
            0x0E => self.run_operation(Ld::new(LdTarget::C, LdTarget::D8), 8),
            0x0F => self.run_operation(Rrca, 4),

            0x10 => self.run_operation(Stop, 4),
            0x11 => self.run_operation(Ld16::new(Ld16Target::DE, Ld16Target::D16), 12),
            0x12 => self.run_operation(Ld::new(LdTarget::DEAddr, LdTarget::A), 8),
            0x13 => self.run_operation(Inc16::new(ArithmeticTarget16Bit::DE), 8),
            0x14 => self.run_operation(Inc::new(ArithmeticTarget8Bit::D), 4),
            0x15 => self.run_operation(Dec::new(ArithmeticTarget8Bit::D), 4),
            0x16 => self.run_operation(Ld::new(LdTarget::D, LdTarget::D8), 8),
            0x17 => self.run_operation(Rla, 4),
            0x18 => self.run_operation(Jr, 12),
            0x19 => self.run_operation(AddHl::new(ArithmeticTarget16Bit::DE), 8),
            0x1A => self.run_operation(Ld::new(LdTarget::A, LdTarget::DEAddr), 8),
            0x1B => self.run_operation(Dec16::new(ArithmeticTarget16Bit::DE), 8),
            0x1C => self.run_operation(Inc::new(ArithmeticTarget8Bit::E), 4),
            0x1D => self.run_operation(Dec::new(ArithmeticTarget8Bit::E), 4),
            0x1E => self.run_operation(Ld::new(LdTarget::E, LdTarget::D8), 8),
            0x1F => self.run_operation(Rra, 4),

            0x20 => self.run_operation(ConditionalJr::new(Condition::NZ), 12),
            0x21 => self.run_operation(Ld16::new(Ld16Target::HL, Ld16Target::D16), 12),
            0x22 => self.run_operation(Ld::new(LdTarget::HLIAddr, LdTarget::A), 8),
            0x23 => self.run_operation(Inc16::new(ArithmeticTarget16Bit::HL), 8),
            0x24 => self.run_operation(Inc::new(ArithmeticTarget8Bit::H), 4),
            0x25 => self.run_operation(Dec::new(ArithmeticTarget8Bit::H), 4),
            0x26 => self.run_operation(Ld::new(LdTarget::H, LdTarget::D8), 8),
            0x27 => self.run_operation(Daa, 4),
            0x28 => self.run_operation(ConditionalJr::new(Condition::Z), 12),
            0x29 => self.run_operation(AddHl::new(ArithmeticTarget16Bit::HL), 8),
            0x2A => self.run_operation(Ld::new(LdTarget::A, LdTarget::HLIAddr), 8),
            0x2B => self.run_operation(Dec16::new(ArithmeticTarget16Bit::HL), 8),
            0x2C => self.run_operation(Inc::new(ArithmeticTarget8Bit::L), 4),
            0x2D => self.run_operation(Dec::new(ArithmeticTarget8Bit::L), 4),
            0x2E => self.run_operation(Ld::new(LdTarget::L, LdTarget::D8), 8),
            0x2F => self.run_operation(Cpl, 4),

            0x30 => self.run_operation(ConditionalJr::new(Condition::NC), 12),
            0x31 => self.run_operation(Ld16::new(Ld16Target::SP, Ld16Target::D16), 12),
            0x32 => self.run_operation(Ld::new(LdTarget::HLDAddr, LdTarget::A), 8),
            0x33 => self.run_operation(Inc16::new(ArithmeticTarget16Bit::SP), 8),
            0x34 => self.run_operation(Inc::new(ArithmeticTarget8Bit::HLAddr), 12),
            0x35 => self.run_operation(Dec::new(ArithmeticTarget8Bit::HLAddr), 12),
            0x36 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::D8), 12),
            0x37 => self.run_operation(Scf, 4),
            0x38 => self.run_operation(ConditionalJr::new(Condition::C), 12),
            0x39 => self.run_operation(AddHl::new(ArithmeticTarget16Bit::SP), 8),
            0x3A => self.run_operation(Ld::new(LdTarget::A, LdTarget::HLIAddr), 8),
            0x3B => self.run_operation(Dec16::new(ArithmeticTarget16Bit::SP), 8),
            0x3C => self.run_operation(Inc::new(ArithmeticTarget8Bit::A), 4),
            0x3D => self.run_operation(Dec::new(ArithmeticTarget8Bit::A), 4),
            0x3E => self.run_operation(Ld::new(LdTarget::A, LdTarget::D8), 8),
            0x3F => self.run_operation(Ccf, 4),

            0x40 => self.run_operation(Ld::new(LdTarget::B, LdTarget::B), 4),
            0x41 => self.run_operation(Ld::new(LdTarget::B, LdTarget::C), 4),
            0x42 => self.run_operation(Ld::new(LdTarget::B, LdTarget::D), 4),
            0x43 => self.run_operation(Ld::new(LdTarget::B, LdTarget::E), 4),
            0x44 => self.run_operation(Ld::new(LdTarget::B, LdTarget::H), 4),
            0x45 => self.run_operation(Ld::new(LdTarget::B, LdTarget::L), 4),
            0x46 => self.run_operation(Ld::new(LdTarget::B, LdTarget::HLAddr), 8),
            0x47 => self.run_operation(Ld::new(LdTarget::B, LdTarget::A), 4),
            0x48 => self.run_operation(Ld::new(LdTarget::C, LdTarget::B), 4),
            0x49 => self.run_operation(Ld::new(LdTarget::C, LdTarget::C), 4),
            0x4A => self.run_operation(Ld::new(LdTarget::C, LdTarget::D), 4),
            0x4B => self.run_operation(Ld::new(LdTarget::C, LdTarget::E), 4),
            0x4C => self.run_operation(Ld::new(LdTarget::C, LdTarget::H), 4),
            0x4D => self.run_operation(Ld::new(LdTarget::C, LdTarget::L), 4),
            0x4E => self.run_operation(Ld::new(LdTarget::C, LdTarget::HLAddr), 8),
            0x4F => self.run_operation(Ld::new(LdTarget::C, LdTarget::A), 4),

            0x50 => self.run_operation(Ld::new(LdTarget::D, LdTarget::B), 4),
            0x51 => self.run_operation(Ld::new(LdTarget::D, LdTarget::C), 4),
            0x52 => self.run_operation(Ld::new(LdTarget::D, LdTarget::D), 4),
            0x53 => self.run_operation(Ld::new(LdTarget::D, LdTarget::E), 4),
            0x54 => self.run_operation(Ld::new(LdTarget::D, LdTarget::H), 4),
            0x55 => self.run_operation(Ld::new(LdTarget::D, LdTarget::L), 4),
            0x56 => self.run_operation(Ld::new(LdTarget::D, LdTarget::HLAddr), 8),
            0x57 => self.run_operation(Ld::new(LdTarget::D, LdTarget::A), 4),
            0x58 => self.run_operation(Ld::new(LdTarget::E, LdTarget::B), 4),
            0x59 => self.run_operation(Ld::new(LdTarget::E, LdTarget::C), 4),
            0x5A => self.run_operation(Ld::new(LdTarget::E, LdTarget::D), 4),
            0x5B => self.run_operation(Ld::new(LdTarget::E, LdTarget::E), 4),
            0x5C => self.run_operation(Ld::new(LdTarget::E, LdTarget::H), 4),
            0x5D => self.run_operation(Ld::new(LdTarget::E, LdTarget::L), 4),
            0x5E => self.run_operation(Ld::new(LdTarget::E, LdTarget::HLAddr), 8),
            0x5F => self.run_operation(Ld::new(LdTarget::E, LdTarget::A), 4),

            0x60 => self.run_operation(Ld::new(LdTarget::H, LdTarget::B), 4),
            0x61 => self.run_operation(Ld::new(LdTarget::H, LdTarget::C), 4),
            0x62 => self.run_operation(Ld::new(LdTarget::H, LdTarget::D), 4),
            0x63 => self.run_operation(Ld::new(LdTarget::H, LdTarget::E), 4),
            0x64 => self.run_operation(Ld::new(LdTarget::H, LdTarget::H), 4),
            0x65 => self.run_operation(Ld::new(LdTarget::H, LdTarget::L), 4),
            0x66 => self.run_operation(Ld::new(LdTarget::H, LdTarget::HLAddr), 8),
            0x67 => self.run_operation(Ld::new(LdTarget::H, LdTarget::A), 4),
            0x68 => self.run_operation(Ld::new(LdTarget::L, LdTarget::B), 4),
            0x69 => self.run_operation(Ld::new(LdTarget::L, LdTarget::C), 4),
            0x6A => self.run_operation(Ld::new(LdTarget::L, LdTarget::D), 4),
            0x6B => self.run_operation(Ld::new(LdTarget::L, LdTarget::E), 4),
            0x6C => self.run_operation(Ld::new(LdTarget::L, LdTarget::H), 4),
            0x6D => self.run_operation(Ld::new(LdTarget::L, LdTarget::L), 4),
            0x6E => self.run_operation(Ld::new(LdTarget::L, LdTarget::HLAddr), 8),
            0x6F => self.run_operation(Ld::new(LdTarget::L, LdTarget::A), 4),

            0x70 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::B), 8),
            0x71 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::C), 8),
            0x72 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::D), 8),
            0x73 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::E), 8),
            0x74 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::H), 8),
            0x75 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::L), 8),
            0x76 => self.run_operation(Halt, 4),
            0x77 => self.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::A), 8),
            0x78 => self.run_operation(Ld::new(LdTarget::A, LdTarget::B), 4),
            0x79 => self.run_operation(Ld::new(LdTarget::A, LdTarget::C), 4),
            0x7A => self.run_operation(Ld::new(LdTarget::A, LdTarget::D), 4),
            0x7B => self.run_operation(Ld::new(LdTarget::A, LdTarget::E), 4),
            0x7C => self.run_operation(Ld::new(LdTarget::A, LdTarget::H), 4),
            0x7D => self.run_operation(Ld::new(LdTarget::A, LdTarget::L), 4),
            0x7E => self.run_operation(Ld::new(LdTarget::A, LdTarget::HLAddr), 8),
            0x7F => self.run_operation(Ld::new(LdTarget::A, LdTarget::A), 4),

            0x80 => self.run_operation(Add::new(ArithmeticTarget8Bit::B), 4),
            0x81 => self.run_operation(Add::new(ArithmeticTarget8Bit::C), 4),
            0x82 => self.run_operation(Add::new(ArithmeticTarget8Bit::D), 4),
            0x83 => self.run_operation(Add::new(ArithmeticTarget8Bit::E), 4),
            0x84 => self.run_operation(Add::new(ArithmeticTarget8Bit::H), 4),
            0x85 => self.run_operation(Add::new(ArithmeticTarget8Bit::L), 4),
            0x86 => self.run_operation(Add::new(ArithmeticTarget8Bit::HLAddr), 8),
            0x87 => self.run_operation(Add::new(ArithmeticTarget8Bit::A), 4),
            0x88 => self.run_operation(Adc::new(ArithmeticTarget8Bit::B), 4),
            0x89 => self.run_operation(Adc::new(ArithmeticTarget8Bit::C), 4),
            0x8A => self.run_operation(Adc::new(ArithmeticTarget8Bit::D), 4),
            0x8B => self.run_operation(Adc::new(ArithmeticTarget8Bit::E), 4),
            0x8C => self.run_operation(Adc::new(ArithmeticTarget8Bit::H), 4),
            0x8D => self.run_operation(Adc::new(ArithmeticTarget8Bit::L), 4),
            0x8E => self.run_operation(Adc::new(ArithmeticTarget8Bit::HLAddr), 8),
            0x8F => self.run_operation(Adc::new(ArithmeticTarget8Bit::A), 4),

            0x90 => self.run_operation(Sub::new(ArithmeticTarget8Bit::B), 4),
            0x91 => self.run_operation(Sub::new(ArithmeticTarget8Bit::C), 4),
            0x92 => self.run_operation(Sub::new(ArithmeticTarget8Bit::D), 4),
            0x93 => self.run_operation(Sub::new(ArithmeticTarget8Bit::E), 4),
            0x94 => self.run_operation(Sub::new(ArithmeticTarget8Bit::H), 4),
            0x95 => self.run_operation(Sub::new(ArithmeticTarget8Bit::L), 4),
            0x96 => self.run_operation(Sub::new(ArithmeticTarget8Bit::HLAddr), 8),
            0x97 => self.run_operation(Sub::new(ArithmeticTarget8Bit::A), 4),
            0x98 => self.run_operation(Sbc::new(ArithmeticTarget8Bit::B), 4),
            0x99 => self.run_operation(Sbc::new(ArithmeticTarget8Bit::C), 4),
            0x9A => self.run_operation(Sbc::new(ArithmeticTarget8Bit::D), 4),
            0x9B => self.run_operation(Sbc::new(ArithmeticTarget8Bit::E), 4),
            0x9C => self.run_operation(Sbc::new(ArithmeticTarget8Bit::H), 4),
            0x9D => self.run_operation(Sbc::new(ArithmeticTarget8Bit::L), 4),
            0x9E => self.run_operation(Sbc::new(ArithmeticTarget8Bit::HLAddr), 8),
            0x9F => self.run_operation(Sbc::new(ArithmeticTarget8Bit::A), 4),

            0xA0 => self.run_operation(And::new(ArithmeticTarget8Bit::B), 4),
            0xA1 => self.run_operation(And::new(ArithmeticTarget8Bit::C), 4),
            0xA2 => self.run_operation(And::new(ArithmeticTarget8Bit::D), 4),
            0xA3 => self.run_operation(And::new(ArithmeticTarget8Bit::E), 4),
            0xA4 => self.run_operation(And::new(ArithmeticTarget8Bit::H), 4),
            0xA5 => self.run_operation(And::new(ArithmeticTarget8Bit::L), 4),
            0xA6 => self.run_operation(And::new(ArithmeticTarget8Bit::HLAddr), 8),
            0xA7 => self.run_operation(And::new(ArithmeticTarget8Bit::A), 4),
            0xA8 => self.run_operation(Xor::new(ArithmeticTarget8Bit::B), 4),
            0xA9 => self.run_operation(Xor::new(ArithmeticTarget8Bit::C), 4),
            0xAA => self.run_operation(Xor::new(ArithmeticTarget8Bit::D), 4),
            0xAB => self.run_operation(Xor::new(ArithmeticTarget8Bit::E), 4),
            0xAC => self.run_operation(Xor::new(ArithmeticTarget8Bit::H), 4),
            0xAD => self.run_operation(Xor::new(ArithmeticTarget8Bit::L), 4),
            0xAE => self.run_operation(Xor::new(ArithmeticTarget8Bit::HLAddr), 8),
            0xAF => self.run_operation(Xor::new(ArithmeticTarget8Bit::A), 4),

            0xB0 => self.run_operation(Or::new(ArithmeticTarget8Bit::B), 4),
            0xB1 => self.run_operation(Or::new(ArithmeticTarget8Bit::C), 4),
            0xB2 => self.run_operation(Or::new(ArithmeticTarget8Bit::D), 4),
            0xB3 => self.run_operation(Or::new(ArithmeticTarget8Bit::E), 4),
            0xB4 => self.run_operation(Or::new(ArithmeticTarget8Bit::H), 4),
            0xB5 => self.run_operation(Or::new(ArithmeticTarget8Bit::L), 4),
            0xB6 => self.run_operation(Or::new(ArithmeticTarget8Bit::HLAddr), 8),
            0xB7 => self.run_operation(Or::new(ArithmeticTarget8Bit::A), 4),
            0xB8 => self.run_operation(Cp::new(ArithmeticTarget8Bit::B), 4),
            0xB9 => self.run_operation(Cp::new(ArithmeticTarget8Bit::C), 4),
            0xBA => self.run_operation(Cp::new(ArithmeticTarget8Bit::D), 4),
            0xBB => self.run_operation(Cp::new(ArithmeticTarget8Bit::E), 4),
            0xBC => self.run_operation(Cp::new(ArithmeticTarget8Bit::H), 4),
            0xBD => self.run_operation(Cp::new(ArithmeticTarget8Bit::L), 4),
            0xBE => self.run_operation(Cp::new(ArithmeticTarget8Bit::HLAddr), 8),
            0xBF => self.run_operation(Cp::new(ArithmeticTarget8Bit::A), 4),

            0xC0 => self.run_operation(ConditionalRet::new(Condition::NZ), 20),
            0xC1 => self.run_operation(Pop::new(PushPopTarget::BC), 12),
            0xC2 => self.run_operation(ConditionalJp::new(Condition::NZ, AddressTarget::A16), 16),
            0xC3 => self.run_operation(Jp::new(AddressTarget::A16), 16),
            0xC4 => self.run_operation(ConditionalCall::new(Condition::NZ), 24),
            0xC5 => self.run_operation(Push::new(PushPopTarget::BC), 16),
            0xC6 => self.run_operation(Add::new(ArithmeticTarget8Bit::D8), 8),
            0xC7 => self.run_operation(Rst::new(0x00), 16),
            0xC8 => self.run_operation(ConditionalRet::new(Condition::Z), 20),
            0xC9 => self.run_operation(Ret, 16),
            0xCA => self.run_operation(ConditionalJp::new(Condition::Z, AddressTarget::A16), 16),
            0xCB => self.run_operation(PrefixCB, 4),
            0xCC => self.run_operation(ConditionalCall::new(Condition::Z), 24),
            0xCD => self.run_operation(Call, 24),
            0xCE => self.run_operation(Adc::new(ArithmeticTarget8Bit::D8), 8),
            0xCF => self.run_operation(Rst::new(0x08), 16),

            0xD0 => self.run_operation(ConditionalRet::new(Condition::NC), 20),
            0xD1 => self.run_operation(Pop::new(PushPopTarget::DE), 12),
            0xD2 => self.run_operation(ConditionalJp::new(Condition::NC, AddressTarget::A16), 16),

            0xD4 => self.run_operation(ConditionalCall::new(Condition::NC), 24),
            0xD5 => self.run_operation(Push::new(PushPopTarget::DE), 16),
            0xD6 => self.run_operation(Sub::new(ArithmeticTarget8Bit::D8), 8),
            0xD7 => self.run_operation(Rst::new(0x10), 16),
            0xD8 => self.run_operation(ConditionalRet::new(Condition::C), 20),
            0xD9 => self.run_operation(Reti, 16),
            0xDA => self.run_operation(ConditionalJp::new(Condition::C, AddressTarget::A16), 16),

            0xDC => self.run_operation(ConditionalCall::new(Condition::C), 24),

            0xDE => self.run_operation(Sbc::new(ArithmeticTarget8Bit::D8), 8),
            0xDF => self.run_operation(Rst::new(0x18), 16),

            0xE0 => self.run_operation(Ld::new(LdTarget::A8, LdTarget::A), 12),
            0xE1 => self.run_operation(Pop::new(PushPopTarget::HL), 12),
            0xE2 => self.run_operation(Ld::new(LdTarget::CAddr, LdTarget::A), 8),

            0xE5 => self.run_operation(Push::new(PushPopTarget::HL), 16),
            0xE6 => self.run_operation(And::new(ArithmeticTarget8Bit::D8), 8),
            0xE7 => self.run_operation(Rst::new(0x20), 16),
            0xE8 => self.run_operation(AddSp, 16),
            0xE9 => self.run_operation(Jp::new(AddressTarget::HLAddr), 4),
            0xEA => self.run_operation(Ld::new(LdTarget::A, LdTarget::A16), 16),

            0xEE => self.run_operation(Xor::new(ArithmeticTarget8Bit::D8), 8),
            0xEF => self.run_operation(Rst::new(0x28), 16),

            0xF0 => self.run_operation(Ld::new(LdTarget::A, LdTarget::A8), 12),
            0xF1 => self.run_operation(Pop::new(PushPopTarget::AF), 12),
            0xF2 => self.run_operation(Ld::new(LdTarget::A, LdTarget::CAddr), 8),
            0xF3 => self.run_operation(Di, 4),

            0xF5 => self.run_operation(Push::new(PushPopTarget::AF), 16),
            0xF6 => self.run_operation(Or::new(ArithmeticTarget8Bit::D8), 8),
            0xF7 => self.run_operation(Rst::new(0x30), 16),
            0xF8 => self.run_operation(LdHlSp, 12),
            0xF9 => self.run_operation(Ld16::new(Ld16Target::SP, Ld16Target::HL), 8),
            0xFA => self.run_operation(Ld::new(LdTarget::A16, LdTarget::A), 16),
            0xFB => self.run_operation(Ei, 4),

            0xFE => self.run_operation(Cp::new(ArithmeticTarget8Bit::D8), 8),
            0xFF => self.run_operation(Rst::new(0x38), 16),

            _ => panic!("No op code corresponding to {op_code:#04X}"),
        }
    }

    pub fn execute_extended(&mut self, op_code: u8) {
        match op_code {
            0x00 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::B), 8),
            0x01 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::C), 8),
            0x02 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::D), 8),
            0x03 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::E), 8),
            0x04 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::H), 8),
            0x05 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::L), 8),
            0x06 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x07 => self.run_operation(Rlc::new(ArithmeticTarget8Bit::A), 8),
            0x08 => self.run_operation(Rrc::new(ArithmeticTarget8Bit::B), 8),
            0x09 => self.run_operation(Rrc::new(ArithmeticTarget8Bit::C), 8),
            0x0A => self.run_operation(Rrc::new(ArithmeticTarget8Bit::D), 8),
            0x0B => self.run_operation(Rrc::new(ArithmeticTarget8Bit::E), 8),
            0x0C => self.run_operation(Rrc::new(ArithmeticTarget8Bit::H), 8),
            0x0D => self.run_operation(Rrc::new(ArithmeticTarget8Bit::L), 8),
            0x0E => self.run_operation(Rrc::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x0F => self.run_operation(Rrc::new(ArithmeticTarget8Bit::A), 8),

            0x10 => self.run_operation(Rl::new(ArithmeticTarget8Bit::B), 8),
            0x11 => self.run_operation(Rl::new(ArithmeticTarget8Bit::C), 8),
            0x12 => self.run_operation(Rl::new(ArithmeticTarget8Bit::D), 8),
            0x13 => self.run_operation(Rl::new(ArithmeticTarget8Bit::E), 8),
            0x14 => self.run_operation(Rl::new(ArithmeticTarget8Bit::H), 8),
            0x15 => self.run_operation(Rl::new(ArithmeticTarget8Bit::L), 8),
            0x16 => self.run_operation(Rl::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x17 => self.run_operation(Rl::new(ArithmeticTarget8Bit::A), 8),
            0x18 => self.run_operation(Rr::new(ArithmeticTarget8Bit::B), 8),
            0x19 => self.run_operation(Rr::new(ArithmeticTarget8Bit::C), 8),
            0x1A => self.run_operation(Rr::new(ArithmeticTarget8Bit::D), 8),
            0x1B => self.run_operation(Rr::new(ArithmeticTarget8Bit::E), 8),
            0x1C => self.run_operation(Rr::new(ArithmeticTarget8Bit::H), 8),
            0x1D => self.run_operation(Rr::new(ArithmeticTarget8Bit::L), 8),
            0x1E => self.run_operation(Rr::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x1F => self.run_operation(Rr::new(ArithmeticTarget8Bit::A), 8),

            0x20 => self.run_operation(Sla::new(ArithmeticTarget8Bit::B), 8),
            0x21 => self.run_operation(Sla::new(ArithmeticTarget8Bit::C), 8),
            0x22 => self.run_operation(Sla::new(ArithmeticTarget8Bit::D), 8),
            0x23 => self.run_operation(Sla::new(ArithmeticTarget8Bit::E), 8),
            0x24 => self.run_operation(Sla::new(ArithmeticTarget8Bit::H), 8),
            0x25 => self.run_operation(Sla::new(ArithmeticTarget8Bit::L), 8),
            0x26 => self.run_operation(Sla::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x27 => self.run_operation(Sla::new(ArithmeticTarget8Bit::A), 8),
            0x28 => self.run_operation(Sra::new(ArithmeticTarget8Bit::B), 8),
            0x29 => self.run_operation(Sra::new(ArithmeticTarget8Bit::C), 8),
            0x2A => self.run_operation(Sra::new(ArithmeticTarget8Bit::D), 8),
            0x2B => self.run_operation(Sra::new(ArithmeticTarget8Bit::E), 8),
            0x2C => self.run_operation(Sra::new(ArithmeticTarget8Bit::H), 8),
            0x2D => self.run_operation(Sra::new(ArithmeticTarget8Bit::L), 8),
            0x2E => self.run_operation(Sra::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x2F => self.run_operation(Sra::new(ArithmeticTarget8Bit::A), 8),

            0x30 => self.run_operation(Swap::new(ArithmeticTarget8Bit::B), 8),
            0x31 => self.run_operation(Swap::new(ArithmeticTarget8Bit::C), 8),
            0x32 => self.run_operation(Swap::new(ArithmeticTarget8Bit::D), 8),
            0x33 => self.run_operation(Swap::new(ArithmeticTarget8Bit::E), 8),
            0x34 => self.run_operation(Swap::new(ArithmeticTarget8Bit::H), 8),
            0x35 => self.run_operation(Swap::new(ArithmeticTarget8Bit::L), 8),
            0x36 => self.run_operation(Swap::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x37 => self.run_operation(Swap::new(ArithmeticTarget8Bit::A), 8),
            0x38 => self.run_operation(Srl::new(ArithmeticTarget8Bit::B), 8),
            0x39 => self.run_operation(Srl::new(ArithmeticTarget8Bit::C), 8),
            0x3A => self.run_operation(Srl::new(ArithmeticTarget8Bit::D), 8),
            0x3B => self.run_operation(Srl::new(ArithmeticTarget8Bit::E), 8),
            0x3C => self.run_operation(Srl::new(ArithmeticTarget8Bit::H), 8),
            0x3D => self.run_operation(Srl::new(ArithmeticTarget8Bit::L), 8),
            0x3E => self.run_operation(Srl::new(ArithmeticTarget8Bit::HLAddr), 16),
            0x3F => self.run_operation(Srl::new(ArithmeticTarget8Bit::A), 8),

            0x40 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::B), 8),
            0x41 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::C), 8),
            0x42 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::D), 8),
            0x43 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::E), 8),
            0x44 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::H), 8),
            0x45 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::L), 8),
            0x46 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::HLAddr), 16),
            0x47 => self.run_operation(Bit::new(0, ArithmeticTarget8Bit::A), 8),
            0x48 => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::B), 8),
            0x49 => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::C), 8),
            0x4A => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::D), 8),
            0x4B => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::E), 8),
            0x4C => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::H), 8),
            0x4D => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::L), 8),
            0x4E => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::HLAddr), 16),
            0x4F => self.run_operation(Bit::new(1, ArithmeticTarget8Bit::A), 8),

            0x50 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::B), 8),
            0x51 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::C), 8),
            0x52 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::D), 8),
            0x53 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::E), 8),
            0x54 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::H), 8),
            0x55 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::L), 8),
            0x56 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::HLAddr), 16),
            0x57 => self.run_operation(Bit::new(2, ArithmeticTarget8Bit::A), 8),
            0x58 => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::B), 8),
            0x59 => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::C), 8),
            0x5A => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::D), 8),
            0x5B => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::E), 8),
            0x5C => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::H), 8),
            0x5D => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::L), 8),
            0x5E => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::HLAddr), 16),
            0x5F => self.run_operation(Bit::new(3, ArithmeticTarget8Bit::A), 8),

            0x60 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::B), 8),
            0x61 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::C), 8),
            0x62 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::D), 8),
            0x63 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::E), 8),
            0x64 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::H), 8),
            0x65 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::L), 8),
            0x66 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::HLAddr), 16),
            0x67 => self.run_operation(Bit::new(4, ArithmeticTarget8Bit::A), 8),
            0x68 => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::B), 8),
            0x69 => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::C), 8),
            0x6A => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::D), 8),
            0x6B => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::E), 8),
            0x6C => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::H), 8),
            0x6D => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::L), 8),
            0x6E => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::HLAddr), 16),
            0x6F => self.run_operation(Bit::new(5, ArithmeticTarget8Bit::A), 8),

            0x70 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::B), 8),
            0x71 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::C), 8),
            0x72 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::D), 8),
            0x73 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::E), 8),
            0x74 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::H), 8),
            0x75 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::L), 8),
            0x76 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::HLAddr), 16),
            0x77 => self.run_operation(Bit::new(6, ArithmeticTarget8Bit::A), 8),
            0x78 => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::B), 8),
            0x79 => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::C), 8),
            0x7A => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::D), 8),
            0x7B => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::E), 8),
            0x7C => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::H), 8),
            0x7D => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::L), 8),
            0x7E => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::HLAddr), 16),
            0x7F => self.run_operation(Bit::new(7, ArithmeticTarget8Bit::A), 8),

            0x80 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::B), 8),
            0x81 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::C), 8),
            0x82 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::D), 8),
            0x83 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::E), 8),
            0x84 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::H), 8),
            0x85 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::L), 8),
            0x86 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::HLAddr), 16),
            0x87 => self.run_operation(Res::new(0, ArithmeticTarget8Bit::A), 8),
            0x88 => self.run_operation(Res::new(1, ArithmeticTarget8Bit::B), 8),
            0x89 => self.run_operation(Res::new(1, ArithmeticTarget8Bit::C), 8),
            0x8A => self.run_operation(Res::new(1, ArithmeticTarget8Bit::D), 8),
            0x8B => self.run_operation(Res::new(1, ArithmeticTarget8Bit::E), 8),
            0x8C => self.run_operation(Res::new(1, ArithmeticTarget8Bit::H), 8),
            0x8D => self.run_operation(Res::new(1, ArithmeticTarget8Bit::L), 8),
            0x8E => self.run_operation(Res::new(1, ArithmeticTarget8Bit::HLAddr), 16),
            0x8F => self.run_operation(Res::new(1, ArithmeticTarget8Bit::A), 8),

            0x90 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::B), 8),
            0x91 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::C), 8),
            0x92 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::D), 8),
            0x93 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::E), 8),
            0x94 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::H), 8),
            0x95 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::L), 8),
            0x96 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::HLAddr), 16),
            0x97 => self.run_operation(Res::new(2, ArithmeticTarget8Bit::A), 8),
            0x98 => self.run_operation(Res::new(3, ArithmeticTarget8Bit::B), 8),
            0x99 => self.run_operation(Res::new(3, ArithmeticTarget8Bit::C), 8),
            0x9A => self.run_operation(Res::new(3, ArithmeticTarget8Bit::D), 8),
            0x9B => self.run_operation(Res::new(3, ArithmeticTarget8Bit::E), 8),
            0x9C => self.run_operation(Res::new(3, ArithmeticTarget8Bit::H), 8),
            0x9D => self.run_operation(Res::new(3, ArithmeticTarget8Bit::L), 8),
            0x9E => self.run_operation(Res::new(3, ArithmeticTarget8Bit::HLAddr), 16),
            0x9F => self.run_operation(Res::new(3, ArithmeticTarget8Bit::A), 8),

            0xA0 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::B), 8),
            0xA1 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::C), 8),
            0xA2 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::D), 8),
            0xA3 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::E), 8),
            0xA4 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::H), 8),
            0xA5 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::L), 8),
            0xA6 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::HLAddr), 16),
            0xA7 => self.run_operation(Res::new(4, ArithmeticTarget8Bit::A), 8),
            0xA8 => self.run_operation(Res::new(5, ArithmeticTarget8Bit::B), 8),
            0xA9 => self.run_operation(Res::new(5, ArithmeticTarget8Bit::C), 8),
            0xAA => self.run_operation(Res::new(5, ArithmeticTarget8Bit::D), 8),
            0xAB => self.run_operation(Res::new(5, ArithmeticTarget8Bit::E), 8),
            0xAC => self.run_operation(Res::new(5, ArithmeticTarget8Bit::H), 8),
            0xAD => self.run_operation(Res::new(5, ArithmeticTarget8Bit::L), 8),
            0xAE => self.run_operation(Res::new(5, ArithmeticTarget8Bit::HLAddr), 16),
            0xAF => self.run_operation(Res::new(5, ArithmeticTarget8Bit::A), 8),

            0xB0 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::B), 8),
            0xB1 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::C), 8),
            0xB2 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::D), 8),
            0xB3 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::E), 8),
            0xB4 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::H), 8),
            0xB5 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::L), 8),
            0xB6 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::HLAddr), 16),
            0xB7 => self.run_operation(Res::new(6, ArithmeticTarget8Bit::A), 8),
            0xB8 => self.run_operation(Res::new(7, ArithmeticTarget8Bit::B), 8),
            0xB9 => self.run_operation(Res::new(7, ArithmeticTarget8Bit::C), 8),
            0xBA => self.run_operation(Res::new(7, ArithmeticTarget8Bit::D), 8),
            0xBB => self.run_operation(Res::new(7, ArithmeticTarget8Bit::E), 8),
            0xBC => self.run_operation(Res::new(7, ArithmeticTarget8Bit::H), 8),
            0xBD => self.run_operation(Res::new(7, ArithmeticTarget8Bit::L), 8),
            0xBE => self.run_operation(Res::new(7, ArithmeticTarget8Bit::HLAddr), 16),
            0xBF => self.run_operation(Res::new(7, ArithmeticTarget8Bit::A), 8),

            0xC0 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::B), 8),
            0xC1 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::C), 8),
            0xC2 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::D), 8),
            0xC3 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::E), 8),
            0xC4 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::H), 8),
            0xC5 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::L), 8),
            0xC6 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::HLAddr), 16),
            0xC7 => self.run_operation(Set::new(0, ArithmeticTarget8Bit::A), 8),
            0xC8 => self.run_operation(Set::new(1, ArithmeticTarget8Bit::B), 8),
            0xC9 => self.run_operation(Set::new(1, ArithmeticTarget8Bit::C), 8),
            0xCA => self.run_operation(Set::new(1, ArithmeticTarget8Bit::D), 8),
            0xCB => self.run_operation(Set::new(1, ArithmeticTarget8Bit::E), 8),
            0xCC => self.run_operation(Set::new(1, ArithmeticTarget8Bit::H), 8),
            0xCD => self.run_operation(Set::new(1, ArithmeticTarget8Bit::L), 8),
            0xCE => self.run_operation(Set::new(1, ArithmeticTarget8Bit::HLAddr), 16),
            0xCF => self.run_operation(Set::new(1, ArithmeticTarget8Bit::A), 8),

            0xD0 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::B), 8),
            0xD1 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::C), 8),
            0xD2 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::D), 8),
            0xD3 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::E), 8),
            0xD4 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::H), 8),
            0xD5 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::L), 8),
            0xD6 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::HLAddr), 16),
            0xD7 => self.run_operation(Set::new(2, ArithmeticTarget8Bit::A), 8),
            0xD8 => self.run_operation(Set::new(3, ArithmeticTarget8Bit::B), 8),
            0xD9 => self.run_operation(Set::new(3, ArithmeticTarget8Bit::C), 8),
            0xDA => self.run_operation(Set::new(3, ArithmeticTarget8Bit::D), 8),
            0xDB => self.run_operation(Set::new(3, ArithmeticTarget8Bit::E), 8),
            0xDC => self.run_operation(Set::new(3, ArithmeticTarget8Bit::H), 8),
            0xDD => self.run_operation(Set::new(3, ArithmeticTarget8Bit::L), 8),
            0xDE => self.run_operation(Set::new(3, ArithmeticTarget8Bit::HLAddr), 16),
            0xDF => self.run_operation(Set::new(3, ArithmeticTarget8Bit::A), 8),

            0xE0 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::B), 8),
            0xE1 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::C), 8),
            0xE2 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::D), 8),
            0xE3 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::E), 8),
            0xE4 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::H), 8),
            0xE5 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::L), 8),
            0xE6 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::HLAddr), 16),
            0xE7 => self.run_operation(Set::new(4, ArithmeticTarget8Bit::A), 8),
            0xE8 => self.run_operation(Set::new(5, ArithmeticTarget8Bit::B), 8),
            0xE9 => self.run_operation(Set::new(5, ArithmeticTarget8Bit::C), 8),
            0xEA => self.run_operation(Set::new(5, ArithmeticTarget8Bit::D), 8),
            0xEB => self.run_operation(Set::new(5, ArithmeticTarget8Bit::E), 8),
            0xEC => self.run_operation(Set::new(5, ArithmeticTarget8Bit::H), 8),
            0xED => self.run_operation(Set::new(5, ArithmeticTarget8Bit::L), 8),
            0xEE => self.run_operation(Set::new(5, ArithmeticTarget8Bit::HLAddr), 16),
            0xEF => self.run_operation(Set::new(5, ArithmeticTarget8Bit::A), 8),

            0xF0 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::B), 8),
            0xF1 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::C), 8),
            0xF2 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::D), 8),
            0xF3 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::E), 8),
            0xF4 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::H), 8),
            0xF5 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::L), 8),
            0xF6 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::HLAddr), 16),
            0xF7 => self.run_operation(Set::new(6, ArithmeticTarget8Bit::A), 8),
            0xF8 => self.run_operation(Set::new(7, ArithmeticTarget8Bit::B), 8),
            0xF9 => self.run_operation(Set::new(7, ArithmeticTarget8Bit::C), 8),
            0xFA => self.run_operation(Set::new(7, ArithmeticTarget8Bit::D), 8),
            0xFB => self.run_operation(Set::new(7, ArithmeticTarget8Bit::E), 8),
            0xFC => self.run_operation(Set::new(7, ArithmeticTarget8Bit::H), 8),
            0xFD => self.run_operation(Set::new(7, ArithmeticTarget8Bit::L), 8),
            0xFE => self.run_operation(Set::new(7, ArithmeticTarget8Bit::HLAddr), 16),
            0xFF => self.run_operation(Set::new(7, ArithmeticTarget8Bit::A), 8),
        }
    }
}
