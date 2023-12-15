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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DataRate {
    pub(crate) mantissa: u8,
    pub(crate) exponent: u8,
    pub(crate) data_rate_hz: u64,
}
impl DataRate {
    // TODO: Not defined for all values, need to figure out.
    pub(crate) const fn new(data_rate_hz: u64) -> Self {
        let exponent = 64 - (data_rate_hz.rotate_left(19) / FXOSC).leading_zeros();
        let mantissa = (data_rate_hz.rotate_left(27) / (FXOSC.rotate_left(exponent - 1))) - 255;
        // When mantissa is 256, wrap to zero and increase exponent by one
        if mantissa == 256 {
            Self {
                mantissa: 0u8,
                exponent: (exponent + 1) as u8,
                data_rate_hz,
            }
        } else {
            Self {
                mantissa: mantissa as u8,
                exponent: exponent as u8,
                data_rate_hz,
            }
        }
    }
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

    #[test]
    fn test_drate() {
        // Some sample settings from SmartRF Studio
        assert_eq!(
            DataRate {
                mantissa: 117,
                exponent: 5,
                data_rate_hz: 1156
            },
            DataRate::new(1156)
        );
        assert_eq!(
            DataRate {
                mantissa: 117,
                exponent: 7,
                data_rate_hz: 4_624
            },
            DataRate::new(4_624)
        );
        assert_eq!(
            DataRate {
                mantissa: 117,
                exponent: 10,
                data_rate_hz: 36_994
            },
            DataRate::new(36_994)
        );
        assert_eq!(
            DataRate {
                mantissa: 34,
                exponent: 12,
                data_rate_hz: 115_051,
            },
            DataRate::new(115_051)
        );
        assert_eq!(
            DataRate {
                mantissa: 59,
                exponent: 14,
                data_rate_hz: 499_877
            },
            DataRate::new(499_877)
        );
        assert_eq!(
            DataRate {
                mantissa: 59,
                exponent: 13,
                data_rate_hz: 249_938
            },
            DataRate::new(249_938)
        );
        assert_eq!(
            DataRate {
                mantissa: 248,
                exponent: 11,
                data_rate_hz: 99_975
            },
            DataRate::new(99_975)
        );
        assert_eq!(
            DataRate {
                mantissa: 131,
                exponent: 11,
                data_rate_hz: 76_766
            },
            DataRate::new(76_766)
        );
        assert_eq!(
            DataRate {
                mantissa: 131,
                exponent: 10,
                data_rate_hz: 38_383
            },
            DataRate::new(38_383)
        );
        assert_eq!(
            DataRate {
                mantissa: 147,
                exponent: 8,
                data_rate_hz: 9_992
            },
            DataRate::new(9_992)
        );
        assert_eq!(
            DataRate {
                mantissa: 131,
                exponent: 7,
                data_rate_hz: 4_797
            },
            DataRate::new(4_797)
        );
        assert_eq!(
            DataRate {
                mantissa: 131,
                exponent: 6,
                data_rate_hz: 2_398
            },
            DataRate::new(2_398)
        );
        assert_eq!(
            DataRate {
                mantissa: 131,
                exponent: 5,
                data_rate_hz: 1_199
            },
            DataRate::new(1_199)
        );

        /* TODO: make this work
        fn calc_drate_rev(mantissa: u8, exponent: u8) -> u64 {
            let q = (256.0 + mantissa as f64) * 2f64.powf(exponent as f64);
            let p = 2f64.powf(28.0);
            ((q / p) * FXOSC as f64).floor() as u64
        }
        for e in 0..255 {
            for m in 0..255 {
                let baud = calc_drate_rev(m, e);
                let (mp, ep) = from_drate(baud);
                assert_eq!((mp, ep), (m as u64, e as u64));
            }
        }
        */
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
