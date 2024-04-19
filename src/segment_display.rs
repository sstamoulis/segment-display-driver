pub enum Error {
    TooManyDigits,
    DecimalPositionOutOfBounds,
}

pub trait SegmentDisplay<const NUM_DIGITS: usize> {
    fn set_digits(&mut self, digits: [u8; NUM_DIGITS], decimal_pos: u8);

    fn show(&mut self);
}

pub trait ShowNumber<T, const NUM_DIGITS: usize> {
    fn show_number(&mut self, num: T);
}

macro_rules! impl_show_number {
    ($($T:ty),+) => {
        $(
            impl<SD, const NUM_DIGITS: usize> ShowNumber<$T, NUM_DIGITS> for SD
            where
                SD: SegmentDisplay<NUM_DIGITS>,
            {
                fn show_number(&mut self, num: $T) {
                    let mut num = num;
                    let digits = core::array::from_fn(|_| {
                        let d = (num % 10) as u8;
                        num /= 10;
                        d
                    });
                    self.set_digits(digits, 0);
                }
            }
        )+
    };
}

impl_show_number!(u8, u16, u32, u64, usize);
