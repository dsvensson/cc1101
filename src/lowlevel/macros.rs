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

        impl $REGISTER<::lowlevel::traits::Mask> {
            #[allow(dead_code)]
            pub fn mask() -> $REGISTER<::lowlevel::traits::Mask> {
                $REGISTER { bits: 0, _mode: ::core::marker::PhantomData }
            }

            $(
                #[allow(dead_code)]
                pub fn $bitfield(&self) -> $uxx {
                    use lowlevel::traits::OffsetSize;

                    let size = $range.size() + 1;
                    let offset = $range.offset();
                    (((1 << size) - 1) as u8) << offset
                }
            )+
        }

        impl ::core::default::Default for $REGISTER<::lowlevel::traits::W> {
            fn default() -> Self {
                $REGISTER { bits: $reset_value, _mode: ::core::marker::PhantomData }
            }
        }

        #[allow(non_snake_case)]
        #[allow(dead_code)]
        pub fn $REGISTER(bits: $uxx) -> $REGISTER<::lowlevel::traits::R> {
            $REGISTER { bits, _mode: ::core::marker::PhantomData }
        }

        impl $REGISTER<::lowlevel::traits::R> {
            #[allow(dead_code)]
            pub fn modify(self) -> $REGISTER<::lowlevel::traits::W> {
                $REGISTER { bits: self.bits, _mode: ::core::marker::PhantomData }
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub fn $bitfield(&self) -> $uxx {
                    use lowlevel::traits::OffsetSize;

                    let offset = $range.offset();
                    let size = $range.size() + 1;
                    let mask = ((1 << size) - 1) as u8;

                    (self.bits >> offset) & mask
                }
            )+
        }

        impl $REGISTER<::lowlevel::traits::W> {
            #[allow(dead_code)]
            pub fn bits(self) -> $uxx {
                self.bits
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub fn $bitfield(&mut self, mut bits: $uxx) -> &mut Self {
                    use lowlevel::traits::OffsetSize;

                    let offset = $range.offset();
                    let size = $range.size() + 1;
                    let mask = ((1 << size) - 1) as u8;

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
