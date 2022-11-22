use super::adc::Adc;
use super::add::Add;
use super::operation::Operation;
use super::sbc::Sbc;
use super::sub::Sub;
use super::targets::ArithmeticTarget8Bit;

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

        0xC6 => Add::new(ArithmeticTarget8Bit::D8, 8),

        0xCE => Adc::new(ArithmeticTarget8Bit::D8, 8),

        0xD6 => Sub::new(ArithmeticTarget8Bit::D8, 8),

        0xDE => Sbc::new(ArithmeticTarget8Bit::D8, 8),
    })
}
