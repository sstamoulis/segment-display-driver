macro_rules! impl_from_int {
    ($($T: ty),+) => {
        $(
            impl<const N: usize> Word<N> {
                paste::paste!{
                    pub fn [<from_ $T>](num: $T) -> Self {
                        // if num has more digits than we can hold
                        // then fill all digits with 9
                        if num > 0 && (10 as $T).checked_pow(N as u32).is_some_and(|d| num / d > 0) {
                            Self([digits::NINE; N])
                        } else if num < 0 && (10 as $T).checked_pow(N as u32 - 1).is_some_and(|d| num.saturating_abs() / d > 0) {
                            let mut arr = [digits::NINE; N];
                            arr[0] = misc::MINUS;
                            Self(arr)
                        } else {
                            let mut arr = [misc::EMPTY; N];
                            let mut offset = 0;
                            if (num < 0) {
                                arr[0] = misc::MINUS;
                                offset = 1;
                            }
                            let mut num = num.unsigned_abs();
                            for i in (offset..N).rev() {
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