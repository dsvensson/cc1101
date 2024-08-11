macro_rules! register {
    ($REGISTER:ident, $reset_value:expr, $uxx:ty, {
        $(#[$($attr:tt)*] $bitfield:ident @ $range:expr,)+
    }) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy)]
        pub struct $REGISTER<MODE> {
            bits: $uxx,
            _mode: ::core::marker::PhantomData<MODE>,
        }

        impl $REGISTER<crate::lowlevel::traits::Mask> {
            pub fn mask() -> $REGISTER<crate::lowlevel::traits::Mask> {
                $REGISTER { bits: 0, _mode: ::core::marker::PhantomData }
            }

            $(
                pub fn $bitfield(&self) -> $uxx {
                    use crate::lowlevel::traits::{OffsetSize, ToWider};

                    let offset = $range.offset();
                    let size = $range.size();
                    (((1 << size.to_wider()) - 1)) << offset
                }
            )+
        }

        impl ::core::default::Default for $REGISTER<crate::lowlevel::traits::W> {
            fn default() -> Self {
                $REGISTER { bits: $reset_value, _mode: ::core::marker::PhantomData }
            }
        }

        #[allow(non_snake_case)]
        pub fn $REGISTER(bits: $uxx) -> $REGISTER<crate::lowlevel::traits::R> {
            $REGISTER { bits, _mode: ::core::marker::PhantomData }
        }

        impl $REGISTER<crate::lowlevel::traits::R> {
            pub fn modify(self) -> $REGISTER<crate::lowlevel::traits::W> {
                $REGISTER { bits: self.bits, _mode: ::core::marker::PhantomData }
            }

            $(
                #[$($attr)*]
                pub fn $bitfield(&self) -> $uxx {
                    use crate::lowlevel::traits::{OffsetSize, ToWider};

                    let offset = $range.offset();
                    let size = $range.size();
                    let mask = ((1 << size.to_wider()) - 1) as $uxx;

                    (self.bits >> offset) & mask
                }
            )+
        }

        impl $REGISTER<crate::lowlevel::traits::W> {
            pub fn bits(self) -> $uxx {
                self.bits
            }

            $(
                #[$($attr)*]
                pub fn $bitfield(&mut self, mut bits: $uxx) -> &mut Self {
                    use crate::lowlevel::traits::{OffsetSize, ToWider};

                    let offset = $range.offset();
                    let size = $range.size();
                    let mask = ((1 << size.to_wider()) - 1) as $uxx;

                    debug_assert!(bits <= mask);
                    bits &= mask;

                    self.bits &= !(mask << offset);
                    self.bits |= bits << offset;

                    self
                }
            )+
        }
    }
}

#[cfg(test)]
mod tests {

    fn bit_mask(bits: u8) -> u8 {
        (1 << bits) - 1
    }

    register!(REG_1, 0b0000_0000, u8, {
        #[doc = "Bit 7 (1 bit) - 0bx000_0000"]
        field_7 @ 7,
        #[doc = "Bit 6 (1 bit) - 0b0x00_0000"]
        field_6 @ 6,
        #[doc = "Bit 5 (1 bit) - 0b00x0_0000"]
        field_5 @ 5,
        #[doc = "Bit 4 (1 bit) - 0b000x_0000"]
        field_4 @ 4,
        #[doc = "Bit 3 (1 bit) - 0b0000_x000"]
        field_3 @ 3,
        #[doc = "Bit 2 (1 bit) - 0b0000_0x00"]
        field_2 @ 2,
        #[doc = "Bit 1 (1 bit) - 0b0000_00x0"]
        field_1 @ 1,
        #[doc = "Bit 0 (1 bit) - 0b0000_000x"]
        field_0 @ 0,
    });

    register!(REG_2, 0b0000_0000, u8, {
        #[doc = "Bit 1..8 (7 bits) - 0bxxxx_xxx0"]
        field_1 @ 1..8,
        #[doc = "Bit 0 (1 bit) - 0b0000_000x"]
        field_0 @ 0,
    });

