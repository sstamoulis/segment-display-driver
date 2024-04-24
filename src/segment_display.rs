use crate::symbols::{self, CannotConvertNaN, Word};

pub enum CommonPin {
    Anode,
    Cathode,
}

pub trait SegmentDisplay<const NUM_DIGITS: usize> {
    fn set_digits(&mut self, digits: Word<NUM_DIGITS>);
    fn show_digits(&mut self);
}

pub trait Show<W, const N: usize> {
    fn show(&mut self, word: W);
}

// impl<SD, const NUM_DIGITS: usize> Show<Word<NUM_DIGITS>, NUM_DIGITS> for SD
// where
//     SD: SegmentDisplay<NUM_DIGITS>,
// {
//     fn show(&mut self, word: Word<NUM_DIGITS>) {
//         self.set_digits(word);
//         self.show_digits();
//     }
// }

impl<SD, W, const NUM_DIGITS: usize> Show<W, NUM_DIGITS> for SD
where
    SD: SegmentDisplay<NUM_DIGITS>,
    W: Into<Word<NUM_DIGITS>>,
{
    fn show(&mut self, word: W) {
        self.set_digits(word.into());
        self.show_digits();
    }
}

pub trait TryShow<W, const N: usize> {
    type Error;
    fn try_show(&mut self, word: W) -> Result<(), Self::Error>;
}

impl<SD, W, const NUM_DIGITS: usize> TryShow<W, NUM_DIGITS> for SD
where
    SD: SegmentDisplay<NUM_DIGITS>,
    W: TryInto<Word<NUM_DIGITS>>,
{
    type Error = CannotConvertNaN;

    fn try_show(&mut self, word: W) -> Result<(), Self::Error> {
        match word.try_into() {
            Ok(word) => {
                self.show(word);
                Ok(())
            }
            Err(_) => Err(CannotConvertNaN),
        }
    }
}

pub trait Clear<const NUM_DIGITS: usize> {
    fn clear(&mut self);
}

impl<SD, const NUM_DIGITS: usize> Clear<NUM_DIGITS> for SD
where
    SD: SegmentDisplay<NUM_DIGITS>,
{
    fn clear(&mut self) {
        self.set_digits(Word::from_symbol_array([symbols::misc::EMPTY; NUM_DIGITS]));
        self.show_digits();
    }
}