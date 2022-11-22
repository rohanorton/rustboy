use super::add::Add;
use super::operation::Operation;
use super::targets::Target;

pub fn lookup_op_code(op_code: u8) -> Box<dyn Operation> {
    Box::new(match op_code {
        0x80 => Add::new(Target::B, 4),
        0x81 => Add::new(Target::C, 4),
        0x82 => Add::new(Target::D, 4),
        0x83 => Add::new(Target::E, 4),
        0x84 => Add::new(Target::H, 4),
        0x85 => Add::new(Target::L, 4),
        0x86 => Add::new(Target::HLAddr, 8),
        0x87 => Add::new(Target::A, 4),

        0xC6 => Add::new(Target::D8, 8),

        _ => panic!("Unimplemented Op Code {:#02x}", op_code),
    })
}
