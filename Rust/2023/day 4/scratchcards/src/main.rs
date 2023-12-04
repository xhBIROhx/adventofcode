use std::fs;

// had a diffrent Idea in mind at first
use std::collections::HashMap;

struct ScratchCards {
    id: u32,
    points: u32
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("there should be an input.txt");
    
    let part1 = analyze_part1(&input);
    println!(" --- PART 2 ---");
    let part2 = analyze_part2(&input);
    println!("part1: {}\npart2: {}",part1,part2);
}

fn analyze_part1(input: &str) -> u32{
    let mut sum = 0;
    for line in input.lines(){
        println!("new line: {line}");
        let numbers: Vec<&str> = line.split(": ").collect();
        let numbers: Vec<&str> = numbers[1].split(" | ").collect();

        let mut winning_numbers: Vec<u32> = Vec::new();
        for winning_number_string in numbers[0].split_whitespace().collect::<Vec<&str>>(){
            //println!("to parse {winning_number_string}");
            winning_numbers.push(winning_number_string.parse().expect("should be numbers only"))
        }

        let mut points = 0;
        for rolled in numbers[1].split_whitespace().collect::<Vec<&str>>(){
            //println!("to parse: {rolled}");
            let rolled: u32 = rolled.parse().unwrap();
            if winning_numbers.contains(&rolled){
                println!("{winning_numbers:?} contained {rolled}");
                if points <2 {
                    points += 1
                }else{
                    points *= 2
                }
            }
        }
        sum += points
    }
    sum
}

fn analyze_part2(input: &str)  -> u32{
    let mut sum = 0;
    let mut cards = HashMap::new();

    for line in input.lines(){
        //println!("new line: {line}");
        let line: Vec<&str> = line.split(": ").collect();
        let numbers: Vec<&str> = line[1].split(" | ").collect();

        let game_id: u32 = line[0].split_whitespace().collect::<Vec<&str>>()[1].parse().expect("should be a number");

        let mut winning_numbers: Vec<u32> = Vec::new();
        for winning_number_string in numbers[0].split_whitespace().collect::<Vec<&str>>(){
            //println!("to parse {winning_number_string}");
            winning_numbers.push(winning_number_string.parse().expect("should be numbers only"))
        }

        let mut points = 0;
        for rolled in numbers[1].split_whitespace().collect::<Vec<&str>>(){
            //println!("to parse: {rolled}");
            let rolled: u32 = rolled.parse().unwrap();
            if winning_numbers.contains(&rolled){
                //println!("{winning_numbers:?} contained {rolled}");
                points += 1
            }
        }

        cards.insert(game_id, ScratchCards{id: game_id, points});
    }

    //sum += cards.len() as u32;
    for card in &cards.values().collect::<Vec<_>>() {
        add_copy_number(&mut sum, card, &cards);
    }
    sum
}

fn add_copy_number(amount: &mut u32, card: &ScratchCards, cards: &HashMap<u32,ScratchCards>){
    let id = card.id;
    let win = card.points;
    //println!("{id} won {win}");
    for new in id+1..=id+win{
        if let Some(card) = cards.get(&new){
            //println!("{id}. won the {}th", new);
            add_copy_number(amount, card, &cards)
        }
    }
    *amount += 1;
    //println!();
}