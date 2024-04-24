macro_rules! impl_from_int {
    ($($T: ty),+) => {
        $(
            impl<const N: usize> core::convert::From<$T> for Word<N> {
                fn from(value: $T) -> Self {
                    let num = value.unsigned_abs();
                    let mut arr = Word::from(num).into_symbol_array();
                    if value < 0 {
                        if arr[0] != misc::EMPTY {
                            arr = [digits::NINE; N];
                        }
                        arr[0] = misc::MINUS;
                    }
                    Self(arr)
                }
            }
        )+
    };
}
