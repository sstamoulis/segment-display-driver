macro_rules! impl_from_uint {
    ($($T: ty),+) => {
        $(
            impl<const N: usize> Word<N> {
                paste::paste!{
                    pub fn [<from_ $T>](num: $T) -> Self {
                        // if num has more digits than we can hold
                        // then fill all digits with 9
                        if (10 as $T).checked_pow(N as u32).is_some_and(|d| num / d > 0) {
                            Self([digits::NINE; N])
                        } else {
                            let mut num = num;
                            let mut arr = [misc::EMPTY; N];
                            for i in (0..N).rev() {
                                let d = (num % 10) as u8;
                                arr[i] = Symbol::from_u8(d).expect("d cannot be greater than 9 because of MOD");
                                num /= 10;
                                if num == 0 {
                                    break;
                                }
                            }
                            Self(arr)
                        }
                    }
                }
            }
        )+
    };
}
