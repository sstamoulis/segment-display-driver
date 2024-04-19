#![allow(dead_code)]

//! Digit symbols

use super::Symbol;

/// ```text
///  _
/// | | GFEDCBA
/// |_| 0111111
/// ```
pub const ZERO: Symbol = Symbol::from_byte(0b00111111);

/// ```text
///
///   | GFEDCBA
///   | 0000110
/// ```
pub const ONE: Symbol = Symbol::from_byte(0b00000110);

/// ```text
///  _
///  _| GFEDCBA
/// |_  1011011
/// ```
pub const TWO: Symbol = Symbol::from_byte(0b01011011);

/// ```text
///  _
///  _| GFEDCBA
///  _| 1001111
/// ```
pub const THREE: Symbol = Symbol::from_byte(0b01001111);

/// ```text
///  
/// |_| GFEDCBA
///   | 1100110
/// ```
pub const FOUR: Symbol = Symbol::from_byte(0b01100110);

/// ```text
///  _
/// |_  GFEDCBA
///  _| 1101101
/// ```
pub const FIVE: Symbol = Symbol::from_byte(0b01101101);

/// ```text
///  _
/// |_  GFEDCBA
/// |_| 1111101
/// ```
pub const SIX: Symbol = Symbol::from_byte(0b01111101);

/// ```text
///  _
///   | GFEDCBA
///   | 0000111
/// ```
pub const SEVEN: Symbol = Symbol::from_byte(0b00000111);

/// ```text
///  _
/// |_| GFEDCBA
/// |_| 1111111
/// ```
pub const EIGHT: Symbol = Symbol::from_byte(0b01111111);

/// ```text
///  _
/// |_| GFEDCBA
///  _| 1101111
/// ```
pub const NINE: Symbol = Symbol::from_byte(0b01101111);
