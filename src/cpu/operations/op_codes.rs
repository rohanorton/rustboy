use super::ld_16::Ld16;
use super::operation::Operation;
use super::rlca::Rlca;
use super::targets::{
    ArithmeticTarget16Bit, ArithmeticTarget8Bit, Ld16Target, LdTarget, PushPopTarget,
};

use super::adc::Adc;
use super::add::Add;
use super::add_hl::AddHl;
use super::add_sp::AddSp;
use super::and::And;
use super::ccf::Ccf;
use super::cp::Cp;
use super::cpl::Cpl;
use super::daa::Daa;
use super::dec::Dec;
use super::dec_16::Dec16;
use super::inc::Inc;
use super::inc_16::Inc16;
use super::ld::Ld;
use super::nop::Nop;
use super::or::Or;
use super::pop::Pop;
use super::push::Push;
use super::sbc::Sbc;
use super::scf::Scf;
use super::sub::Sub;
use super::xor::Xor;

// Macro to simplify op-code match creation. Wraps result in Box, to prevent type error.
macro_rules! boxed_operation {(
    $op_code:expr, { $($lhs:expr => $rhs:expr, $cycles:expr;)+ }
) => (
    match $op_code {
        $($lhs => (::std::boxed::Box::new($rhs), $cycles),)+
        _ => panic!("Unimplemented Op Code {:#02x}", $op_code),
    }
)}

