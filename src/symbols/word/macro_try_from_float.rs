macro_rules! impl_try_from_float {
    ($($T: ty),+) => {
        $(
            impl<const N: usize> core::convert::TryFrom<$T> for Word<N> {
                type Error = CannotConvertNaN;

                fn try_from(value: $T) -> Result<Self, Self::Error> {
                    if value.is_nan() {
                        return Err(CannotConvertNaN);
                    }
                    let num = (value as isize).unsigned_abs();
                    let mut arr = Word::<N>::from(num).into_symbol_array();
                    let mut space_left = arr.iter().filter(|s| **s == misc::EMPTY).count();
                    if value < 0.0 {
                        space_left = space_left.saturating_sub(1);
                    }
                    arr.rotate_left(space_left);
                    if value < 0.0 {
                        arr[0] = misc::MINUS;
                    }
                    if space_left > 0 {
                        // add dot to last integral digit
                        if let Some(s) = arr.get_mut(N - space_left - 1) {
                            *s = s.with_dot();
                        }

                        // get fractional part digits
                        let mut num = if value < 0.0 { value.neg() } else { value };
                        num = num - (num as usize as $T);
                        for i in N-space_left..N {
                            num *= 10.0;
                            let d = num as u8;
                            arr[i] = Symbol::from_u8(d).expect("d cannot be greater than 9 because of MOD");
                            num -= d as $T;
                        }
                    }
                    Ok(Self(arr))
                }
            }
        )+
    };
}
