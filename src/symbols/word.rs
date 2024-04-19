#![allow(dead_code)]

use super::{digits, misc, Symbol};

/// `Word` is an array of `Symbol`
#[derive(Debug, PartialEq, Eq)]
pub struct Word<const N: usize>([Symbol; N]);

pub struct CannotConvert;

macro_rules! fn_from_uint {
    ($($T: ty),+) => {
        $(
            paste::paste!{
                pub fn [<from_ $T>](num: $T) -> Self {
                    let mut num = num;
                    let mut arr = [misc::EMPTY; N];
                    for i in (0..N).rev() {
                        let d = (num % 10) as u8;
                        arr[i] = Symbol::from_u8(d).expect("d cannot be greater than 9 because of MOD");
                        num /= 10;
                        // ignore zeroes after the number
                        if num == 0 {
                            break;
                        }
                    }
                    Self(arr)
                }
            }
        )+
    };
}

impl<const N: usize> Word<N> {
    pub const fn from_symbol_array(arr: [Symbol; N]) -> Self {
        Self(arr)
    }

    fn_from_uint!(u8, u16, u32, u64, usize);

    pub fn try_from_f32(num: f32) -> Result<Self, CannotConvert> {
        let n = num as i32;
        if n >= 0 && n / 10_i32.pow(N as u32) > 0 {
            // if integral part is positive
            // and has more digits than we can hold
            // then fill all digits with 9
            Ok(Self([digits::NINE; N]))
        } else if n < 0 && n / 10_i32.pow(N as u32 - 1) > 0 {
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
            
            todo!()
        }
    }

    // pub fn from_u8(num: u8) -> Self {
    //     let mut num = num;
    //     let mut arr = [misc::EMPTY; N];
    //     for i in (0..N).rev() {
    //         let d = (num % 10) as u8;
    //         arr[i] = Symbol::from_u8(d).expect("d cannot be greater than 9 because of MOD");
    //         num /= 10;
    //         // ignore zeroes after the number
    //         if num == 0 {
    //             break;
    //         }
    //     }
    //     Self(arr)
    // }
}

#[cfg(test)]
mod tests {
    use crate::symbols::{
        digits::*,
        misc::{self, EMPTY},
    };

    use super::{super::digits, Word};

    const WORD_83: Word<4> = Word([EMPTY, EMPTY, EIGHT, THREE]);
    const WORD_0: Word<4> = Word([EMPTY, EMPTY, EMPTY, ZERO]);
    const WORD_0_62: Word<4> = Word([EMPTY, ZERO.with_dot(), SIX, TWO]);

    #[test]
    fn word_from_symbol_array() {
        let word = Word::from_symbol_array([EMPTY, EMPTY, EIGHT, THREE]);
        assert_eq!(word, WORD_83)
    }

    #[test]
    fn word_from_u8() {
        assert_eq!(Word::from_u8(83), WORD_83)
    }
    #[test]
    fn word_from_u16() {
        assert_eq!(Word::from_u16(83), WORD_83)
    }
    #[test]
    fn word_from_u32() {
        assert_eq!(Word::from_u32(83), WORD_83)
    }
    #[test]
    fn word_from_u64() {
        assert_eq!(Word::from_u64(83), WORD_83)
    }
    #[test]
    fn word_from_usize() {
        assert_eq!(Word::from_usize(83), WORD_83)
    }

    #[test]
    fn zero_word_from_u8() {
        assert_eq!(Word::from_u8(0), WORD_0)
    }
    #[test]
    fn zero_word_from_u16() {
        assert_eq!(Word::from_u16(0), WORD_0)
    }
    #[test]
    fn zero_word_from_u32() {
        assert_eq!(Word::from_u32(0), WORD_0)
    }
    #[test]
    fn zero_word_from_u64() {
        assert_eq!(Word::from_u64(0), WORD_0)
    }
    #[test]
    fn zero_word_from_usize() {
        assert_eq!(Word::from_usize(0), WORD_0)
    }

    #[test]
    fn word_from_f32() {
        assert!(matches!(Word::try_from_f32(0.62), Ok(WORD_0_62)))
    }
}
