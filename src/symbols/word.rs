#![allow(dead_code)]

#[macro_use]
mod macro_from_uint;
#[macro_use]
mod macro_from_int;
#[macro_use]
mod macro_try_from_float;

use super::{digits, misc, Symbol};
use paste::paste;

/// `Word` is an array of `Symbol`
#[derive(Debug, PartialEq, Eq)]
pub struct Word<const N: usize>([Symbol; N]);

impl_from_uint!(u8, u16, u32, u64, usize);
impl_from_int!(i8, i16, i32, i64, isize);
impl_try_from_float!(f32, f64);

impl<const N: usize> Word<N> {
    pub const fn from_symbol_array(arr: [Symbol; N]) -> Self {
        Self(arr)
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::{
        digits::*,
        misc::{self, EMPTY, MINUS},
        CannotConvertNaN,
    };

    use super::{super::digits, Word};

    const WORD_83: Word<4> = Word([EMPTY, EMPTY, EIGHT, THREE]);
    const WORD_0: Word<4> = Word([EMPTY, EMPTY, EMPTY, ZERO]);
    const WORD_0_619: Word<4> = Word([ZERO.with_dot(), SIX, ONE, NINE]);
    const WORD_0_620: Word<4> = Word([ZERO.with_dot(), SIX, TWO, ZERO]);
    const WORD_MINUS_0_62: Word<4> = Word([MINUS, ZERO.with_dot(), SIX, TWO]);
    const WORD_MINUS_0_61: Word<4> = Word([MINUS, ZERO.with_dot(), SIX, ONE]);
    const WORD_9999: Word<4> = Word([NINE; 4]);
    const WORD_MINUS_999: Word<4> = Word([MINUS, NINE, NINE, NINE]);
    const WORD_MINUS_83: Word<4> = Word([MINUS, EMPTY, EIGHT, THREE]);
    const WORD_MINUS_154: Word<4> = Word([MINUS, ONE, FIVE, FOUR]);

    #[test]
    fn word_from_symbol_array() {
        let word = Word::from_symbol_array([EMPTY, EMPTY, EIGHT, THREE]);
        assert_eq!(word, WORD_83)
    }

    #[test]
    fn word_from_uint() {
        assert_eq!(Word::from_u8(83), WORD_83);
        assert_eq!(Word::from_u16(83), WORD_83);
        assert_eq!(Word::from_u32(83), WORD_83);
        assert_eq!(Word::from_u64(83), WORD_83);
        assert_eq!(Word::from_usize(83), WORD_83);

        assert_eq!(Word::from_u8(0), WORD_0);
        assert_eq!(Word::from_u16(0), WORD_0);
        assert_eq!(Word::from_u32(0), WORD_0);
        assert_eq!(Word::from_u64(0), WORD_0);
        assert_eq!(Word::from_usize(0), WORD_0);
    }

    #[test]
    fn word_from_int() {
        assert_eq!(Word::from_i8(83), WORD_83);
        assert_eq!(Word::from_i16(83), WORD_83);
        assert_eq!(Word::from_i32(83), WORD_83);
        assert_eq!(Word::from_i64(83), WORD_83);
        assert_eq!(Word::from_isize(83), WORD_83);
    }

    #[test]
    fn word_from_negative_int() {
        assert_eq!(Word::from_i8(-83), WORD_MINUS_83);
        assert_eq!(Word::from_i16(-154), WORD_MINUS_154);
        assert_eq!(Word::from_i32(-154), WORD_MINUS_154);
        assert_eq!(Word::from_i64(-154), WORD_MINUS_154);
        assert_eq!(Word::from_isize(-154), WORD_MINUS_154);
    }

    #[test]
    fn word_from_f32() {
        assert!(
            matches!(Word::try_from_f32(0.62),Ok(word) if word == WORD_0_620 || word == WORD_0_619)
        )
    }
    #[test]
    fn word_from_f32_negative() {
        let result = Word::try_from_f32(-0.62);
        assert!(matches!(result,Ok(word) if word == WORD_MINUS_0_62 || word == WORD_MINUS_0_61))
    }
    #[test]
    fn word_from_f32_nan() {
        assert!(matches!(
            Word::<4>::try_from_f32(f32::NAN),
            Err(CannotConvertNaN)
        ))
    }
    #[test]
    fn word_from_f32_positive_inf() {
        assert!(matches!(Word::try_from_f32(f32::INFINITY), Ok(word) if word == WORD_9999))
    }
    #[test]
    fn word_from_f32_negative_inf() {
        assert!(matches!(Word::try_from_f32(f32::NEG_INFINITY), Ok(word) if word == WORD_MINUS_999))
    }
    
    #[test]
    fn word_from_f64() {
        assert!(
            matches!(Word::try_from_f64(0.62),Ok(word) if word == WORD_0_620 || word == WORD_0_619)
        )
    }
    #[test]
    fn word_from_f64_negative() {
        let result = Word::try_from_f64(-0.62);
        assert!(matches!(result,Ok(word) if word == WORD_MINUS_0_62 || word == WORD_MINUS_0_61))
    }
    #[test]
    fn word_from_f64_nan() {
        assert!(matches!(
            Word::<4>::try_from_f64(f64::NAN),
            Err(CannotConvertNaN)
        ))
    }
    #[test]
    fn word_from_f64_positive_inf() {
        assert!(matches!(Word::try_from_f64(f64::INFINITY), Ok(word) if word == WORD_9999))
    }
    #[test]
    fn word_from_f64_negative_inf() {
        assert!(matches!(Word::try_from_f64(f64::NEG_INFINITY), Ok(word) if word == WORD_MINUS_999))
    }
}
