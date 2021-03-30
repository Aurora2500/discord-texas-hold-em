use std::collections::HashMap;
use std::fmt;

use rand::seq::SliceRandom;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Card(Suit, Rank);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Card {
    pub fn new(s: Suit, r: Rank) -> Self {
        Card(s, r)
    }

    pub fn suit(&self) -> Suit {
        self.0
    }

    pub fn rank(&self) -> Rank {
        self.1
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.1, self.0)
    }
}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
            for s in [
                Suit::Clubs,
                Suit::Diamonds,
                Suit::Hearts,
                Suit::Spades
            ].iter() {
                for r in [
                    Rank::Ace,
                    Rank::Two,
                    Rank::Three,
                    Rank::Four,
                    Rank::Five,
                    Rank::Six,
                    Rank::Seven,
                    Rank::Eight,
                    Rank::Nine,
                    Rank::Ten,
                    Rank::Jack,
                    Rank::Queen,
                    Rank::King
                ].iter() {
                    cards.push(Card::new(*s, *r));
                }
            }
        Deck{ cards }
    }

    pub fn pop(&mut self) -> Option<Card> {
        let res = self.cards
                   .iter()
                   .enumerate()
                   .collect::<Vec<(usize, &Card)>>()
                   .choose(&mut rand::thread_rng()).cloned();
        if let Some((i, &out)) = res {
            self.cards.remove(i);
            return Some(out);
        } else {
            return None;
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

struct Hand (Card, Card);

impl Hand {
    fn new(deck: &mut Deck) -> Option<Self> {
        Some(Hand(deck.pop()?, deck.pop()?))
    }
}

pub struct Board(Card, Card, Card, Card, Card);


impl Board {
    fn new(deck: &mut Deck) -> Option<Self> {
       Some(Board(deck.pop()?, deck.pop()?, deck.pop()?, deck.pop()?, deck.pop()?))
    }

    fn from_cards(c1: Card, c2: Card, c3: Card, c4: Card, c5: Card) -> Self {
        Board(c1, c2, c3, c4, c5)
    }

    pub fn flop (&self) -> (Card, Card, Card) {
        (self.0, self.1, self.2)
    }

    pub fn turn (&self) -> Card {
        self.3
    }

    pub fn river (&self) -> Card {
        self.4
    }
}

struct Game {
    deck: Deck,
    board: Option<Board>,
    players: HashMap<u64, Option<Hand>>,
    bet: u16,
}

impl Game {
    pub fn new(players: Vec<u64>) -> Self {
        Game {
            deck: Deck::new(),
            board: None,
            players: players.into_iter().map(|u| { (u, None) }).collect(),
            bet: 0,
        }
    }
}

#[cfg(test)]
mod test;
