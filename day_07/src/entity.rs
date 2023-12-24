use std::cmp::{max_by_key, Ordering};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Joker,
    Number(u8),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

#[derive(Debug)]
pub struct Bidder {
    pub hand: Hand,
    pub bid: u32,
}

impl Card {
    pub fn strength(&self) -> u8 {
        match *self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Joker => 0,
            Card::Number(val) => val,
        }
    }
}

impl Hand {
    pub fn new(cards: [Card; 5], enable_joker: bool) -> Hand {
        let hand_type = if enable_joker {
            determine_hand_type(&convert_jokers(cards))
        } else {
            determine_hand_type(&cards)
        };
        Hand { cards, hand_type }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let type_ordering = self.hand_type.cmp(&other.hand_type);
        if type_ordering != Ordering::Equal {
            return type_ordering;
        }

        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let card_ordering = card.cmp(other_card);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn convert_jokers(mut cards: [Card; 5]) -> [Card; 5] {
    let mut counts: HashMap<Card, u8> = HashMap::new();
    for card in cards {
        *counts.entry(card).or_insert(0) += 1;
    }
    let jokers = counts.remove(&Card::Joker).unwrap_or(0);
    if jokers == 0 {
        return cards;
    }

    let card_to_replace_with = find_most_common_highest_non_joker_card(counts);
    for i in 0..5 {
        if cards[i] == Card::Joker {
            cards[i] = card_to_replace_with;
        }
    }

    cards
}

fn find_most_common_highest_non_joker_card(counts: HashMap<Card, u8>) -> Card {
    counts.into_iter()
        .fold((Card::Ace, 0), |current @ (card, count), other @ (other_card, other_count)| {
            if card == other_card {
                return (card, count + other_count);
            }

            if count > other_count {
                current
            } else if other_count > count {
                other
            } else {
                max_by_key(current, other, |(card, _)| {
                    card.strength()
                })
            }
        }).0
}

fn determine_hand_type(cards: &[Card; 5]) -> HandType {
    let mut counts = [0; 15];
    for card in cards {
        counts[card.strength() as usize] += 1;
    }

    let mut pairs = 0;
    let mut three_of_a_kind = false;
    for i in 2..15 {
        let count = counts[i];
        match count {
            2 => pairs += 1,
            3 => three_of_a_kind = true,
            4 => return HandType::FourOfAKind,
            5 => return HandType::FiveOfAKind,
            _ => (),
        }
    }

    if three_of_a_kind && pairs == 1 {
        HandType::FullHouse
    } else if three_of_a_kind {
        HandType::ThreeOfAKind
    } else if pairs == 2 {
        HandType::TwoPairs
    } else if pairs == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}
