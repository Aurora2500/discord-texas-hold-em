use std::collections::HashMap;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use rand::seq::SliceRandom;

mod combo;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Card(Suit, Rank);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Rank {
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
    King,
    Ace
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Rank {
    fn next(&self) -> Self {
        match self {
            Rank::Two => Rank::Three,
            Rank::Three => Rank::Four,
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Eight,
            Rank::Eight => Rank::Nine,
            Rank::Nine => Rank::Ten,
            Rank::Ten => Rank::Jack,
            Rank::Jack => Rank::Queen,
            Rank::Queen => Rank::King,
            Rank::King => Rank::Ace,
            Rank::Ace => Rank::Two
        }
    }

    fn is_next(self, other: Self) -> bool {
        self == other.next()
    }

    fn prev(&self) -> Self {
        match self {
            Rank::Three => Rank::Two,
            Rank::Four => Rank::Three,
            Rank::Five => Rank::Four,
            Rank::Six => Rank::Five,
            Rank::Seven => Rank::Six,
            Rank::Eight => Rank::Seven,
            Rank::Nine => Rank::Eight,
            Rank::Ten => Rank::Nine,
            Rank::Jack => Rank::Ten,
            Rank::Queen => Rank::Jack,
            Rank::King => Rank::Queen,
            Rank::Ace => Rank::King,
            Rank::Two => Rank::Ace
        }
    }

    fn is_prev(self, other: Self) -> bool {
        self == other.prev()
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

pub struct Deck {
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

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Hand (Card, Card);

impl Hand {
    fn new(deck: &mut Deck) -> Option<Self> {
        Some(Hand(deck.pop()?, deck.pop()?))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub id: u64,
    pub hand: Option<Hand>,
    pub is_playing: bool,
    pub money: u32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Player {}

impl Player {
    fn new(id: u64, money: u32) -> Self {
        Player {
            id,
            hand: None,
            is_playing: false,
            money
       }
    }
}

pub struct PlayerRing {
    vec: Vec<u64>,
    map: HashMap<u64, Player>,
    order_count: usize,
}

impl PlayerRing {
    pub fn new(players: &[u64], starting_money: u32) -> Self {
        let vec: Vec<u64> =
            players.to_vec();
        let map = vec.iter()
            .map(|id| (*id, Player::new(*id, starting_money)) )
            .collect();
        PlayerRing {
            vec,
            map,
            order_count: 0
        }
    }

    pub fn next(&mut self) -> &Player {
        let return_player = self.vec[self.order_count];
        self.order_count = (self.order_count + 1) % self.vec.len(); 
        &self.map[&return_player]
    }

    pub fn get_player(&mut self, id: u64) -> Option<&Player> {
        self.map.get(&id)
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

pub struct Game {
    pub deck: Deck,
    pub board: Option<Board>,
    pub players: HashMap<u64, Option<Hand>>,
    pub bet: u16,
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