pub fn lookup_op_code(op_code: u8) -> (Box<dyn Operation>, u8) {
    boxed_operation!(op_code, {
        0x00 => Nop, 4;
        0x01 => Ld16::new (Ld16Target::BC, Ld16Target::D16), 12;
        0x02 => Ld::new (LdTarget::BCAddr, LdTarget::A), 8;
        0x03 => Inc16::new(ArithmeticTarget16Bit::BC), 8;
        0x04 => Inc::new(ArithmeticTarget8Bit::B), 4;
        0x05 => Dec::new(ArithmeticTarget8Bit::B), 4;
        0x06 => Ld::new (LdTarget::B, LdTarget::D8), 8;
        0x07 => Rlca, 4;
        0x08 => Ld16::new (Ld16Target::A16, Ld16Target::SP), 20;
        0x09 => AddHl::new(ArithmeticTarget16Bit::BC), 8;
        0x0A => Ld::new (LdTarget::A, LdTarget::BCAddr), 8;
        0x0B => Dec16::new(ArithmeticTarget16Bit::BC), 8;
        0x0C => Inc::new(ArithmeticTarget8Bit::C), 4;
        0x0D => Dec::new(ArithmeticTarget8Bit::C), 4;
        0x0E => Ld::new (LdTarget::C, LdTarget::D8), 8;

        0x11 => Ld16::new (Ld16Target::DE, Ld16Target::D16), 12;
        0x12 => Ld::new (LdTarget::DEAddr, LdTarget::A), 8;
        0x13 => Inc16::new(ArithmeticTarget16Bit::DE), 8;
        0x14 => Inc::new(ArithmeticTarget8Bit::D), 4;
        0x15 => Dec::new(ArithmeticTarget8Bit::D), 4;
        0x16 => Ld::new (LdTarget::D, LdTarget::D8), 8;

        0x19 => AddHl::new(ArithmeticTarget16Bit::DE), 8;
        0x1A => Ld::new (LdTarget::A, LdTarget::DEAddr), 8;
        0x1B => Dec16::new(ArithmeticTarget16Bit::DE), 8;
        0x1C => Inc::new(ArithmeticTarget8Bit::E), 4;
        0x1D => Dec::new(ArithmeticTarget8Bit::E), 4;
        0x1E => Ld::new (LdTarget::E, LdTarget::D8), 8;

        0x21 => Ld16::new (Ld16Target::HL, Ld16Target::D16), 12;
        0x22 => Ld::new (LdTarget::HLIAddr, LdTarget::A), 8;
        0x23 => Inc16::new(ArithmeticTarget16Bit::HL), 8;
        0x24 => Inc::new(ArithmeticTarget8Bit::H), 4;
        0x25 => Dec::new(ArithmeticTarget8Bit::H), 4;
        0x26 => Ld::new (LdTarget::H, LdTarget::D8), 8;
        0x27 => Daa, 4;

        0x29 => AddHl::new(ArithmeticTarget16Bit::HL), 8;
        0x2A => Ld::new (LdTarget::A, LdTarget::HLIAddr), 8;
        0x2B => Dec16::new(ArithmeticTarget16Bit::HL), 8;
        0x2C => Inc::new(ArithmeticTarget8Bit::L), 4;
        0x2D => Dec::new(ArithmeticTarget8Bit::L), 4;
        0x2E => Ld::new (LdTarget::L, LdTarget::D8), 8;
        0x2F => Cpl, 4;

        0x31 => Ld16::new (Ld16Target::SP, Ld16Target::D16), 12;
        0x32 => Ld::new (LdTarget::HLDAddr, LdTarget::A), 8;
        0x33 => Inc16::new(ArithmeticTarget16Bit::SP), 8;
        0x34 => Inc::new(ArithmeticTarget8Bit::HLAddr), 12;
        0x35 => Dec::new(ArithmeticTarget8Bit::HLAddr), 12;
        0x36 => Ld::new (LdTarget::HLAddr, LdTarget::D8), 12;
        0x37 => Scf, 4;

        0x39 => AddHl::new(ArithmeticTarget16Bit::SP), 8;
        0x3A => Ld::new (LdTarget::A, LdTarget::HLIAddr), 8;
        0x3B => Dec16::new(ArithmeticTarget16Bit::SP), 8;
        0x3C => Inc::new(ArithmeticTarget8Bit::A), 4;
        0x3D => Dec::new(ArithmeticTarget8Bit::A), 4;
        0x3E => Ld::new (LdTarget::A, LdTarget::D8), 8;
        0x3F => Ccf, 4;

        0x40 => Ld::new (LdTarget::B, LdTarget::B), 4;
        0x41 => Ld::new (LdTarget::B, LdTarget::C), 4;
        0x42 => Ld::new (LdTarget::B, LdTarget::D), 4;
        0x43 => Ld::new (LdTarget::B, LdTarget::E), 4;
        0x44 => Ld::new (LdTarget::B, LdTarget::H), 4;
        0x45 => Ld::new (LdTarget::B, LdTarget::L), 4;
        0x46 => Ld::new (LdTarget::B, LdTarget::HLAddr), 8;
        0x47 => Ld::new (LdTarget::B, LdTarget::A), 4;
        0x48 => Ld::new (LdTarget::C, LdTarget::B), 4;
        0x49 => Ld::new (LdTarget::C, LdTarget::C), 4;
        0x4A => Ld::new (LdTarget::C, LdTarget::D), 4;
        0x4B => Ld::new (LdTarget::C, LdTarget::E), 4;
        0x4C => Ld::new (LdTarget::C, LdTarget::H), 4;
        0x4D => Ld::new (LdTarget::C, LdTarget::L), 4;
        0x4E => Ld::new (LdTarget::C, LdTarget::HLAddr), 8;
        0x4F => Ld::new (LdTarget::C, LdTarget::A), 4;

        0x50 => Ld::new (LdTarget::D, LdTarget::B), 4;
        0x51 => Ld::new (LdTarget::D, LdTarget::C), 4;
        0x52 => Ld::new (LdTarget::D, LdTarget::D), 4;
        0x53 => Ld::new (LdTarget::D, LdTarget::E), 4;
        0x54 => Ld::new (LdTarget::D, LdTarget::H), 4;
        0x55 => Ld::new (LdTarget::D, LdTarget::L), 4;
        0x56 => Ld::new (LdTarget::D, LdTarget::HLAddr), 8;
        0x57 => Ld::new (LdTarget::D, LdTarget::A), 4;
        0x58 => Ld::new (LdTarget::E, LdTarget::B), 4;
        0x59 => Ld::new (LdTarget::E, LdTarget::C), 4;
        0x5A => Ld::new (LdTarget::E, LdTarget::D), 4;
        0x5B => Ld::new (LdTarget::E, LdTarget::E), 4;
        0x5C => Ld::new (LdTarget::E, LdTarget::H), 4;
        0x5D => Ld::new (LdTarget::E, LdTarget::L), 4;
        0x5E => Ld::new (LdTarget::E, LdTarget::HLAddr), 8;
        0x5F => Ld::new (LdTarget::E, LdTarget::A), 4;

        0x60 => Ld::new (LdTarget::H, LdTarget::B), 4;
        0x61 => Ld::new (LdTarget::H, LdTarget::C), 4;
        0x62 => Ld::new (LdTarget::H, LdTarget::D), 4;
        0x63 => Ld::new (LdTarget::H, LdTarget::E), 4;
        0x64 => Ld::new (LdTarget::H, LdTarget::H), 4;
        0x65 => Ld::new (LdTarget::H, LdTarget::L), 4;
        0x66 => Ld::new (LdTarget::H, LdTarget::HLAddr), 8;
        0x67 => Ld::new (LdTarget::H, LdTarget::A), 4;
        0x68 => Ld::new (LdTarget::L, LdTarget::B), 4;
        0x69 => Ld::new (LdTarget::L, LdTarget::C), 4;
        0x6A => Ld::new (LdTarget::L, LdTarget::D), 4;
        0x6B => Ld::new (LdTarget::L, LdTarget::E), 4;
        0x6C => Ld::new (LdTarget::L, LdTarget::H), 4;
        0x6D => Ld::new (LdTarget::L, LdTarget::L), 4;
        0x6E => Ld::new (LdTarget::L, LdTarget::HLAddr), 8;
        0x6F => Ld::new (LdTarget::L, LdTarget::A), 4;

        0x70 => Ld::new (LdTarget::HLAddr, LdTarget::B), 8;
        0x71 => Ld::new (LdTarget::HLAddr, LdTarget::C), 8;
        0x72 => Ld::new (LdTarget::HLAddr, LdTarget::D), 8;
        0x73 => Ld::new (LdTarget::HLAddr, LdTarget::E), 8;
        0x74 => Ld::new (LdTarget::HLAddr, LdTarget::H), 8;
        0x75 => Ld::new (LdTarget::HLAddr, LdTarget::L), 8;

        0x77 => Ld::new (LdTarget::HLAddr, LdTarget::A), 8;
        0x78 => Ld::new (LdTarget::A, LdTarget::B), 4;
        0x79 => Ld::new (LdTarget::A, LdTarget::C), 4;
        0x7A => Ld::new (LdTarget::A, LdTarget::D), 4;
        0x7B => Ld::new (LdTarget::A, LdTarget::E), 4;
        0x7C => Ld::new (LdTarget::A, LdTarget::H), 4;
        0x7D => Ld::new (LdTarget::A, LdTarget::L), 4;
        0x7E => Ld::new (LdTarget::A, LdTarget::HLAddr), 8;
        0x7F => Ld::new (LdTarget::A, LdTarget::A), 4;

        0x80 => Add::new(ArithmeticTarget8Bit::B), 4;
        0x81 => Add::new(ArithmeticTarget8Bit::C), 4;
        0x82 => Add::new(ArithmeticTarget8Bit::D), 4;
        0x83 => Add::new(ArithmeticTarget8Bit::E), 4;
        0x84 => Add::new(ArithmeticTarget8Bit::H), 4;
        0x85 => Add::new(ArithmeticTarget8Bit::L), 4;
        0x86 => Add::new(ArithmeticTarget8Bit::HLAddr), 8;
        0x87 => Add::new(ArithmeticTarget8Bit::A), 4;

        0x88 => Adc::new(ArithmeticTarget8Bit::B), 4;
        0x89 => Adc::new(ArithmeticTarget8Bit::C), 4;
        0x8A => Adc::new(ArithmeticTarget8Bit::D), 4;
        0x8B => Adc::new(ArithmeticTarget8Bit::E), 4;
        0x8C => Adc::new(ArithmeticTarget8Bit::H), 4;
        0x8D => Adc::new(ArithmeticTarget8Bit::L), 4;
        0x8E => Adc::new(ArithmeticTarget8Bit::HLAddr), 8;
        0x8F => Adc::new(ArithmeticTarget8Bit::A), 4;

        0x90 => Sub::new(ArithmeticTarget8Bit::B), 4;
        0x91 => Sub::new(ArithmeticTarget8Bit::C), 4;
        0x92 => Sub::new(ArithmeticTarget8Bit::D), 4;
        0x93 => Sub::new(ArithmeticTarget8Bit::E), 4;
        0x94 => Sub::new(ArithmeticTarget8Bit::H), 4;
        0x95 => Sub::new(ArithmeticTarget8Bit::L), 4;
        0x96 => Sub::new(ArithmeticTarget8Bit::HLAddr), 8;
        0x97 => Sub::new(ArithmeticTarget8Bit::A), 4;

        0x98 => Sbc::new(ArithmeticTarget8Bit::B), 4;
        0x99 => Sbc::new(ArithmeticTarget8Bit::C), 4;
        0x9A => Sbc::new(ArithmeticTarget8Bit::D), 4;
        0x9B => Sbc::new(ArithmeticTarget8Bit::E), 4;
        0x9C => Sbc::new(ArithmeticTarget8Bit::H), 4;
        0x9D => Sbc::new(ArithmeticTarget8Bit::L), 4;
        0x9E => Sbc::new(ArithmeticTarget8Bit::HLAddr), 8;
        0x9F => Sbc::new(ArithmeticTarget8Bit::A), 4;

        0xA0 => And::new(ArithmeticTarget8Bit::B), 4;
        0xA1 => And::new(ArithmeticTarget8Bit::C), 4;
        0xA2 => And::new(ArithmeticTarget8Bit::D), 4;
        0xA3 => And::new(ArithmeticTarget8Bit::E), 4;
        0xA4 => And::new(ArithmeticTarget8Bit::H), 4;
        0xA5 => And::new(ArithmeticTarget8Bit::L), 4;
        0xA6 => And::new(ArithmeticTarget8Bit::HLAddr), 8;
        0xA7 => And::new(ArithmeticTarget8Bit::A), 4;

        0xA8 => Xor::new(ArithmeticTarget8Bit::B), 4;
        0xA9 => Xor::new(ArithmeticTarget8Bit::C), 4;
        0xAA => Xor::new(ArithmeticTarget8Bit::D), 4;
        0xAB => Xor::new(ArithmeticTarget8Bit::E), 4;
        0xAC => Xor::new(ArithmeticTarget8Bit::H), 4;
        0xAD => Xor::new(ArithmeticTarget8Bit::L), 4;
        0xAE => Xor::new(ArithmeticTarget8Bit::HLAddr), 8;
        0xAF => Xor::new(ArithmeticTarget8Bit::A), 4;

        0xB0 => Or::new(ArithmeticTarget8Bit::B), 4;
        0xB1 => Or::new(ArithmeticTarget8Bit::C), 4;
        0xB2 => Or::new(ArithmeticTarget8Bit::D), 4;
        0xB3 => Or::new(ArithmeticTarget8Bit::E), 4;
        0xB4 => Or::new(ArithmeticTarget8Bit::H), 4;
        0xB5 => Or::new(ArithmeticTarget8Bit::L), 4;
        0xB6 => Or::new(ArithmeticTarget8Bit::HLAddr), 8;
        0xB7 => Or::new(ArithmeticTarget8Bit::A), 4;

        0xB8 => Cp::new(ArithmeticTarget8Bit::B), 4;
        0xB9 => Cp::new(ArithmeticTarget8Bit::C), 4;
        0xBA => Cp::new(ArithmeticTarget8Bit::D), 4;
        0xBB => Cp::new(ArithmeticTarget8Bit::E), 4;
        0xBC => Cp::new(ArithmeticTarget8Bit::H), 4;
        0xBD => Cp::new(ArithmeticTarget8Bit::L), 4;
        0xBE => Cp::new(ArithmeticTarget8Bit::HLAddr), 8;
        0xBF => Cp::new(ArithmeticTarget8Bit::A), 4;

        0xC1 => Pop::new(PushPopTarget::BC), 12;

        0xC5 => Push::new(PushPopTarget::BC), 16;
        0xC6 => Add::new(ArithmeticTarget8Bit::D8), 8;

        0xCE => Adc::new(ArithmeticTarget8Bit::D8), 8;

        0xD1 => Pop::new(PushPopTarget::DE), 12;

        0xD5 => Push::new(PushPopTarget::DE), 16;
        0xD6 => Sub::new(ArithmeticTarget8Bit::D8), 8;

        0xDE => Sbc::new(ArithmeticTarget8Bit::D8), 8;

        0xE0 => Ld::new (LdTarget::A8, LdTarget::A), 12;
        0xE1 => Pop::new(PushPopTarget::HL), 12;
        0xE2 => Ld::new (LdTarget::CAddr, LdTarget::A), 8;

        0xE5 => Push::new(PushPopTarget::HL), 16;
        0xE6 => And::new(ArithmeticTarget8Bit::D8), 8;

        0xE8 => AddSp, 16;

        0xEA => Ld::new (LdTarget::A, LdTarget::A16), 16;

        0xEE => Xor::new(ArithmeticTarget8Bit::D8), 8;

        0xF0 => Ld::new (LdTarget::A, LdTarget::A8), 12;
        0xF1 => Pop::new(PushPopTarget::AF), 12;
        0xF2 => Ld::new (LdTarget::A, LdTarget::CAddr), 8;

        0xF5 => Push::new(PushPopTarget::AF), 16;
        0xF6 => Or::new(ArithmeticTarget8Bit::D8), 8;

        0xF9 => Ld16::new (Ld16Target::SP, Ld16Target::HL), 8;
        0xFA => Ld::new (LdTarget::A16, LdTarget::A), 16;

        0xFE => Cp::new(ArithmeticTarget8Bit::D8), 8;
    })
}
