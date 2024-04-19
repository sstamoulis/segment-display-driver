#![allow(dead_code)]

//! Misc symbols

use super::Symbol;

/// ```text
///
///  _  GFEDCBA
///     1000000
/// ```
pub const MINUS: Symbol = Symbol::from_byte(0b01000000);

/// ```text
///
///     GFEDCBA
///     0000000
/// ```
pub const EMPTY: Symbol = Symbol::from_byte(0b00000000);
