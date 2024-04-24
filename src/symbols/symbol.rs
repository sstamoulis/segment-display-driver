use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::segment_display::CommonPin;

use super::digits;

/// A symbol in common cathode configuration
/// if a bit is 1, then that led is on
/// # Example
/// ``` text
///  _
/// | | GFEDCBA Byte
/// |_| 0111111 0x00111111
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbol(u8);

#[derive(Debug)]
pub struct NoSymbolFound;

impl Symbol {
    pub const fn from_byte(byte: u8) -> Self {
        Self(byte)
    }
}

impl Symbol {
    #[allow(dead_code)]
    pub const fn from_u8(num: u8) -> Result<Self, NoSymbolFound> {
        match num {
            0 => Ok(digits::ZERO),
            1 => Ok(digits::ONE),
            2 => Ok(digits::TWO),
            3 => Ok(digits::THREE),
            4 => Ok(digits::FOUR),
            5 => Ok(digits::FIVE),
            6 => Ok(digits::SIX),
            7 => Ok(digits::SEVEN),
            8 => Ok(digits::EIGHT),
            9 => Ok(digits::NINE),
            _ => Err(NoSymbolFound),
        }
    }

    pub const fn with_dot(self) -> Self {
        Self(self.0 | 0b10000000)
    }

    pub const fn get_value(&self, common_pin: &CommonPin) -> u8 {
        match common_pin {
            CommonPin::Anode => !self.0,
            CommonPin::Cathode => self.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::digits, Symbol};

    #[test]
    fn zero_from_u8() {
        assert!(matches!(
            Symbol::from_u8(0),
            Ok(digits::ZERO)
        ))
    }
}