    register!(REG_3, 0b0000_0000, u8, {
        #[doc = "Bit 6..7 (2 bits) - 0bxx00_0000"]
        field_1 @ 6..8,
        #[doc = "Bit 0..6 (6 bits) - 0b00xx_xxxx"]
        field_0 @ 0..6,
    });

    register!(REG_4, 0b0000_0000, u8, {
        #[doc = "Bit 5..8 (3 bits) - 0bxxx0_0000"]
        field_1 @ 5..8 ,
        #[doc = "Bit 0..5 (5 bits) - 0b000x_xxxx"]
        field_0 @ 0..5,
    });

    register!(REG_5, 0b0000_0000, u8, {
        #[doc = "Bit 4..8 (4 bits) - 0bxxxx_0000"]
        field_1 @ 4..8,
        #[doc = "Bit 0..4 (4 bits) - 0b0000_xxxx"]
        field_0 @ 0..4,
    });

    #[test]
    fn test_reg_read() {
        // REG 1
        assert_eq!(REG_1(u8::MAX).field_0(), bit_mask(1));
        assert_eq!(REG_1(u8::MAX).field_1(), bit_mask(1));
        assert_eq!(REG_1(u8::MAX).field_2(), bit_mask(1));
        assert_eq!(REG_1(u8::MAX).field_3(), bit_mask(1));
        assert_eq!(REG_1(u8::MAX).field_4(), bit_mask(1));
        assert_eq!(REG_1(u8::MAX).field_5(), bit_mask(1));
        assert_eq!(REG_1(u8::MAX).field_6(), bit_mask(1));
        assert_eq!(REG_1(u8::MAX).field_7(), bit_mask(1));

        // REG 2
        assert_eq!(REG_2(u8::MAX).field_0(), bit_mask(1));
        assert_eq!(REG_2(u8::MAX).field_1(), bit_mask(7));

        // REG 3
        assert_eq!(REG_3(u8::MAX).field_0(), bit_mask(6));
        assert_eq!(REG_3(u8::MAX).field_1(), bit_mask(2));

        // REG 4
        assert_eq!(REG_4(u8::MAX).field_0(), bit_mask(5));
        assert_eq!(REG_4(u8::MAX).field_1(), bit_mask(3));

        // REG 5
        assert_eq!(REG_5(u8::MAX).field_0(), bit_mask(4));
        assert_eq!(REG_5(u8::MAX).field_1(), bit_mask(4));
    }
    #[test]
    fn test_reg_write() {
        // REG 1
        assert_eq!(REG_1::default().field_0(1).bits(), 0b0000_0001);
        assert_eq!(REG_1::default().field_1(1).bits(), 0b0000_0010);
        assert_eq!(REG_1::default().field_2(1).bits(), 0b0000_0100);
        assert_eq!(REG_1::default().field_3(1).bits(), 0b0000_1000);
        assert_eq!(REG_1::default().field_4(1).bits(), 0b0001_0000);
        assert_eq!(REG_1::default().field_5(1).bits(), 0b0010_0000);
        assert_eq!(REG_1::default().field_6(1).bits(), 0b0100_0000);
        assert_eq!(REG_1::default().field_7(1).bits(), 0b1000_0000);

        // REG 2
        assert_eq!(REG_2::default().field_0(bit_mask(1)).bits(), 0b0000_0001);
        assert_eq!(REG_2::default().field_1(bit_mask(7)).bits(), 0b1111_1110);

        // REG 3
        assert_eq!(REG_3::default().field_0(bit_mask(6)).bits(), 0b0011_1111);
        assert_eq!(REG_3::default().field_1(bit_mask(2)).bits(), 0b1100_0000);

        // REG 4
        assert_eq!(REG_4::default().field_0(bit_mask(5)).bits(), 0b0001_1111);
        assert_eq!(REG_4::default().field_1(bit_mask(3)).bits(), 0b1110_0000);

        // REG 5
        assert_eq!(REG_5::default().field_0(bit_mask(4)).bits(), 0b0000_1111);
        assert_eq!(REG_5::default().field_1(bit_mask(4)).bits(), 0b1111_0000);
    }
}
