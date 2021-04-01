use super::*;

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum CardCombo {
    HighCard(Rank, Rank, Rank, Rank, Rank),
    Pair(Rank, Rank),
    TwoPairs(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Rank),
    Straight(Rank),
    Flush(Rank, Rank, Rank, Rank, Rank),
    FullHouse(Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

type CC = CardCombo;

fn find_royal_flush(cards: &[Card]) -> Option<CardCombo> {
    if let Some(CC::StraightFlush(Rank::Ace)) = find_straight_flush(cards) {
        Some(CC::RoyalFlush) }
    else {
        None
    }
}

fn find_straight_flush(cards: &[Card]) -> Option<CardCombo> {
    let mut suitcount: HashMap<Suit, u8> = HashMap::new();
    for s in cards.iter().map(|c| c.suit()) {
        *suitcount.entry(s).or_insert(0) += 1;
    }

    if let Some((suit, _)) = suitcount
        .iter()
        .find(|(_,i)| **i >= 5) {
            let mut flush_sorted = cards
                .iter()
                .filter(|c| c.suit() == *suit)
                .map(|c| c.rank())
                .collect::<Vec<Rank>>();
            flush_sorted.sort_by(|a, b| b.cmp(a));
            let res =
                flush_sorted
                .windows(5)
                .map(|s|
                    (s[0],
                    s
                    .windows(2)
                    .map(|is| (is[0], is[1]))
                    .map(|(a, b)| a.is_next(b))
                    .all(|i| i)
                    ))
                .find(|(r, b)| *b);
            res.map(|(r,_)| CC::StraightFlush(r))
    } else {
        None
    }
}

fn find_four_of_a_kind(cards: &[Card]) -> Option<CardCombo> {
    let mut sorted_cards: Vec<Rank> =
        cards
        .iter()
        .map(|c| c.rank())
        .collect::<Vec<_>>();
    sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
    if let Some((four_rank, _, _, _)) =
            sorted_cards.windows(4)
            .map(|s| (s[0], s[1], s[2], s[3]))
            .find(|(a, b, c, d)| a==b && b==c && c==d) {
        let high_rank =
            sorted_cards
            .iter()
            .filter(|r| **r != four_rank)
            .max()
            .unwrap();
        Some(CC::FourOfAKind(four_rank, *high_rank))
    } else {
        None
    }
}

fn find_full_house(cards: &[Card]) -> Option<CardCombo> {
    let mut sorted_cards: Vec<Rank> =
        cards
        .iter()
        .map(|c| c.rank())
        .collect::<Vec<_>>();
    sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
    if let Some((three_rank, _, _)) =
            sorted_cards.windows(3)
            .map(|s| (s[0], s[1], s[2]))
            .find(|(a, b, c)| a==b && b==c) {
        if sorted_cards.windows(2)
                .map(|s| (s[0], s[1]))
                .filter(|(a, _)| *a != three_rank)
                .any(|(a, b)| a==b) {
            return Some(CC::FullHouse(three_rank));
        }
    }
    None

}

fn find_flush(cards: &[Card]) -> Option<CardCombo> {
    let mut suitcount: HashMap<Suit, u8> = HashMap::new();
    for s in cards.iter().map(|c| c.suit()) {
        *suitcount.entry(s).or_insert(0) += 1;
    }

    if let Some((suit, _)) = suitcount
        .iter()
        .find(|(_,i)| **i >= 5) {
            let mut flush_sorted = cards
                .iter()
                .filter(|c| c.suit() == *suit)
                .map(|c| c.rank())
                .collect::<Vec<Rank>>();
            flush_sorted.sort_by(|a, b| b.cmp(a));
            let c1 = flush_sorted[0];
            let c2 = flush_sorted[1];
            let c3 = flush_sorted[2];
            let c4 = flush_sorted[3];
            let c5 = flush_sorted[4];
            Some(CC::Flush(c1, c2, c3, c4, c5))
    } else {
        None
    }
}

fn find_straight(cards: &[Card]) -> Option<CardCombo> {
    let mut sorted_cards = cards.iter().map(|c| c.rank()).collect::<Vec<Rank>>();
    sorted_cards.sort_by(|a, b| b.cmp(a));
    println!("{:?}", sorted_cards);
    let res =
        sorted_cards
        .windows(5)
        .map(|s|
            (s[0],
            s
            .windows(2)
            .map(|is| (is[0], is[1]))
            .map(|(a, b)| a.is_next(b))
            .all(|i| i)
            ))
        .find(|(r, b)| *b);
    res.map(|(r, _)| CC::Straight(r))
}

fn find_three_of_a_kind(cards: &[Card]) -> Option<CardCombo> {
    let mut sorted_cards: Vec<Rank> =
        cards
        .iter()
        .map(|c| c.rank())
        .collect::<Vec<_>>();
    sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
    if let Some((three_rank, _, _)) =
            sorted_cards.windows(3)
            .map(|s| (s[0], s[1], s[2]))
            .find(|(a, b, c)| a==b && b==c) {
        let high_rank =
            sorted_cards
            .iter()
            .filter(|r| **r != three_rank)
            .max()
            .unwrap();
        Some(CC::ThreeOfAKind(three_rank, *high_rank))
    } else {
        None
    }
}

fn find_two_pairs(cards: &[Card]) -> Option<CardCombo> {
    let mut sorted_cards: Vec<Rank> =
        cards
        .iter()
        .map(|c| c.rank())
        .collect::<Vec<_>>();
    sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
    if let Some((first_pair, _)) =
            sorted_cards.windows(2)
            .map(|s| (s[0], s[1]))
            .find(|(a, b)| a==b) {
        if let Some((second_pair, _)) =
            sorted_cards
                .windows(2)
                .map(|s| (s[0], s[1]))
                .filter(|(a, _)| *a != first_pair )
                .find(|(a, b)| a == b) {
            let high_rank : Rank =
                *sorted_cards
                .iter()
                .filter(|r| ![first_pair, second_pair].contains(r))
                .max()
                .unwrap();
            Some(CC::TwoPairs(first_pair, second_pair, high_rank))
        } else {
            None
        }
    } else {
        None
    }
}

fn find_pair(cards: &[Card]) -> Option<CardCombo> {
    let mut sorted_cards: Vec<Rank> =
        cards
        .iter()
        .map(|c| c.rank())
        .collect::<Vec<_>>();
    sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
    if let Some((pair_rank, _)) =
            sorted_cards.windows(2)
            .map(|s| (s[0], s[1]))
            .find(|(a, b)| a==b) {
        let high_rank =
            sorted_cards
            .iter()
            .filter(|r| **r != pair_rank)
            .max()
            .unwrap();
        Some(CC::Pair(pair_rank, *high_rank))
    } else {
        None
    }
}

fn find_high_card(cards: &[Card]) -> CardCombo {
    let mut sorted_cards: Vec<Rank> =
        cards
        .iter()
        .map(|c| c.rank())
        .collect::<Vec<_>>();
    sorted_cards.sort_unstable_by(|a, b| b.cmp(a));
    let c1 = sorted_cards[0];
    let c2 = sorted_cards[1];
    let c3 = sorted_cards[2];
    let c4 = sorted_cards[3];
    let c5 = sorted_cards[4];

    CC::HighCard(c1, c2, c3, c4, c5)
}

pub fn find_best_card(cards: &[Card]) -> CardCombo {
    find_royal_flush(cards)
        .or_else(|| find_straight_flush(cards))
        .or_else(|| find_four_of_a_kind(cards))
        .or_else(|| find_full_house(cards))
        .or_else(|| find_flush(cards))
        .or_else(|| find_straight(cards))
        .or_else(|| find_three_of_a_kind(cards))
        .or_else(|| find_two_pairs(cards))
        .or_else(|| find_pair(cards))
        .unwrap_or_else(|| find_high_card(cards))
}

