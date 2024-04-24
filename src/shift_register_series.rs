use core::fmt::Debug;

use crate::{
    segment_display::{Clear as _, CommonPin, SegmentDisplay},
    symbols::Word,
};
use embedded_hal::digital::v2::OutputPin;

pub struct ShiftRegisterSeries<const NUM_DIGITS: usize, CLK, DT, LATCH, E>
where
    CLK: OutputPin<Error = E>,
    DT: OutputPin<Error = E>,
    LATCH: OutputPin<Error = E>,
{
    clock_pin: CLK,
    data_pin: DT,
    latch_pin: LATCH,
    common_pin: CommonPin,
}

impl<CLK, DT, LATCH, E, const NUM_DIGITS: usize> ShiftRegisterSeries<NUM_DIGITS, CLK, DT, LATCH, E>
where
    CLK: OutputPin<Error = E>,
    DT: OutputPin<Error = E>,
    LATCH: OutputPin<Error = E>,
    E: Debug,
{
    #[allow(dead_code)]
    pub fn new(clock_pin: CLK, data_pin: DT, latch_pin: LATCH, common_pin: CommonPin) -> Self {
        let mut me = Self {
            clock_pin,
            data_pin,
            latch_pin,
            common_pin,
        };
        me.clock_pin.set_high().unwrap();
        me.latch_pin.set_high().unwrap();
        me.clear();
        me
    }
}

impl<CLK, DT, LATCH, E, const NUM_DIGITS: usize> SegmentDisplay<NUM_DIGITS>
    for ShiftRegisterSeries<NUM_DIGITS, CLK, DT, LATCH, E>
where
    CLK: OutputPin<Error = E>,
    DT: OutputPin<Error = E>,
    LATCH: OutputPin<Error = E>,
    E: Debug,
{
    fn set_digits(&mut self, digits: Word<NUM_DIGITS>) {
        self.latch_pin.set_low().unwrap();
        for symbol in digits.into_symbol_array().into_iter().rev() {
            for i in 0..8 {
                let value = symbol.get_value(&self.common_pin);
                self.clock_pin.set_low().unwrap();
                let state = (value & (0b10000000 >> i)) > 0;
                self.data_pin.set_state(state.into()).unwrap();
                self.clock_pin.set_high().unwrap();
            }
        }
        self.clock_pin.set_low().unwrap();
        self.data_pin.set_low().unwrap();
    }

    fn show_digits(&mut self) {
        self.latch_pin.set_high().unwrap();
    }
}
