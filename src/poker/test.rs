use super::*;

#[test]
fn deck() {
    let mut deck = Deck::new();
    // Deck should start with 51 cards.
    assert_eq!(deck.len(), 52); 
    let card = deck.pop().unwrap(); 
    // When taking out a card, it should not be in the deck.
    assert!(!deck.cards.contains(&card)); 
    // After taking out a card, there shall only be 51 cards.
    assert_eq!(deck.len(), 51); 
    for _ in 0..51 {
        deck.pop();
    }
    // After taking out all the cards, taking out more will give you None.
    assert_eq!(deck.pop(), None); 
}

#[test]
fn board() {
    let c1 = Card(Suit::Clubs, Rank::Ace);
    let c2 = Card(Suit::Hearts, Rank::Ten);
    let c3 = Card(Suit::Diamonds, Rank::Jack);
    let c4 = Card(Suit::Spades, Rank::Seven);
    let c5 = Card(Suit::Clubs, Rank::Two);

    let board = Board::from_cards(c1, c2, c3, c4, c5);

    assert_eq!(board.flop(), (c1, c2, c3));
    assert_eq!(board.turn(), c4);
    assert_eq!(board.river(), c5);
}

#[test]
fn player_ring() {
    let players = [1, 2, 3];
    let mut ring = PlayerRing::new(&players, 0);
    assert_eq!(ring.next().id, 1);
    assert_eq!(ring.next().id, 2);
    assert_eq!(ring.next().id, 3);
    assert_eq!(ring.next().id, 1);
}
