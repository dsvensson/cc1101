use crate::lowlevel::FXOSC;

pub const fn from_frequency(hz: u64) -> (u8, u8, u8) {
    let freq = hz * 1u64.rotate_left(16) / FXOSC;
    let freq0 = (freq & 0xff) as u8;
    let freq1 = ((freq >> 8) & 0xff) as u8;
    let freq2 = ((freq >> 16) & 0xff) as u8;
    (freq0, freq1, freq2)
}

pub const fn from_deviation(v: u64) -> (u8, u8) {
    let exponent = 64 - (v.rotate_left(14) / FXOSC).leading_zeros() - 1;
    let mantissa = (v.rotate_left(17) / (FXOSC.rotate_left(exponent))) - 7;
    ((mantissa & 0x7) as u8, (exponent & 0x7) as u8)
}

// TODO: Not defined for all values, need to figure out.
pub const fn from_drate(v: u64) -> (u8, u8) {
    let exponent = 64 - (v.rotate_left(19) / FXOSC).leading_zeros();
    let division = (v.rotate_left(27)) / (FXOSC.rotate_left(exponent - 1));
    let mantissa = if division >= 255 {
        division - 255
    } else {
        todo!()
    };
    // When mantissa is 256, wrap to zero and increase exponent by one
    [(mantissa as u8, exponent as u8), (0u8, (exponent + 1) as u8)][(mantissa == 256) as usize]
}

pub fn from_chanbw(v: u64) -> (u8, u8) {
    let exponent = 64 - (FXOSC / (8 * 4 * v)).leading_zeros() - 1;
    let mantissa = FXOSC / (v * 8 * 2u64.pow(exponent)) - 4;
    (mantissa as u8 & 0x3, exponent as u8 & 0x3)
}

#[cfg(test)]
mod tests {
    use crate::lowlevel::convert::*;
    use crate::lowlevel::FXOSC;

    #[test]
    fn test_frequency() {
        assert_eq!(from_frequency(0_000_000), (0x00, 0x00, 0x00)); // Maybe irrelevant
        assert_eq!(from_frequency(433_000_000), (0x62, 0xA7, 0x10));
        assert_eq!(from_frequency(868_000_000), (0x76, 0x62, 0x21));
        assert_eq!(from_frequency(902_000_000), (0x3B, 0xB1, 0x22));
        assert_eq!(from_frequency(918_000_000), (0xC4, 0x4E, 0x23));
        assert_eq!(from_frequency(1_000_000_000), (0x27, 0x76, 0x26)); // Maybe irrelevant
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

    #[test]
    fn test_drate() {
        // Some sample settings from SmartRF Studio
        assert_eq!((117, 5), from_drate(1156));
        assert_eq!((117, 7), from_drate(4624));
        assert_eq!((117, 10), from_drate(36994));
        assert_eq!((34, 12), from_drate(115051));
        assert_eq!((59, 14), from_drate(499877));
        assert_eq!((59, 13), from_drate(249938));
        assert_eq!((248, 11), from_drate(99975));
        assert_eq!((131, 11), from_drate(76766));
        assert_eq!((131, 10), from_drate(38383));
        assert_eq!((147, 8), from_drate(9992));
        assert_eq!((131, 7), from_drate(4797));
        assert_eq!((131, 6), from_drate(2398));
        assert_eq!((131, 5), from_drate(1199));

        // TODO: make this work
        fn calc_drate_rev(mantissa: u8, exponent: u8) -> u64 {
            let q = (256.0 + mantissa as f64) * 2f64.powf(exponent as f64);
            let p = 2f64.powf(28.0);
            ((q / p) * FXOSC as f64).floor() as u64
        }
        for e in 0..255 {
            for m in 0..255 {
                let baud = calc_drate_rev(m, e);
                let (mp, ep) = from_drate(baud);
                assert_eq!((mp, ep), (m, e));
            }
        }
    }

    #[test]
    fn test_chanbw() {
        assert_eq!(from_chanbw(812500), (0b00, 0b00));
        assert_eq!(from_chanbw(650000), (0b01, 0b00));
        assert_eq!(from_chanbw(541666), (0b10, 0b00));
        assert_eq!(from_chanbw(464285), (0b11, 0b00));
        assert_eq!(from_chanbw(406250), (0b00, 0b01));
        assert_eq!(from_chanbw(325000), (0b01, 0b01));
        assert_eq!(from_chanbw(270833), (0b10, 0b01));
        assert_eq!(from_chanbw(232142), (0b11, 0b01));
        assert_eq!(from_chanbw(203125), (0b00, 0b10));
        assert_eq!(from_chanbw(162000), (0b01, 0b10));
        assert_eq!(from_chanbw(135416), (0b10, 0b10));
        assert_eq!(from_chanbw(116071), (0b11, 0b10));
        assert_eq!(from_chanbw(101562), (0b00, 0b11));
        assert_eq!(from_chanbw(81250), (0b01, 0b11));
        assert_eq!(from_chanbw(67708), (0b10, 0b11));
        assert_eq!(from_chanbw(58035), (0b11, 0b11));
    }
}
