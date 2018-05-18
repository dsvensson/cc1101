macro_rules! register {
    ($REGISTER:ident, $reset_value:expr, $uxx:ty, {
        $(#[$($attr:tt)*] $bitfield:ident @ $range:expr,)+
    }) => {
        #[derive(Clone, Copy)]
        pub(crate) struct $REGISTER<MODE> {
            bits: $uxx,
            _mode: ::core::marker::PhantomData<MODE>,
        }

        impl $REGISTER<::traits::Mask> {
            #[allow(dead_code)]
            pub(crate) fn mask() -> $REGISTER<::traits::Mask> {
                $REGISTER { bits: 0, _mode: ::core::marker::PhantomData }
            }

            $(
                #[allow(dead_code)]
                pub(crate) fn $bitfield(&self) -> $uxx {
                    use ::traits::OffsetSize;

                    let size = $range.size() + 1;
                    let offset = $range.offset();
                    (((1 << size) - 1) as u8) << offset
                }
            )+
        }

        impl ::core::default::Default for $REGISTER<::traits::W> {
            fn default() -> Self {
                $REGISTER { bits: $reset_value, _mode: ::core::marker::PhantomData }
            }
        }

        #[allow(non_snake_case)]
        #[allow(dead_code)]
        pub(crate) fn $REGISTER(bits: $uxx) -> $REGISTER<::traits::R> {
            $REGISTER { bits, _mode: ::core::marker::PhantomData }
        }

        impl $REGISTER<::traits::R> {
            #[allow(dead_code)]
            pub(crate) fn modify(self) -> $REGISTER<::traits::W> {
                $REGISTER { bits: self.bits, _mode: ::core::marker::PhantomData }
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub(crate) fn $bitfield(&self) -> $uxx {
                    use ::traits::OffsetSize;

                    let offset = $range.offset();
                    let size = $range.size() + 1;
                    let mask = ((1 << size) - 1) as u8;

                    (self.bits >> offset) & mask
                }
            )+
        }

        impl $REGISTER<::traits::W> {
            #[allow(dead_code)]
            pub(crate) fn bits(self) -> $uxx {
                self.bits
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub(crate) fn $bitfield(&mut self, mut bits: $uxx) -> &mut Self {
                    use ::traits::OffsetSize;

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
