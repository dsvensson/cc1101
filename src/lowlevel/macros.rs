macro_rules! register {
    ($PARENT:ident, $REGISTER:ident, $reset_value:expr, $uxx:ty, {
        $(#[$($attr:tt)*] $bitfield:ident @ $range:expr,)+
    }) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy)]
        pub struct $REGISTER<MODE=::lowlevel::traits::R> {
            bits: u8,
            _mode: ::core::marker::PhantomData<MODE>,
        }

        impl ::core::default::Default for $REGISTER<::lowlevel::traits::W> {
            fn default() -> Self {
                $REGISTER { bits: $reset_value, _mode: ::core::marker::PhantomData }
            }
        }

        #[allow(non_snake_case)]
        pub fn $REGISTER(bits: $uxx) -> $REGISTER<::lowlevel::traits::R> {
            $REGISTER { bits, _mode: ::core::marker::PhantomData }
        }

        impl ::lowlevel::registers::RegisterClass for $REGISTER<::lowlevel::traits::R> {
            const REGISTER_CLASS: ::lowlevel::registers::Register = ::lowlevel::registers::Register::$PARENT($PARENT::$REGISTER);
        }

        impl $REGISTER<::lowlevel::traits::R> {

            pub fn modify(self) -> $REGISTER<::lowlevel::traits::W> {
                $REGISTER { bits: self.bits, _mode: ::core::marker::PhantomData }
            }

            $(
                #[$($attr)*]
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
            pub fn bits(self) -> $uxx {
                self.bits
            }

            $(
                #[$($attr)*]
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

        impl Into<::lowlevel::registers::Register> for $REGISTER<::lowlevel::traits::R> {
            fn into(self) -> ::lowlevel::registers::Register {
                ::lowlevel::registers::Register::$PARENT($PARENT::$REGISTER)
            }
        }

        impl Into<u8> for $REGISTER<::lowlevel::traits::R> {
            fn into(self) -> u8{
                self.bits
            }
        }

        impl From<u8> for $REGISTER<::lowlevel::traits::R> {
            fn from(val: u8) -> Self {
                $REGISTER { bits: val, _mode: ::core::marker::PhantomData }
            }
        }
    }
}
