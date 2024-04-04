use aoc::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumFromChar)]
enum Card {
    Joker,
    #[char = '2'] Two,
    #[char = '3'] Three,
    #[char = '4'] Four,
    #[char = '5'] Five,
    #[char = '6'] Six,
    #[char = '7'] Seven,
    #[char = '8'] Eight,
    #[char = '9'] Nine,
    #[char = 'T'] Ten,
    #[char = 'J'] Jack,
    #[char = 'Q'] Queen,
    #[char = 'K'] King,
    #[char = 'A'] Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Hand {
        Hand { cards }
    }

    pub fn jacks_to_jokers(self) -> Hand {
        let mut cards = self.cards;
        for card in &mut cards {
            if *card == Card::Jack { *card = Card::Joker }
        }
        Hand::new(cards)
    }

    pub fn hand_type(&self) -> HandType {
        let mut rank_counts = [0; 14];
        for card in &self.cards {
            rank_counts[*card as usize] += 1;
        }
        let jokers = rank_counts[0];
        let rank_counts = &mut rank_counts[1..];
        rank_counts.sort();
        let mut count_iter = rank_counts.iter().rev();
        // Hands are always improved by counting jokers as the most common card
        let highest = count_iter.next().unwrap() + jokers;
        let next_highest = *count_iter.next().unwrap();
        match (highest, next_highest) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type_cmp = self.hand_type().partial_cmp(&other.hand_type());
        match hand_type_cmp {
            Some(std::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            _ => hand_type_cmp
        }
    }
}

fn parse_line(line: &str) -> (Hand, u64) {
    let (hand, bet) = line.split_once(' ').unwrap();
    let cards: Vec<_> = hand.chars().map(|c| c.try_into().unwrap()).collect();
    let bet = bet.parse().unwrap();
    (Hand::new(cards.try_into().unwrap()), bet)
}

struct Day07;
impl Problem for Day07 {
    type Solution = u64;

    fn part_1(input: &str) -> Self::Solution {
        let mut bets: Vec<_> = input.lines()
            .map(parse_line)
            .collect();
        bets.sort_by_key(|(hand, _)| hand.clone());
        bets.iter()
            .enumerate()
            .map(|(i, (_hand, bet))| (i as u64 + 1) * bet )
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        let mut bets: Vec<_> = input.lines()
            .map(|line| {
                let (hand, bet) = parse_line(line);
                (hand.jacks_to_jokers(), bet)
            })
            .collect();
        bets.sort_by_key(|(hand, _)| hand.clone());
        bets.iter()
            .enumerate()
            .map(|(i, (_hand, bet))| (i as u64 + 1) * bet )
            .sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day07::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483";

    test_part_1!(Day07, SAMPLE, 6440);
    test_part_2!(Day07, SAMPLE, 5905);
}