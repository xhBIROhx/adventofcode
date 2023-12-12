use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("test_input.txt").expect("there should be an input.txt");

    //I don't know combinatorics, I have no idea how to do this properly, but "I'll be back"
    //I tried brute forcing, maybe it's computable.. it's not

    let part1 = analyze_part1(&input);
    //let part2 = analyze_part2(&input);
    println!("part1: {part1}");
    //println!("part2: {part2}");
}

fn analyze_part1(input: &str) -> u32 {
    let mut sum = 0;

    for lines in input.lines() {
        let parts = lines.split(' ').collect::<Vec<&str>>();
        let mut springs = parts[0].to_string();
        let numbers = parts[1].split(',').map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        //reduce spring AKA remove all ".."
        while springs.contains("..") {
            springs = springs.replace("..", ".");
        }
        println!("at: {springs}");

        //get combinations
        let combinations = all_possible(springs);
        for spring_combination in combinations {
            //if checks out += 1
            if correct_spring(&spring_combination, &numbers) {
                //println!("pos");
                sum +=1;
            }
        }
    }
    sum
    //0 //stfu rustc
}

fn all_possible(springs: String) -> Vec<String> {
    let (mut missing_index, mut return_strings) = (HashSet::new(),Vec::new());

    for (i,char) in springs.chars().enumerate() {
        if char == '?' {
            missing_index.insert(i);
        }
    }

    for i in 0..2_u32.pow(missing_index.len() as u32) {
        //binary counter:
        //println!("b: {i:b}");
        let mut binary = format!("{i:b}")
            .chars()
            .rev()
            .map(|f| if f == '0' {false} else {true})
            .collect::<Vec<bool>>();
        //fill the binary
        for _ in binary.len()..missing_index.len() {
            binary.push(false);
        }
        //println!("found bin: {binary:?}");
        let mut binary = binary.iter();

        let mut string = Vec::new();
        for i in 0..springs.len() {
            if missing_index.get(&i).is_none() { //not missing
                string.push(springs.chars().nth(i).unwrap())
            }else { // is a '?'
                if *binary.next().unwrap() { // if it was 1 AKA .
                    string.push('.');
                }else { //if it was 0 AKA #
                    string.push('#');
                }
            }
        }
        //println!("found: {}", string.iter().collect::<String>());
        return_strings.push(string.iter().collect::<String>());
    }

    return_strings
    //vec!["a".to_string()] //stfu rustc
}

fn correct_spring(spring: &str, check: &Vec<u32>) -> bool {
    let mut combination = Vec::new();

    let mut first = true;
    for spring in spring.chars() {
        if spring == '.'{
            first = true;
            continue;
        }else{
            //if #
            if first {
                combination.push(1 as u32);
                first = false;
            }else{
                let i = combination.len()-1;
                combination[i] +=1;
            }
        }
    }

    //println!("checking {spring} aginst {check:?}, got {combination:?}");
    &combination == check

    //false // stfu rustc
}

fn analyze_part2(input: &str) -> u64 {
    let mut sum = 0;

    for lines in input.lines() {
        
        let parts = lines.split(' ').collect::<Vec<&str>>();
        let mut springs = parts[0].to_string();
        let mut numbers = parts[1].split(',').map(|f| f.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        
        //reduce spring AKA remove all ".."
        while springs.contains("..") {
            springs = springs.replace("..", ".");
        }


        //make the 5 copies thing, ah this is run forever.
        for _ in 0..4 { //numbers thing
            numbers.extend(numbers.clone());
        }
        for _ in 0..4 {
            springs += &("?".to_owned()+&springs)
        }

        println!("at: {springs} {numbers:?}");

        //get combinations
        let combinations = all_possible_p2(springs);
        for spring_combination in combinations {
            //if checks out += 1
            if correct_spring_p2(&spring_combination, &numbers) {
                //println!("pos");
                sum +=1;
            }
        }
    }
    sum
    //0 //stfu rustc
}

fn all_possible_p2(springs: String) -> Vec<String> {
    let (mut missing_index, mut return_strings) = (HashSet::new(),Vec::new());

    for (i,char) in springs.chars().enumerate() {
        if char == '?' {
            missing_index.insert(i);
        }
    }

    for i in 0..2_u128.pow(missing_index.len() as u32) {
        //binary counter:
        //println!("b: {i:b}");
        let mut binary = format!("{i:b}")
            .chars()
            .rev()
            .map(|f| if f == '0' {false} else {true})
            .collect::<Vec<bool>>();
        //fill the binary
        for _ in binary.len()..missing_index.len() {
            binary.push(false);
        }
        //println!("found bin: {binary:?}");
        let mut binary = binary.iter();

        let mut string = Vec::new();
        for i in 0..springs.len() {
            if missing_index.get(&i).is_none() { //not missing
                string.push(springs.chars().nth(i).unwrap())
            }else { // is a '?'
                if *binary.next().unwrap() { // if it was 1 AKA .
                    string.push('.');
                }else { //if it was 0 AKA #
                    string.push('#');
                }
            }
        }
        //println!("found: {}", string.iter().collect::<String>());
        return_strings.push(string.iter().collect::<String>());
    }

    return_strings
    //vec!["a".to_string()] //stfu rustc
}

fn correct_spring_p2(spring: &str, check: &Vec<u64>) -> bool {
    let mut combination = Vec::new();

    let mut first = true;
    for spring in spring.chars() {
        if spring == '.'{
            first = true;
            continue;
        }else{
            //if #
            if first {
                combination.push(1 as u64);
                first = false;
            }else{
                let i = combination.len()-1;
                combination[i] +=1;
            }
        }
    }

    //println!("checking {spring} aginst {check:?}, got {combination:?}");
    &combination == check

    //false // stfu rustc
}
