use std::fs;
use std::collections::HashMap;

trait StringParse {
    fn poker_parse(&self) -> Vec<u32>;
    fn poker_parse_part2(&self) -> Vec<u32>;
}

trait ScoreParse {
    fn poker_hand_parse(&self) -> u32;
    fn poker_hand_parse_part2(&self) -> u32;
}

impl StringParse for str {
    fn poker_parse(&self) -> Vec<u32> {
        let mut hand = Vec::new(); 

        for char in self.chars() {
            hand.push(match char.to_digit(10) {
                Some(t) => t,
                _ => match char { // A, K, Q, J, T
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("tried to parse \'{char}\' to poker")
                }
            })
        }

        hand
    }

    fn poker_parse_part2(&self) -> Vec<u32> {
        let mut hand = Vec::new(); 

        for char in self.chars() {
            hand.push(match char.to_digit(10) {
                Some(t) => t,
                None => match char { // A, K, Q, T .. J
                    'J' => 1,
                    'T' => 10,
                    'Q' => 11,
                    'K' => 12,
                    'A' => 13,
                    _ => panic!("tried to parse \'{char}\' to poker")
                }
            })
        }

        hand
    }
}

impl ScoreParse for Vec<u32> {
    fn poker_hand_parse(&self) -> u32 {
            let mut map: HashMap<&u32, u32> = HashMap::new();
            /* 
            6:Five of a kind
            5:Four of a kind
            4:Full house
            3:Three of a kind
            2:Two pair
            1:One pair
            0:High card
            */
    
            for card in self {
                let card_amount = map.entry(card).or_insert(0);
                *card_amount += 1;
            }
    
            let parts: Vec<(&u32, u32)> = map.drain().collect();

            //logic based on len
            match parts.len() {
                1 => return 6,
                2 => {
                    if parts[0].1 == 1 || parts[0].1 == 4 {
                        return 5
                    }else{
                        return 4
                    }
                },
                3 => {
                    for part in &parts[1..]{
                        if part.1 == 2 {
                            return 2
                        }
                    }
                    return 3;
                },
                4 => return 1,
                5 => return 0,
                _ => panic!("the hand had {} unique cards, out of 5 total cards",parts.len())
            }
    }

    fn poker_hand_parse_part2(&self) -> u32 {
        let mut map: HashMap<&u32, u32> = HashMap::new();
        /* 
        6:Five of a kind
        5:Four of a kind
        4:Full house
        3:Three of a kind
        2:Two pair
        1:One pair
        0:High card
        */

        for card in self {
            let card_amount = map.entry(card).or_insert(0);
            *card_amount += 1;
        }

        let jokers = match map.get(&1) {
            Some(t) => *t,
            None => 0
        };

        //println!("hand: {self:?} was hashed to {map:?} with {jokers} jokers");

        let parts: Vec<(&u32, u32)> = map.drain().collect();

        //if there is A joker it must be:
            //based on len:
            //1: five of a kind
            //2: five of a kind
            //3:
                //max=2:
                    //jokers=2: four of a kinf
                    //jokers=1: full house
                //max=3: four of a kind
            //4: three of kind
            //5: one pair
        
        if jokers == 0 {
            match parts.len() {
                1 => return 6,
                2 => {
                    if parts[0].1 == 1 || parts[0].1 == 4 {
                        return 5
                    }else{
                        return 4
                    }
                },
                3 => {
                    for part in &parts[1..]{
                        if part.1 == 2 {
                            return 2
                        }
                    }
                    return 3;
                },
                4 => return 1,
                5 => return 0,
                _ => panic!("the hand had {} unique cards, out of 5 total cards",parts.len())
            }
        } else {
            match parts.len() { // -amount of J's EXEPTION when all 5 are J's
                1 => return 6,
                2 => return 6,
                3 => {
                    for part in &parts[1..]{
                        if part.1 == 2 && jokers == 1{
                            return 4
                        }
                    }
                    return 5;
                },
                4 => return 3,
                5 => return 1,
                _ => panic!("the hand had {} unique cards, out of 5 total cards",parts.len())
            }
        }
        
    }
}

#[derive(Debug,Clone)]
struct Hand {
    hand: Vec<u32>,
    bid: u32,
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("These should be an input.txt");

    let part1 = analyze_part1(&input);
    let part2 = analyze_part2(&input);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn analyze_part1(input: &str) -> u32 {
    let mut games = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        let hand = parts[0].poker_parse();
        let bid: u32 = parts[1].parse().expect("should be correctly formatted");
        let score = hand.poker_hand_parse();

        //save all
        let entry = games.entry(score).or_insert(vec![]);
        entry.push(Hand { hand, bid });
    }

    let mut sorted_games = Vec::new();

    for i in 0..=6 { 
        let game = match games.get(&i) {
            Some(t) => t,
            None => continue // not found
        };

        if game.len() == 1 {//if only one of this kind return it, happy days
            sorted_games.push(game[0].clone())
        }else{//if more
            sort_hands(&mut sorted_games, game, 0);
        }
    }

    //fancy debug
    //println!("{}", &format!("{:?}",sorted_games).replace(", Hand ", "\n")[5..].replace(']', ""));

    let mut sum = 0;
    for (i,sorted_games) in sorted_games.iter().enumerate() {
        sum += sorted_games.bid * (i as u32 +1);
    }
    
    sum
}

fn sort_hands(sorted_games: &mut Vec<Hand>, hand: &Vec<Hand>, digit_deep: usize){
    let mut map = HashMap::new();
    for hand in hand {
        let entry = map.entry(hand.hand[digit_deep]).or_insert(vec![]);
        entry.push(hand.clone());
    }

    for i in 1..=14 as u32 {
        let hand = match map.get(&i) {
            Some(t) => t,
            None => continue // not found
        };

        if hand.len() == 1 {//if only one of this kind return it, happy days
            sorted_games.push(hand[0].clone())
        }else{//if more
            sort_hands(sorted_games, hand, digit_deep+1);
        }
    }
}

fn analyze_part2(input: &str) -> u32 {
    let mut games = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        let hand = parts[0].poker_parse_part2();
        let bid: u32 = parts[1].parse().expect("should be correctly formatted");
        let score = hand.poker_hand_parse_part2();

        //save all
        let entry = games.entry(score).or_insert(vec![]);
        entry.push(Hand { hand, bid });
    }

    let mut sorted_games = Vec::new();

    for i in 0..=6 { 
        let game = match games.get(&i) {
            Some(t) => t,
            None => continue // not found
        };

        if game.len() == 1 {//if only one of this kind return it, happy days
            sorted_games.push(game[0].clone())
        }else{//if more
            sort_hands(&mut sorted_games, game, 0);
        }
    }

    //fancy debug
    println!("{}", &format!("{:?}",sorted_games).replace(", Hand ", "\n")[5..].replace(']', ""));

    let mut sum = 0;
    for (i,sorted_games) in sorted_games.iter().enumerate() {
        sum += sorted_games.bid * (i as u32 +1);
    }
    
    sum
}

