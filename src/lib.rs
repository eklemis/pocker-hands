use std::cmp::Ordering;
/// Struct representation of a single hand
enum CardRank {
    ROYAL_FLUSH = 1,
    STRAIGHT_FLUSH = 2,
    FOUR_OF_A_KIND = 3,
    FULL_HOUSE = 4,
    FLUSH = 5,
    STRAIGHT = 6,
    THREE_OF_A_KIND = 7,
    TWO_PAIRS = 8,
    ONE_PAIR = 9,
    HIGH_CARD = 10,
}

struct Hand {
    origin: String,
    origin_translated: Vec<Card>,
    ordered: Vec<Card>,
    ranked_card: Vec<Card>,
    rank: CardRank,
}
impl Hand {
    pub fn new() {}
}

#[derive(Eq)]
struct Card {
    //the card string representation. example:4S or 5H
    str_rep: String,
    value: u8,
}
impl Card {
    fn new(str_rep: &str) -> Self {
        let left_str = &str_rep[0..str_rep.len() - 1];
        let number = match left_str {
            "A" => 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            _ => left_str.parse().unwrap(),
        };
        Card {
            str_rep: String::from(left_str),
            value: number,
        }
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }
    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }
    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }
    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut win_hands: Vec<&str> = Vec::new();
    //Single hand always win
    if hands.len() == 1 {
        win_hands.push(hands[0]);
        return win_hands;
    }
    unimplemented!("Out of {:?}, which hand wins?", hands)
}

