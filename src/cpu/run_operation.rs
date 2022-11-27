use crate::cpu::Cpu;

use super::operations::condition::Condition;
use super::operations::targets::{
    AddressTarget, ArithmeticTarget16Bit, ArithmeticTarget8Bit, Ld16Target, LdTarget, PushPopTarget,
};

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

pub fn run_operation(cpu: &mut Cpu, op_code: u8) {
    match op_code {
        0x00 => cpu.run_operation(Nop, 4),
        0x01 => cpu.run_operation(Ld16::new(Ld16Target::BC, Ld16Target::D16), 12),
        0x02 => cpu.run_operation(Ld::new(LdTarget::BCAddr, LdTarget::A), 8),
        0x03 => cpu.run_operation(Inc16::new(ArithmeticTarget16Bit::BC), 8),
        0x04 => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::B), 4),
        0x05 => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::B), 4),
        0x06 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::D8), 8),
        0x07 => cpu.run_operation(Rlca, 4),
        0x08 => cpu.run_operation(Ld16::new(Ld16Target::A16, Ld16Target::SP), 20),
        0x09 => cpu.run_operation(AddHl::new(ArithmeticTarget16Bit::BC), 8),
        0x0A => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::BCAddr), 8),
        0x0B => cpu.run_operation(Dec16::new(ArithmeticTarget16Bit::BC), 8),
        0x0C => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::C), 4),
        0x0D => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::C), 4),
        0x0E => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::D8), 8),
        0x0F => cpu.run_operation(Rrca, 4),

        0x10 => cpu.run_operation(Stop, 4),
        0x11 => cpu.run_operation(Ld16::new(Ld16Target::DE, Ld16Target::D16), 12),
        0x12 => cpu.run_operation(Ld::new(LdTarget::DEAddr, LdTarget::A), 8),
        0x13 => cpu.run_operation(Inc16::new(ArithmeticTarget16Bit::DE), 8),
        0x14 => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::D), 4),
        0x15 => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::D), 4),
        0x16 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::D8), 8),
        0x17 => cpu.run_operation(Rla, 4),
        0x18 => cpu.run_operation(Jr, 12),
        0x19 => cpu.run_operation(AddHl::new(ArithmeticTarget16Bit::DE), 8),
        0x1A => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::DEAddr), 8),
        0x1B => cpu.run_operation(Dec16::new(ArithmeticTarget16Bit::DE), 8),
        0x1C => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::E), 4),
        0x1D => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::E), 4),
        0x1E => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::D8), 8),
        0x1F => cpu.run_operation(Rra, 4),

        0x20 => cpu.run_operation(ConditionalJr::new(Condition::NZ), 12),
        0x21 => cpu.run_operation(Ld16::new(Ld16Target::HL, Ld16Target::D16), 12),
        0x22 => cpu.run_operation(Ld::new(LdTarget::HLIAddr, LdTarget::A), 8),
        0x23 => cpu.run_operation(Inc16::new(ArithmeticTarget16Bit::HL), 8),
        0x24 => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::H), 4),
        0x25 => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::H), 4),
        0x26 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::D8), 8),
        0x27 => cpu.run_operation(Daa, 4),
        0x28 => cpu.run_operation(ConditionalJr::new(Condition::Z), 12),
        0x29 => cpu.run_operation(AddHl::new(ArithmeticTarget16Bit::HL), 8),
        0x2A => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::HLIAddr), 8),
        0x2B => cpu.run_operation(Dec16::new(ArithmeticTarget16Bit::HL), 8),
        0x2C => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::L), 4),
        0x2D => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::L), 4),
        0x2E => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::D8), 8),
        0x2F => cpu.run_operation(Cpl, 4),

        0x30 => cpu.run_operation(ConditionalJr::new(Condition::NC), 12),
        0x31 => cpu.run_operation(Ld16::new(Ld16Target::SP, Ld16Target::D16), 12),
        0x32 => cpu.run_operation(Ld::new(LdTarget::HLDAddr, LdTarget::A), 8),
        0x33 => cpu.run_operation(Inc16::new(ArithmeticTarget16Bit::SP), 8),
        0x34 => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::HLAddr), 12),
        0x35 => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::HLAddr), 12),
        0x36 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::D8), 12),
        0x37 => cpu.run_operation(Scf, 4),
        0x38 => cpu.run_operation(ConditionalJr::new(Condition::C), 12),
        0x39 => cpu.run_operation(AddHl::new(ArithmeticTarget16Bit::SP), 8),
        0x3A => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::HLIAddr), 8),
        0x3B => cpu.run_operation(Dec16::new(ArithmeticTarget16Bit::SP), 8),
        0x3C => cpu.run_operation(Inc::new(ArithmeticTarget8Bit::A), 4),
        0x3D => cpu.run_operation(Dec::new(ArithmeticTarget8Bit::A), 4),
        0x3E => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::D8), 8),
        0x3F => cpu.run_operation(Ccf, 4),

        0x40 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::B), 4),
        0x41 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::C), 4),
        0x42 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::D), 4),
        0x43 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::E), 4),
        0x44 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::H), 4),
        0x45 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::L), 4),
        0x46 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::HLAddr), 8),
        0x47 => cpu.run_operation(Ld::new(LdTarget::B, LdTarget::A), 4),
        0x48 => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::B), 4),
        0x49 => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::C), 4),
        0x4A => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::D), 4),
        0x4B => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::E), 4),
        0x4C => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::H), 4),
        0x4D => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::L), 4),
        0x4E => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::HLAddr), 8),
        0x4F => cpu.run_operation(Ld::new(LdTarget::C, LdTarget::A), 4),

        0x50 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::B), 4),
        0x51 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::C), 4),
        0x52 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::D), 4),
        0x53 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::E), 4),
        0x54 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::H), 4),
        0x55 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::L), 4),
        0x56 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::HLAddr), 8),
        0x57 => cpu.run_operation(Ld::new(LdTarget::D, LdTarget::A), 4),
        0x58 => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::B), 4),
        0x59 => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::C), 4),
        0x5A => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::D), 4),
        0x5B => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::E), 4),
        0x5C => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::H), 4),
        0x5D => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::L), 4),
        0x5E => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::HLAddr), 8),
        0x5F => cpu.run_operation(Ld::new(LdTarget::E, LdTarget::A), 4),

        0x60 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::B), 4),
        0x61 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::C), 4),
        0x62 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::D), 4),
        0x63 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::E), 4),
        0x64 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::H), 4),
        0x65 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::L), 4),
        0x66 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::HLAddr), 8),
        0x67 => cpu.run_operation(Ld::new(LdTarget::H, LdTarget::A), 4),
        0x68 => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::B), 4),
        0x69 => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::C), 4),
        0x6A => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::D), 4),
        0x6B => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::E), 4),
        0x6C => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::H), 4),
        0x6D => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::L), 4),
        0x6E => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::HLAddr), 8),
        0x6F => cpu.run_operation(Ld::new(LdTarget::L, LdTarget::A), 4),

        0x70 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::B), 8),
        0x71 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::C), 8),
        0x72 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::D), 8),
        0x73 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::E), 8),
        0x74 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::H), 8),
        0x75 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::L), 8),
        0x76 => cpu.run_operation(Halt, 4),
        0x77 => cpu.run_operation(Ld::new(LdTarget::HLAddr, LdTarget::A), 8),
        0x78 => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::B), 4),
        0x79 => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::C), 4),
        0x7A => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::D), 4),
        0x7B => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::E), 4),
        0x7C => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::H), 4),
        0x7D => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::L), 4),
        0x7E => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::HLAddr), 8),
        0x7F => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::A), 4),

        0x80 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::B), 4),
        0x81 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::C), 4),
        0x82 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::D), 4),
        0x83 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::E), 4),
        0x84 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::H), 4),
        0x85 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::L), 4),
        0x86 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::HLAddr), 8),
        0x87 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::A), 4),
        0x88 => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::B), 4),
        0x89 => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::C), 4),
        0x8A => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::D), 4),
        0x8B => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::E), 4),
        0x8C => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::H), 4),
        0x8D => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::L), 4),
        0x8E => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::HLAddr), 8),
        0x8F => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::A), 4),

        0x90 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::B), 4),
        0x91 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::C), 4),
        0x92 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::D), 4),
        0x93 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::E), 4),
        0x94 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::H), 4),
        0x95 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::L), 4),
        0x96 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::HLAddr), 8),
        0x97 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::A), 4),
        0x98 => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::B), 4),
        0x99 => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::C), 4),
        0x9A => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::D), 4),
        0x9B => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::E), 4),
        0x9C => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::H), 4),
        0x9D => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::L), 4),
        0x9E => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::HLAddr), 8),
        0x9F => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::A), 4),

        0xA0 => cpu.run_operation(And::new(ArithmeticTarget8Bit::B), 4),
        0xA1 => cpu.run_operation(And::new(ArithmeticTarget8Bit::C), 4),
        0xA2 => cpu.run_operation(And::new(ArithmeticTarget8Bit::D), 4),
        0xA3 => cpu.run_operation(And::new(ArithmeticTarget8Bit::E), 4),
        0xA4 => cpu.run_operation(And::new(ArithmeticTarget8Bit::H), 4),
        0xA5 => cpu.run_operation(And::new(ArithmeticTarget8Bit::L), 4),
        0xA6 => cpu.run_operation(And::new(ArithmeticTarget8Bit::HLAddr), 8),
        0xA7 => cpu.run_operation(And::new(ArithmeticTarget8Bit::A), 4),
        0xA8 => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::B), 4),
        0xA9 => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::C), 4),
        0xAA => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::D), 4),
        0xAB => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::E), 4),
        0xAC => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::H), 4),
        0xAD => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::L), 4),
        0xAE => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::HLAddr), 8),
        0xAF => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::A), 4),

        0xB0 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::B), 4),
        0xB1 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::C), 4),
        0xB2 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::D), 4),
        0xB3 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::E), 4),
        0xB4 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::H), 4),
        0xB5 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::L), 4),
        0xB6 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::HLAddr), 8),
        0xB7 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::A), 4),
        0xB8 => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::B), 4),
        0xB9 => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::C), 4),
        0xBA => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::D), 4),
        0xBB => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::E), 4),
        0xBC => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::H), 4),
        0xBD => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::L), 4),
        0xBE => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::HLAddr), 8),
        0xBF => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::A), 4),

        0xC0 => cpu.run_operation(ConditionalRet::new(Condition::NZ), 20),
        0xC1 => cpu.run_operation(Pop::new(PushPopTarget::BC), 12),
        0xC2 => cpu.run_operation(ConditionalJp::new(Condition::NZ, AddressTarget::A16), 16),
        0xC3 => cpu.run_operation(Jp::new(AddressTarget::A16), 16),
        0xC4 => cpu.run_operation(ConditionalCall::new(Condition::NZ), 24),
        0xC5 => cpu.run_operation(Push::new(PushPopTarget::BC), 16),
        0xC6 => cpu.run_operation(Add::new(ArithmeticTarget8Bit::D8), 8),
        0xC7 => cpu.run_operation(Rst::new(0x00), 16),
        0xC8 => cpu.run_operation(ConditionalRet::new(Condition::Z), 20),
        0xC9 => cpu.run_operation(Ret, 16),
        0xCA => cpu.run_operation(ConditionalJp::new(Condition::Z, AddressTarget::A16), 16),
        0xCB => cpu.run_operation(PrefixCB, 4),
        0xCC => cpu.run_operation(ConditionalCall::new(Condition::Z), 24),
        0xCD => cpu.run_operation(Call, 24),
        0xCE => cpu.run_operation(Adc::new(ArithmeticTarget8Bit::D8), 8),
        0xCF => cpu.run_operation(Rst::new(0x08), 16),

        0xD0 => cpu.run_operation(ConditionalRet::new(Condition::NC), 20),
        0xD1 => cpu.run_operation(Pop::new(PushPopTarget::DE), 12),
        0xD2 => cpu.run_operation(ConditionalJp::new(Condition::NC, AddressTarget::A16), 16),

        0xD4 => cpu.run_operation(ConditionalCall::new(Condition::NC), 24),
        0xD5 => cpu.run_operation(Push::new(PushPopTarget::DE), 16),
        0xD6 => cpu.run_operation(Sub::new(ArithmeticTarget8Bit::D8), 8),
        0xD7 => cpu.run_operation(Rst::new(0x10), 16),
        0xD8 => cpu.run_operation(ConditionalRet::new(Condition::C), 20),
        0xD9 => cpu.run_operation(Reti, 16),
        0xDA => cpu.run_operation(ConditionalJp::new(Condition::C, AddressTarget::A16), 16),

        0xDC => cpu.run_operation(ConditionalCall::new(Condition::C), 24),

        0xDE => cpu.run_operation(Sbc::new(ArithmeticTarget8Bit::D8), 8),
        0xDF => cpu.run_operation(Rst::new(0x18), 16),

        0xE0 => cpu.run_operation(Ld::new(LdTarget::A8, LdTarget::A), 12),
        0xE1 => cpu.run_operation(Pop::new(PushPopTarget::HL), 12),
        0xE2 => cpu.run_operation(Ld::new(LdTarget::CAddr, LdTarget::A), 8),

        0xE5 => cpu.run_operation(Push::new(PushPopTarget::HL), 16),
        0xE6 => cpu.run_operation(And::new(ArithmeticTarget8Bit::D8), 8),
        0xE7 => cpu.run_operation(Rst::new(0x20), 16),
        0xE8 => cpu.run_operation(AddSp, 16),
        0xE9 => cpu.run_operation(Jp::new(AddressTarget::HLAddr), 4),
        0xEA => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::A16), 16),

        0xEE => cpu.run_operation(Xor::new(ArithmeticTarget8Bit::D8), 8),
        0xEF => cpu.run_operation(Rst::new(0x28), 16),

        0xF0 => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::A8), 12),
        0xF1 => cpu.run_operation(Pop::new(PushPopTarget::AF), 12),
        0xF2 => cpu.run_operation(Ld::new(LdTarget::A, LdTarget::CAddr), 8),
        0xF3 => cpu.run_operation(Di, 4),

        0xF5 => cpu.run_operation(Push::new(PushPopTarget::AF), 16),
        0xF6 => cpu.run_operation(Or::new(ArithmeticTarget8Bit::D8), 8),
        0xF7 => cpu.run_operation(Rst::new(0x30), 16),
        0xF8 => cpu.run_operation(LdHlSp, 12),
        0xF9 => cpu.run_operation(Ld16::new(Ld16Target::SP, Ld16Target::HL), 8),
        0xFA => cpu.run_operation(Ld::new(LdTarget::A16, LdTarget::A), 16),
        0xFB => cpu.run_operation(Ei, 4),

        0xFE => cpu.run_operation(Cp::new(ArithmeticTarget8Bit::D8), 8),
        0xFF => cpu.run_operation(Rst::new(0x38), 16),

        _ => panic!("No op code corresponding to {op_code:#04X}"),
    }
}
