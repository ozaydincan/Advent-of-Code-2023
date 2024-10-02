use itertools::Itertools;

const INPUT: &str = include_str!("hands.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: i64,
    rank: HandKind,
}

impl Hand {
    fn hands_vec(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let (hand_str, bid_str) = line.split_at(5);
                let bid: i64 = bid_str.trim().parse().unwrap();
                let cards: [char; 5] = hand_str.chars().collect::<Vec<_>>().try_into().unwrap();

                let counts = cards
                    .iter()
                    .copied()
                    .counts()
                    .into_values()
                    .sorted()
                    .collect::<Vec<_>>();

                let hand_kind = match counts.as_slice() {
                    [1, 1, 1, 1, 1] => HandKind::HighCard,
                    [1, 1, 1, 2] => HandKind::OnePair,
                    [1, 2, 2] => HandKind::TwoPair,
                    [1, 1, 3] => HandKind::ThreeOfAKind,
                    [2, 3] => HandKind::FullHouse,
                    [1, 4] => HandKind::FourOfAKind,
                    [5] => HandKind::FiveOfAKind,
                    _ => unreachable!(),
                };

                Hand {
                    cards,
                    bid,
                    rank: hand_kind,
                }
            })
            .collect()
    }

    fn compare_cards(cards1: &[char; 5], cards2: &[char; 5]) -> std::cmp::Ordering {
        let cards_order = "23456789TJQKA";
        let values1: Vec<_> = cards1
            .iter()
            .map(|c| cards_order.find(*c).unwrap())
            .collect();
        let values2: Vec<_> = cards2
            .iter()
            .map(|c| cards_order.find(*c).unwrap())
            .collect();
        values1.cmp(&values2)
    }

    fn part_1(mut hands: Vec<Self>) -> i64 {
        hands.sort_by(|a, b| {
            if a.rank == b.rank {
                Self::compare_cards(&b.cards, &a.cards)
            } else {
                b.rank.cmp(&a.rank)
            }
        });

        hands
            .iter()
            .rev()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank as i64 + 1))
            .sum()
    }
}

fn main() {
    let input = INPUT;

    let hands = Hand::hands_vec(input);
    let sum = Hand::part_1(hands);

    println!("{}", sum);
}

#[cfg(test)]
mod test {
    use crate::Hand;

    const HANDS: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1() {
        let hands = Hand::hands_vec(HANDS);
        let sum = Hand::part_1(hands);

        println!("{}", sum);
        assert_eq!(6440, sum);
    }
}
