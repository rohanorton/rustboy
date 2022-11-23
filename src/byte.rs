pub fn lower_nibble(a: u8) -> u8 {
    a & 0xF
}

pub fn upper_nibble(a: u8) -> u8 {
    (a & 0xF0) >> 4
}

#[cfg(test)]
mod test {
    use super::{lower_nibble, upper_nibble};

    #[test]
    fn lower_nibble_returns_lowest_four_bits() {
        let cases = [
            (0x9E, 0x0E),
            (0x47, 0x07),
            (0x90, 0x00),
            (0xD6, 0x06),
            (0x64, 0x04),
            (0xD6, 0x06),
            (0xA6, 0x06),
            (0xB3, 0x03),
            (0x5C, 0x0C),
            (0xBF, 0x0F),
            (0xDC, 0x0C),
            (0x4C, 0x0C),
            (0x05, 0x05),
            (0x82, 0x02),
            (0xA3, 0x03),
            (0x20, 0x00),
            (0x54, 0x04),
            (0xFB, 0x0B),
            (0x47, 0x07),
            (0x0D, 0x0D),
        ];
        for (input, output) in cases {
            assert_eq!(
                lower_nibble(input),
                output,
                "Lower nibble of {input:#02x} should be {output:#02x}"
            );
        }
    }

    #[test]
    fn upper_nibble_returns_highest_four_bits() {
        let cases = [
            (0x9E, 0x09),
            (0x47, 0x04),
            (0x90, 0x09),
            (0xD6, 0x0D),
            (0x64, 0x06),
            (0xD6, 0x0D),
            (0xA6, 0x0A),
            (0xB3, 0x0B),
            (0x5C, 0x05),
            (0xBF, 0x0B),
            (0xDC, 0x0D),
            (0x4C, 0x04),
            (0x05, 0x00),
            (0x82, 0x08),
            (0xA3, 0x0A),
            (0x20, 0x02),
            (0x54, 0x05),
            (0xFB, 0x0F),
            (0x47, 0x04),
            (0x0D, 0x00),
        ];
        for (input, output) in cases {
            assert_eq!(
                upper_nibble(input),
                output,
                "Upper nibble of {input:#02x} should be {output:#02x}"
            );
        }
    }
}
