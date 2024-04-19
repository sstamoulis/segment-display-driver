macro_rules! impl_try_from_float {
    ($($T: ty),+) => {
        #[derive(Debug)]
        pub struct CannotConvertNaN;

        $(

            impl<const N: usize> Word<N> {
                paste::paste!{
                    pub fn [<try_from_ $T>](num: $T) -> Result<Self, CannotConvertNaN> {
                        if num.is_nan() {
                            return Err(CannotConvertNaN);
                        }
                        let n = num as i32;
                        if n >= 0 && n / 10_i32.pow(N as u32) > 0 {
                            // if integral part is positive
                            // and has more digits than we can hold
                            // then fill all digits with 9
                            Ok(Self([digits::NINE; N]))
                        } else if n < 0 && n.saturating_abs() / 10_i32.pow(N as u32 - 1) > 0 {
                            // if integral part is negative
                            // and has more digits than we can hold
                            // then fill all digits with 9
                            // except for the left-most which will be a minus
                            let mut arr = [digits::NINE; N];
                            arr[0] = misc::MINUS;
                            Ok(Self(arr))
                        } else {
                            // fill word from left to right with the integral part and
                            // then if we can fit any decimal digits,
                            // add a dot to the last integral digit
                            // and add the decimal digits
                            let mut arr = [misc::EMPTY; N];
                
                            // set first display to minus if number is negative
                            let mut offset = 0;
                            if num < 0.0 {
                                arr[0] = misc::MINUS;
                                offset = 1;
                            }
                            let mut n = n.unsigned_abs();
                            let mut iter = offset..N;
                
                            // get integral part digits
                            // while let Some(i) = iter.next() {
                            for i in iter.by_ref() {
                                let div = 10_u32.pow((i - offset) as u32);
                                let d = (n / div) as u8;
                                let s = Symbol::from_u8(d).expect("d cannot be greater than 9 because of MOD");
                                n %= div;
                                if n == 0 {
                                    arr[i] = s.with_dot();
                                    break;
                                } else {
                                    arr[i] = s;
                                }
                            }
                
                            // get fractional part digits
                            let mut num = if num < 0.0 { -1.0 * num } else { num };
                            num = num - (num as u32 as $T);
                            for i in iter {
                                num *= 10.0;
                                let d = num as u8;
                                arr[i] = Symbol::from_u8(d).expect("d cannot be greater than 9 because of MOD");
                                num -= d as $T;
                            }
                            Ok(Self(arr))
                        }
                    }
                }
            }
        )+
    };
}