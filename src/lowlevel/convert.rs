use crate::lowlevel::FXOSC;

pub const fn from_frequency(hz: u64) -> (u8, u8, u8) {
    let freq = hz * 1u64.rotate_left(16) / FXOSC;
    let freq0 = (freq & 0xff) as u8;
    let freq1 = ((freq >> 8) & 0xff) as u8;
    let freq2 = ((freq >> 16) & 0xff) as u8;
    (freq0, freq1, freq2)
}

pub const fn from_deviation(v: u64) -> (u8, u8) {
    let exponent = 8 - ((v.rotate_left(14) / FXOSC) as u8).leading_zeros() - 1;
    let mantissa = (v.rotate_left(17) / (FXOSC.rotate_left(exponent))) - 7;
    ((mantissa & 0x7) as u8, (exponent & 0x7) as u8)
}

#[cfg(test)]
mod tests {
    use crate::lowlevel::convert::*;
    use crate::lowlevel::FXOSC;

    #[test]
    fn test_frequency() {
        assert_eq!(from_frequency(433_000_000), (0x62, 0xA7, 0x10));
        assert_eq!(from_frequency(868_000_000), (0x76, 0x62, 0x21));
        assert_eq!(from_frequency(902_000_000), (0x3B, 0xB1, 0x22));
        assert_eq!(from_frequency(918_000_000), (0xC4, 0x4E, 0x23));
    }

    #[test]
    fn test_deviation() {
        // f_dev = f_osc / 2^17 * (8 + DEVIATION_M) * 2^DEVIATION_E
        fn calc_rev_dev(dev_m: u8, dev_e: u8) -> u64 {
            (((FXOSC as f32 / (2u64.pow(17) as f32)) as f32)
                * (8f32 + dev_m as f32)
                * (2u64.pow(dev_e as u32) as f32)) as u64
        }

        for e in 0..7 {
            for m in 1..7 {
                assert_eq!(from_deviation(calc_rev_dev(m, e)), (m, e));
            }
        }
    }
}
