use self::Hand::*;
use self::HandResult::*;

#[derive(Debug, Eq, PartialEq)]
pub enum HandResult {
    Win,
    Lose,
    Draw,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}
pub trait Beats {
    fn beats(&self) -> Self;
}
impl Beats for Hand {
    fn beats(&self) -> Self {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}
pub fn play_hand(own_hand: Hand, other_hand: Hand) -> HandResult {
    let (own_beats, other_beats) = (own_hand.beats(), other_hand.beats());

    match (own_beats, other_beats) {
        _ if own_beats == other_hand => Win,
        _ if other_beats == own_hand => Lose,
        _                            => Draw,
    }
}

