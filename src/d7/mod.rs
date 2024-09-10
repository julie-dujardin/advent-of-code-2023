use std::collections::HashMap;
use std::fs;

const CARDS_SORTED1: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARDS_SORTED2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn parse_file(file_path: &str) -> Vec<(String, usize)> {
    let mut hands = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut groups = line.split(' ');
        hands.push((
            String::from(groups.next().unwrap()),
            groups.next().unwrap().parse().unwrap(),
        ))
    }
    hands
}

fn get_hand_type1(hand: &String) -> usize {
    let mut card_counts = HashMap::new();
    for card in hand.chars() {
        card_counts.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut card_counts_sorted: Vec<(&char, &usize)> = card_counts.iter().collect();
    card_counts_sorted.sort_by(|a, b| b.1.cmp(a.1));

    if *card_counts_sorted[0].1 == 5 {
        return 70; // Five of a kind
    }
    if *card_counts_sorted[0].1 == 4 {
        return 60; // Four of a kind
    }
    if *card_counts_sorted[0].1 == 3 {
        return if *card_counts_sorted[1].1 == 2 {
            50 // Full house
        } else {
            40 // Three of a kind
        };
    }
    if *card_counts_sorted[0].1 == 2 {
        return if *card_counts_sorted[1].1 == 2 {
            30 // Two pairs
        } else {
            20 // One pair
        };
    }
    10 // High card
}

fn evaluate_cards(hand: &String, card_order: [char; 13]) -> f64 {
    let mut tie_break = 0.;
    for (i, char) in hand.chars().enumerate() {
        tie_break += card_order.iter().position(|&r| r == char).unwrap() as f64
            * 10f64.powf(-((i + 1) as f64) * 2.);
    }
    tie_break
}

fn evaluate_hand1(hand: &String) -> f64 {
    get_hand_type1(hand) as f64 + evaluate_cards(hand, CARDS_SORTED1)
}

fn sum_hands(sorted_hands: Vec<(String, usize)>) -> usize {
    let mut winnings = 0;

    for (rank, (_, bet)) in sorted_hands.iter().enumerate() {
        winnings += bet * (rank + 1);
    }

    winnings
}

pub fn camels1(file_path: &str) -> usize {
    let mut hands = parse_file(file_path);

    hands.sort_unstable_by(|a, b| {
        evaluate_hand1(&a.0)
            .partial_cmp(&evaluate_hand1(&b.0))
            .unwrap() // we won't get any NaN so this is safe
    });

    sum_hands(hands)
}

fn get_hand_type2(hand: &String) -> usize {
    let mut card_counts = HashMap::new();
    for card in hand.chars() {
        card_counts.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    let joker_count = *card_counts.get(&'J').unwrap_or(&0);
    card_counts.remove(&'J');
    let mut card_counts_sorted: Vec<(&char, &usize)> = card_counts.iter().collect();
    card_counts_sorted.sort_by(|a, b| b.1.cmp(a.1));

    if joker_count == 5 || *card_counts_sorted[0].1 + joker_count >= 5 {
        return 70; // Five of a kind
    }
    if *card_counts_sorted[0].1 + joker_count >= 4 {
        return 60; // Four of a kind
    }
    if *card_counts_sorted[0].1 + joker_count >= 3 {
        // If we get to 3 of a kind with joker, we have no joker left to reach full house,
        // because we'd have already spent it on a four of a kind
        return if *card_counts_sorted[1].1 == 2 {
            50 // Full house
        } else {
            40 // Three of a kind
        };
    }
    if *card_counts_sorted[0].1 + joker_count >= 2 {
        return if *card_counts_sorted[1].1 == 2 {
            30 // Two pairs
        } else {
            20 // One pair
        };
    }
    10 // High card
}

fn evaluate_hand2(hand: &String) -> f64 {
    get_hand_type2(hand) as f64 + evaluate_cards(hand, CARDS_SORTED2)
}

pub fn camels2(file_path: &str) -> usize {
    let mut hands = parse_file(file_path);

    hands.sort_unstable_by(|a, b| {
        evaluate_hand2(&a.0)
            .partial_cmp(&evaluate_hand2(&b.0))
            .unwrap() // we won't get any NaN so this is safe
    });

    sum_hands(hands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::load_results;

    #[test]
    fn test_get_hand_type2() {
        assert_eq!(get_hand_type2(&String::from("JJJJJ")), 70);
        assert_eq!(get_hand_type2(&String::from("JJJ67")), 60);
    }

    #[test]
    fn p1() {
        let (expected_p1, _) = load_results("d7").unwrap();
        assert_eq!(
            camels1("test-data/d7/input_test1.txt"),
            expected_p1["input_test1"]
        );
        assert_eq!(camels1("test-data/d7/input.txt"), expected_p1["input"]);
    }

    #[test]
    fn p2() {
        let (_, expected_p2) = load_results("d7").unwrap();
        assert_eq!(
            camels2("test-data/d7/input_test1.txt"),
            expected_p2["input_test1"]
        );
        assert_eq!(camels2("test-data/d7/input.txt"), expected_p2["input"]);
    }
}
