use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialOrd, PartialEq, Debug, Ord, Eq)]
enum Hand {
    // All hand sorted
    HighCard(Vec<u8>),
    // Pair and rest
    Pair(u8, Vec<u8>),
    // Pair, pair and kicker
    TwoPair(u8,u8,u8),
    // Three and rest
    ThreeOfAKind(u8, Vec<u8>),
    // High card
    Straight(u8),
    // All hand sorted
    Flush(Vec<u8>),
    // Three and Pair
    FullHouse(u8, u8),
    // Four and kicker
    FourOfAKind(u8,u8),
    // High card
    StraightFlush(u8),
    RoyalFlush
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: Suit
}

#[derive(Debug)]
enum CardParseError {
    UnknownSuit,
    UnknownRank,
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suit = match s.chars().next_back().unwrap() {
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            _ => Err(CardParseError::UnknownSuit)?,
        };
        let rank_slice = &s[..s.len() - 1];
        let rank = match (rank_slice, rank_slice.parse::<u8>()) {
            (_, Ok(rank)) => rank,
            ("J", Err(_)) => 11,
            ("Q", Err(_)) => 12,
            ("K", Err(_)) => 13,
            ("A", Err(_)) => 14,
            (_, _) => Err(CardParseError::UnknownRank)?,
        };
        Ok(Self { rank, suit })
    }
}

impl From<Vec<Card>> for Hand {
    fn from(mut cards: Vec<Card>) -> Self {
        cards.sort_unstable_by_key(|a| a.rank);
        let ranks = cards.iter().map(|card| card.rank).collect::<Vec<_>>();
        let suites = cards.iter().map(|card| card.suit).collect::<Vec<_>>();
        let mut ranks_with_counts = ranks_with_counts(ranks.as_slice());
        ranks_with_counts.sort_unstable_by(|(rank_a, count_a), (rank_b, count_b)| count_b.cmp(count_a).then(rank_b.cmp(rank_a)));
        let single_suit = all_single_suit(suites.iter());

        match (suites, ranks.as_slice(), single_suit) {
            // Flush
            (_, [10, 11, 12, 13, 14], true) => Hand::RoyalFlush,
            // StraightFlush (simple and starting with Ace)
            (_, ranks, true) if ranks.windows(2).all(|w| w[1] == w[0] + 1) => Hand::StraightFlush(ranks[ranks.len() - 1]),
            (_, [2,3,4,5,14], true) => Hand::StraightFlush(5),
            // Four of a kind
            (_, _, _) if ranks_with_counts[0].1 == 4 => Hand::FourOfAKind(ranks_with_counts[0].0,ranks_with_counts[1].0),
            // FullHouse
            (_, _, _) if ranks_with_counts[0].1 == 3 && ranks_with_counts[1].1 == 2 => Hand::FullHouse(ranks_with_counts[0].0,ranks_with_counts[1].0),
            // Flush
            (_, _, true) => Hand::Flush(ranks.into_iter().rev().collect()),
            // Straight (simple and starting with Ace)
            (_, ranks, _) if ranks.windows(2).all(|w| w[1] == w[0] + 1) => Hand::Straight(ranks[ranks.len() - 1]),
            (_, [2,3,4,5,14], _) => Hand::Straight(5),
            // Three of a kind
            (_, _, _) if ranks_with_counts[0].1 == 3 => Hand::ThreeOfAKind(ranks_with_counts[0].0, ranks.into_iter().rev().filter(|&r| r != ranks_with_counts[0].0).collect()),
            // Two pair
            (_, _, _) if ranks_with_counts[0].1 == 2 && ranks_with_counts[1].1 == 2 => Hand::TwoPair(ranks_with_counts[0].0,ranks_with_counts[1].0, ranks_with_counts[2].0),
            // Pair
            (_, _, _) if ranks_with_counts[0].1 == 2 => Hand::Pair(ranks_with_counts[0].0, ranks.into_iter().rev().filter(|&r| r != ranks_with_counts[0].0).collect()),
            // High card
            _ => Hand::HighCard(ranks.into_iter().rev().collect()),
        }
    }
}

fn all_single_suit<'a>(mut suits_iter: impl Iterator<Item = &'a Suit>) -> bool {
    if let Some(&first) = suits_iter.next() {
        return suits_iter.all(|&suit| first == suit)
    }
    false
}

fn ranks_with_counts(ranks: &[u8]) -> Vec<(u8, usize)> {
    let counts = ranks.iter().fold(HashMap::new(), |mut map, rank| {
        map.entry(*rank).and_modify(|v| *v += 1).or_insert(1);
        map
    });
    counts.into_iter().collect::<Vec<(_,_)>>()
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_ord =  hands.iter()
        .map(|hand| hand.split_whitespace()
            .map(Card::from_str)
            .collect::<Result<Vec<_>,_>>()
            .unwrap()
            .into())
        .enumerate()
        .collect::<Vec<(usize, Hand)>>();

    hands_ord.sort_unstable_by(|(_, hand_a), (_, hand_b)| hand_b.cmp(hand_a));
    hands_ord.iter().filter(|(_,hand)| *hand == hands_ord[0].1)
        .map(|(i, _)|hands[*i])
        .collect()
}