#[cfg(test)]
mod tests {
    use super::winning_hands;
    use std::collections::HashSet;
    fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
        let mut hs = HashSet::new();
        for item in input.iter() {
            hs.insert(*item);
        }
        hs
    }
    /// Test that the expected output is produced from the given input
    /// using the `winning_hands` function.
    ///
    /// Note that the output can be in any order. Here, we use a HashSet to
    /// abstract away the order of outputs.
    fn test<'a, 'b>(input: &[&'a str], expected: &[&'b str]) {
        assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
    }
    #[test]
    fn test_single_hand_always_wins() {
        test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
    }
    #[test]
    #[ignore]
    fn test_duplicate_hands_always_tie() {
        let input = &["3S 4S 5D 6H JH", "3S 4S 5D 6H JH", "3S 4S 5D 6H JH"];
        assert_eq!(&winning_hands(input), input)
    }
    #[test]
    #[ignore]
    fn test_highest_card_of_all_hands_wins() {
        test(
            &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"],
            &["3S 4S 5D 6H JH"],
        )
    }
    #[test]
    #[ignore]
    fn test_a_tie_has_multiple_winners() {
        test(
            &[
                "4D 5S 6S 8D 3C",
                "2S 4C 7S 9H 10H",
                "3S 4S 5D 6H JH",
                "3H 4H 5C 6C JD",
            ],
            &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"],
        )
    }
    #[test]
    #[ignore]
    fn test_high_card_can_be_low_card_in_an_otherwise_tie() {
        // multiple hands with the same high cards, tie compares next highest ranked,
        // down to last card
        test(&["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"], &["3S 5H 6S 8D 7H"])
    }
    #[test]
    #[ignore]
    fn test_one_pair_beats_high_card() {
        test(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"])
    }
    #[test]
    #[ignore]
    fn test_highest_pair_wins() {
        test(&["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"], &["2S 4H 6C 4D JD"])
    }
    #[test]
    #[ignore]
    fn test_two_pairs_beats_one_pair() {
        test(&["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"], &["4S 5H 4C 8C 5C"])
    }
    #[test]
    #[ignore]
    fn test_two_pair_ranks() {
        // both hands have two pairs, highest ranked pair wins
        test(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])
    }
    #[test]
    #[ignore]
    fn test_two_pairs_second_pair_cascade() {
        // both hands have two pairs, with the same highest ranked pair,
        // tie goes to low pair
        test(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])
    }
    #[test]
    #[ignore]
    fn test_two_pairs_last_card_cascade() {
        // both hands have two identically ranked pairs,
        // tie goes to remaining card (kicker)
        test(&["JD QH JS 8D QC", "JS QS JC 2D QD"], &["JD QH JS 8D QC"])
    }
    #[test]
    #[ignore]
    fn test_three_of_a_kind_beats_two_pair() {
        test(&["2S 8H 2H 8D JH", "4S 5H 4C 8S 4H"], &["4S 5H 4C 8S 4H"])
    }
    #[test]
    #[ignore]
    fn test_three_of_a_kind_ranks() {
        //both hands have three of a kind, tie goes to highest ranked triplet
        test(&["2S 2H 2C 8D JH", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
    }
    #[test]
    #[ignore]
    fn test_three_of_a_kind_cascade_ranks() {
        // with multiple decks, two players can have same three of a kind,
        // ties go to highest remaining cards
        test(&["4S AH AS 7C AD", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
    }
    #[test]
    #[ignore]
    fn test_straight_beats_three_of_a_kind() {
        test(&["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"], &["3S 4D 2S 6D 5C"])
    }
    #[test]
    #[ignore]
    fn test_aces_can_end_a_straight_high() {
        // aces can end a straight (10 J Q K A)
        test(&["4S 5H 4C 8D 4H", "10D JH QS KD AC"], &["10D JH QS KD AC"])
    }
    #[test]
    #[ignore]
    fn test_aces_can_end_a_straight_low() {
        // aces can start a straight (A 2 3 4 5)
        test(&["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"], &["4D AH 3S 2D 5C"])
    }
    #[test]
    #[ignore]
    fn test_straight_cascade() {
        // both hands with a straight, tie goes to highest ranked card
        test(&["4S 6C 7S 8D 5H", "5S 7H 8S 9D 6H"], &["5S 7H 8S 9D 6H"])
    }
    #[test]
    #[ignore]
    fn test_straight_scoring() {
        // even though an ace is usually high, a 5-high straight is the lowest-scoring straight
        test(&["2H 3C 4D 5D 6H", "4S AH 3S 2D 5H"], &["2H 3C 4D 5D 6H"])
    }
    #[test]
    #[ignore]
    fn test_flush_beats_a_straight() {
        test(&["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"], &["2S 4S 5S 6S 7S"])
    }
    #[test]
    #[ignore]
    fn test_flush_cascade() {
        // both hands have a flush, tie goes to high card, down to the last one if necessary
        test(&["4H 7H 8H 9H 6H", "2S 4S 5S 6S 7S"], &["4H 7H 8H 9H 6H"])
    }
    #[test]
    #[ignore]
    fn test_full_house_beats_a_flush() {
        test(&["3H 6H 7H 8H 5H", "4S 5C 4C 5D 4H"], &["4S 5C 4C 5D 4H"])
    }
    #[test]
    #[ignore]
    fn test_full_house_ranks() {
        // both hands have a full house, tie goes to highest-ranked triplet
        test(&["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 8S 8D"])
    }
    #[test]
    #[ignore]
    fn test_full_house_cascade() {
        // with multiple decks, both hands have a full house with the same triplet, tie goes to the pair
        test(&["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 9S 9D"])
    }
    #[test]
    #[ignore]
    fn test_four_of_a_kind_beats_full_house() {
        test(&["4S 5H 4D 5D 4H", "3S 3H 2S 3D 3C"], &["3S 3H 2S 3D 3C"])
    }
    #[test]
    #[ignore]
    fn test_four_of_a_kind_ranks() {
        // both hands have four of a kind, tie goes to high quad
        test(&["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"], &["4S 5H 5S 5D 5C"])
    }
    #[test]
    #[ignore]
    fn test_four_of_a_kind_cascade() {
        // with multiple decks, both hands with identical four of a kind, tie determined by kicker
        test(&["3S 3H 2S 3D 3C", "3S 3H 4S 3D 3C"], &["3S 3H 4S 3D 3C"])
    }
    #[test]
    #[ignore]
    fn test_straight_flush_beats_four_of_a_kind() {
        test(&["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"], &["7S 8S 9S 6S 10S"])
    }
    #[test]
    #[ignore]
    fn test_straight_flush_ranks() {
        // both hands have straight flush, tie goes to highest-ranked card
        test(&["4H 6H 7H 8H 5H", "5S 7S 8S 9S 6S"], &["5S 7S 8S 9S 6S"])
    }
}
