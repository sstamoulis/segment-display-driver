use crate::symbols::{self, CannotConvertNaN, Word};

pub trait SegmentDisplay<const NUM_DIGITS: usize> {
    fn set_digits(&mut self, digits: Word<NUM_DIGITS>);

    fn show(&mut self);
}

pub trait ShowNumber<T, const NUM_DIGITS: usize> {
    fn show_number(&mut self, num: T);
}

pub trait TryShowNumber<T, const NUM_DIGITS: usize> {
    fn try_show_number(&mut self, num: T) -> Result<(), CannotConvertNaN>;
}

pub trait ShowWord<const NUM_DIGITS: usize> {
    fn show_word(&mut self, word: Word<NUM_DIGITS>);
}

pub trait Clear<const NUM_DIGITS: usize> {
    fn clear(&mut self);
}

impl<SD, const NUM_DIGITS: usize> ShowWord<NUM_DIGITS> for SD
where
    SD: SegmentDisplay<NUM_DIGITS>,
{
    fn show_word(&mut self, word: Word<NUM_DIGITS>) {
        self.set_digits(word);
        self.show();
    }
}

impl<SD, const NUM_DIGITS: usize> Clear<NUM_DIGITS> for SD
where
    SD: SegmentDisplay<NUM_DIGITS>,
{
    fn clear(&mut self) {
        self.set_digits(Word::from_symbol_array([symbols::misc::EMPTY; NUM_DIGITS]));
        self.show();
    }
}

macro_rules! impl_show_number {
    ($($T:ty),+) => {
        $(
            paste::paste!{
                impl<SD, const NUM_DIGITS: usize> ShowNumber<$T, NUM_DIGITS> for SD
                where
                    SD: SegmentDisplay<NUM_DIGITS>,
                {
                    fn show_number(&mut self, num: $T) {
                        let word = Word::<NUM_DIGITS>::[<from_ $T>](num);
                        self.set_digits(word);
                        self.show();
                    }
                }
            }
        )+
    };
}

macro_rules! impl_try_show_number {
    ($($T:ty),+) => {
        $(
            paste::paste!{
                impl<SD, const NUM_DIGITS: usize> TryShowNumber<$T, NUM_DIGITS> for SD
                where
                    SD: SegmentDisplay<NUM_DIGITS>,
                {
                    fn try_show_number(&mut self, num: $T) -> Result<(), CannotConvertNaN> {
                        let word = Word::<NUM_DIGITS>::[<try_from_ $T>](num)?;
                        self.set_digits(word);
                        self.show();
                        Ok(())
                    }
                }
            }
        )+
    };
}

impl_show_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_try_show_number!(f32, f64);
