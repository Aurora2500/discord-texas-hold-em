use super::*;

type R = Rank;
type S = Suit;

#[test]
fn orderinng() {
    assert!(CC::RoyalFlush > CC::StraightFlush(R::Six));
    assert!(CC::StraightFlush(R::Jack) > CC::StraightFlush(R::Six));
    assert!(
        CC::Flush(R::Jack, R::Seven, R::Four, R::Three, R::Two)
        >
        CC::Flush(R::Jack, R::Six, R::Five, R::Four, R::Three)
    );

    assert!(CC::Pair(R::Seven, R::Six) < CC::Pair(R::Eight, R::Four));
    assert!(CC::Pair(R::Seven, R::Four) < CC::Pair(R::Seven, R::Six));
    assert!(CC::Pair(R::Seven, R::Four) == CC::Pair(R::Seven, R::Four));
}

#[cfg(test)]
mod combos {
    use super::*;
    #[test]
    fn royal_flush() {
        let cards = [
            Card(S::Clubs, R::Four),
            Card(S::Hearts, R::Jack),
            Card(S::Hearts, R::King),
            Card(S::Spades, R::Three),
            Card(S::Hearts, R::Ace),
            Card(S::Hearts, R::Queen),
            Card(S::Hearts, R::Ten),
        ];
        assert_eq!(Some(CC::RoyalFlush), find_royal_flush(&cards));
    }

    #[test]
    fn straight_flush() {
        let cards = [
            Card(S::Hearts, R::Eight),
            Card(S::Hearts, R::Jack),
            Card(S::Hearts, R::Nine),
            Card(S::Spades, R::Three),
            Card(S::Spades, R::Ace),
            Card(S::Hearts, R::Queen),
            Card(S::Hearts, R::Ten),
        ];
        assert_eq!(Some(CC::StraightFlush(R::Queen)), find_straight_flush(&cards));
    }

    #[test]
    fn four_of_a_kind() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Eight),
            Card(S::Clubs, R::Nine),
            Card(S::Diamonds, R::Eight),
            Card(S::Clubs, R::Four),
            Card(S::Diamonds, R::Nine),
            Card(S::Spades, R::Eight),
        ];
        assert_eq!(Some(CC::FourOfAKind(R::Eight, R::Nine)), find_four_of_a_kind(&cards))
    }

    #[test]
    fn full_house() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Eight),
            Card(S::Clubs, R::Nine),
            Card(S::Diamonds, R::Eight),
            Card(S::Clubs, R::Four),
            Card(S::Diamonds, R::Nine),
            Card(S::Hearts, R::Seven),
        ];
        assert_eq!(Some(CC::FullHouse(R::Eight)), find_full_house(&cards));
    }

    #[test]
    fn flush() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Three),
            Card(S::Clubs, R::Nine),
            Card(S::Diamonds, R::Two),
            Card(S::Clubs, R::Four),
            Card(S::Clubs, R::Six),
            Card(S::Clubs, R::Seven),
        ];
        assert_eq!(Some(CC::Flush(R::Nine, R::Eight, R::Seven, R::Six, R::Four)), find_flush(&cards));
    }

    #[test]
    fn straight() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Nine),
            Card(S::Spades, R::Seven),
            Card(S::Diamonds, R::Nine),
            Card(S::Hearts, R::Five),
            Card(S::Diamonds, R::Six),
            Card(S::Hearts, R::Three),
        ];
        assert_eq!(Some(CC::Straight(R::Nine)), find_straight(&cards));
    }

    #[test]
    fn three_of_a_kind() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Eight),
            Card(S::Spades, R::Eight),
            Card(S::Diamonds, R::Nine),
            Card(S::Hearts, R::Four),
            Card(S::Diamonds, R::Six),
            Card(S::Hearts, R::Three),
        ];
        assert_eq!(Some(CC::ThreeOfAKind(R::Eight, R::Nine)), find_three_of_a_kind(&cards));

    }

    #[test]
    fn two_pairs() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Three),
            Card(S::Spades, R::Nine),
            Card(S::Diamonds, R::Nine),
            Card(S::Hearts, R::Four),
            Card(S::Diamonds, R::Six),
            Card(S::Hearts, R::Eight),
        ];
        assert_eq!(Some(CC::TwoPairs(R::Nine, R::Eight, R::Six)), find_two_pairs(&cards));
    }

    #[test]
    fn pair() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Three),
            Card(S::Spades, R::Nine),
            Card(S::Diamonds, R::Two),
            Card(S::Hearts, R::Four),
            Card(S::Diamonds, R::Six),
            Card(S::Hearts, R::Eight),
        ];
        assert_eq!(Some(CC::Pair(R::Eight, R::Nine)), find_pair(&cards));
    }

    #[test]
    fn highest_card() {
        let cards = [
            Card(S::Clubs, R::Eight),
            Card(S::Hearts, R::Three),
            Card(S::Spades, R::Nine),
            Card(S::Diamonds, R::Two),
            Card(S::Hearts, R::Four),
            Card(S::Diamonds, R::Six),
            Card(S::Clubs, R::Seven),
        ];
        assert_eq!(CC::HighCard(R::Nine, R::Eight, R::Seven, R::Six, R::Four), find_high_card(&cards)) 
    }
}
