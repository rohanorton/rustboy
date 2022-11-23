use super::operation::Operation;
use super::targets::{ArithmeticTarget16Bit, ArithmeticTarget8Bit};

use super::adc::Adc;
use super::add::Add;
use super::add_hl::AddHl;
use super::and::And;
use super::ccf::Ccf;
use super::cp::Cp;
use super::cpl::Cpl;
use super::daa::Daa;
use super::dec::Dec;
use super::dec_16::Dec16;
use super::inc::Inc;
use super::inc_16::Inc16;
use super::nop::Nop;
use super::or::Or;
use super::sbc::Sbc;
use super::scf::Scf;
use super::sub::Sub;
use super::xor::Xor;

// Macro to simplify op-code match creation. Wraps result in Box, to prevent type error.
macro_rules! boxed_operation {(
    $op_code:expr, { $($lhs:expr => $rhs:expr,)+ }
) => (
    match $op_code {
        $($lhs => ::std::boxed::Box::new($rhs),)+
        _ => panic!("Unimplemented Op Code {:#02x}", $op_code),
    }
)}

pub fn lookup_op_code(op_code: u8) -> Box<dyn Operation> {
    boxed_operation!(op_code, {
        0x00 => Nop::new(4),

        0x03 => Inc16::new(ArithmeticTarget16Bit::BC, 8),
        0x04 => Inc::new(ArithmeticTarget8Bit::B, 4),
        0x05 => Dec::new(ArithmeticTarget8Bit::B, 4),

        0x09 => AddHl::new(ArithmeticTarget16Bit::BC, 8),

        0x0B => Dec16::new(ArithmeticTarget16Bit::BC, 8),
        0x0C => Inc::new(ArithmeticTarget8Bit::C, 4),
        0x0D => Dec::new(ArithmeticTarget8Bit::C, 4),

        0x13 => Inc16::new(ArithmeticTarget16Bit::DE, 8),
        0x14 => Inc::new(ArithmeticTarget8Bit::D, 4),
        0x15 => Dec::new(ArithmeticTarget8Bit::D, 4),

        0x19 => AddHl::new(ArithmeticTarget16Bit::DE, 8),

        0x1B => Dec16::new(ArithmeticTarget16Bit::DE, 8),
        0x1C => Inc::new(ArithmeticTarget8Bit::E, 4),
        0x1D => Dec::new(ArithmeticTarget8Bit::E, 4),

        0x23 => Inc16::new(ArithmeticTarget16Bit::HL, 8),
        0x24 => Inc::new(ArithmeticTarget8Bit::H, 4),
        0x25 => Dec::new(ArithmeticTarget8Bit::H, 4),

        0x27 => Daa::new(4),

        0x29 => AddHl::new(ArithmeticTarget16Bit::HL, 8),

        0x2B => Dec16::new(ArithmeticTarget16Bit::HL, 8),
        0x2C => Inc::new(ArithmeticTarget8Bit::L, 4),
        0x2D => Dec::new(ArithmeticTarget8Bit::L, 4),

        0x2F => Cpl::new(4),

        0x33 => Inc16::new(ArithmeticTarget16Bit::SP, 8),
        0x34 => Inc::new(ArithmeticTarget8Bit::HLAddr, 12),
        0x35 => Dec::new(ArithmeticTarget8Bit::HLAddr, 12),

        0x37 => Scf::new(4),

        0x39 => AddHl::new(ArithmeticTarget16Bit::SP, 8),

        0x3B => Dec16::new(ArithmeticTarget16Bit::SP, 8),
        0x3C => Inc::new(ArithmeticTarget8Bit::A, 4),
        0x3D => Dec::new(ArithmeticTarget8Bit::A, 4),

        0x3F => Ccf::new(4),

        0x80 => Add::new(ArithmeticTarget8Bit::B, 4),
        0x81 => Add::new(ArithmeticTarget8Bit::C, 4),
        0x82 => Add::new(ArithmeticTarget8Bit::D, 4),
        0x83 => Add::new(ArithmeticTarget8Bit::E, 4),
        0x84 => Add::new(ArithmeticTarget8Bit::H, 4),
        0x85 => Add::new(ArithmeticTarget8Bit::L, 4),
        0x86 => Add::new(ArithmeticTarget8Bit::HLAddr, 8),
        0x87 => Add::new(ArithmeticTarget8Bit::A, 4),

        0x88 => Adc::new(ArithmeticTarget8Bit::B, 4),
        0x89 => Adc::new(ArithmeticTarget8Bit::C, 4),
        0x8A => Adc::new(ArithmeticTarget8Bit::D, 4),
        0x8B => Adc::new(ArithmeticTarget8Bit::E, 4),
        0x8C => Adc::new(ArithmeticTarget8Bit::H, 4),
        0x8D => Adc::new(ArithmeticTarget8Bit::L, 4),
        0x8E => Adc::new(ArithmeticTarget8Bit::HLAddr, 8),
        0x8F => Adc::new(ArithmeticTarget8Bit::A, 4),

        0x90 => Sub::new(ArithmeticTarget8Bit::B, 4),
        0x91 => Sub::new(ArithmeticTarget8Bit::C, 4),
        0x92 => Sub::new(ArithmeticTarget8Bit::D, 4),
        0x93 => Sub::new(ArithmeticTarget8Bit::E, 4),
        0x94 => Sub::new(ArithmeticTarget8Bit::H, 4),
        0x95 => Sub::new(ArithmeticTarget8Bit::L, 4),
        0x96 => Sub::new(ArithmeticTarget8Bit::HLAddr, 8),
        0x97 => Sub::new(ArithmeticTarget8Bit::A, 4),

        0x98 => Sbc::new(ArithmeticTarget8Bit::B, 4),
        0x99 => Sbc::new(ArithmeticTarget8Bit::C, 4),
        0x9A => Sbc::new(ArithmeticTarget8Bit::D, 4),
        0x9B => Sbc::new(ArithmeticTarget8Bit::E, 4),
        0x9C => Sbc::new(ArithmeticTarget8Bit::H, 4),
        0x9D => Sbc::new(ArithmeticTarget8Bit::L, 4),
        0x9E => Sbc::new(ArithmeticTarget8Bit::HLAddr, 8),
        0x9F => Sbc::new(ArithmeticTarget8Bit::A, 4),

        0xA0 => And::new(ArithmeticTarget8Bit::B, 4),
        0xA1 => And::new(ArithmeticTarget8Bit::C, 4),
        0xA2 => And::new(ArithmeticTarget8Bit::D, 4),
        0xA3 => And::new(ArithmeticTarget8Bit::E, 4),
        0xA4 => And::new(ArithmeticTarget8Bit::H, 4),
        0xA5 => And::new(ArithmeticTarget8Bit::L, 4),
        0xA6 => And::new(ArithmeticTarget8Bit::HLAddr, 8),
        0xA7 => And::new(ArithmeticTarget8Bit::A, 4),

        0xA8 => Xor::new(ArithmeticTarget8Bit::B, 4),
        0xA9 => Xor::new(ArithmeticTarget8Bit::C, 4),
        0xAA => Xor::new(ArithmeticTarget8Bit::D, 4),
        0xAB => Xor::new(ArithmeticTarget8Bit::E, 4),
        0xAC => Xor::new(ArithmeticTarget8Bit::H, 4),
        0xAD => Xor::new(ArithmeticTarget8Bit::L, 4),
        0xAE => Xor::new(ArithmeticTarget8Bit::HLAddr, 8),
        0xAF => Xor::new(ArithmeticTarget8Bit::A, 4),

        0xB0 => Or::new(ArithmeticTarget8Bit::B, 4),
        0xB1 => Or::new(ArithmeticTarget8Bit::C, 4),
        0xB2 => Or::new(ArithmeticTarget8Bit::D, 4),
        0xB3 => Or::new(ArithmeticTarget8Bit::E, 4),
        0xB4 => Or::new(ArithmeticTarget8Bit::H, 4),
        0xB5 => Or::new(ArithmeticTarget8Bit::L, 4),
        0xB6 => Or::new(ArithmeticTarget8Bit::HLAddr, 8),
        0xB7 => Or::new(ArithmeticTarget8Bit::A, 4),

        0xB8 => Cp::new(ArithmeticTarget8Bit::B, 4),
        0xB9 => Cp::new(ArithmeticTarget8Bit::C, 4),
        0xBA => Cp::new(ArithmeticTarget8Bit::D, 4),
        0xBB => Cp::new(ArithmeticTarget8Bit::E, 4),
        0xBC => Cp::new(ArithmeticTarget8Bit::H, 4),
        0xBD => Cp::new(ArithmeticTarget8Bit::L, 4),
        0xBE => Cp::new(ArithmeticTarget8Bit::HLAddr, 8),
        0xBF => Cp::new(ArithmeticTarget8Bit::A, 4),

        0xC6 => Add::new(ArithmeticTarget8Bit::D8, 8),

        0xCE => Adc::new(ArithmeticTarget8Bit::D8, 8),

        0xD6 => Sub::new(ArithmeticTarget8Bit::D8, 8),

        0xDE => Sbc::new(ArithmeticTarget8Bit::D8, 8),

        0xE6 => And::new(ArithmeticTarget8Bit::D8, 8),

        0xEE => Xor::new(ArithmeticTarget8Bit::D8, 8),

        0xF6 => Or::new(ArithmeticTarget8Bit::D8, 8),

        0xFE => Cp::new(ArithmeticTarget8Bit::D8, 8),
    })
}
