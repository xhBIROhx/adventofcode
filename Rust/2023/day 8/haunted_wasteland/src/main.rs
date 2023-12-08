use std::{fs, collections::HashMap};


fn main() {
    let input = fs::read_to_string("input.txt").expect("there should be an input.txt");

    //I had a bunch of testing functions and everything, but they didn't work so I deleted them

    let part1 = analyze_part1(&input);
    let part2 = analyze_part2(&input);

    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn analyze_part1(input: &str) -> u32 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    
    //sequence --- RIGHT=TRUE, LEFT=FALSE
    let sequence: Vec<bool> = parts[0].chars().map(|f| {
        if f == 'L' {
            false
        }else if f == 'R' {
            true
        }else {
            panic!("found \'{f}\' in the L/R sequence")
        }
    }).collect();

    //fancy debug
    //println!("{}\n{}",parts[0], format!("{:?}",sequence.iter().map(|f|if *f {'R'}else{'L'}).collect::<Vec<char>>()).replace("', '", "").replace("['", ""));
    
    //map
    let mut map = HashMap::new();
    for line in parts[1].lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        //parts[0] is the start

        let destinations: Vec<String> = parts[1].replace('(', "").replace(')', "").split(", ").map(|f| f.to_string()).collect();

        map.insert(parts[0].to_string(), destinations);
    }


    //to the sequence
    //let mut current_position = parts[1].lines().next().expect("there should be first line").split(" = ").collect::<Vec<&str>>()[0];
    let mut current_position = "AAA";
    let mut i: usize = 0;
    let mut res = 1;
    loop {
        let current_move = sequence[i];
        let current_options = match map.get(current_position) {
            Some(t) => t,
            None => panic!("This location wasn't found")
        };

        //get where I moved
        let next_position ;
        if current_move { //0left 1right
            next_position = &current_options[1];
            //println!("moving right");
        }else{
            next_position = &current_options[0];
            //println!("moving left");
        }

        //check if it's the end
        //println!("{current_position}, moved {current_move} to {next_position}");
        if next_position == "ZZZ"{
            //end
            break;
        }else{
            current_position = next_position;
            //println!("we're at {current_position}");
        }

        i+=1;
        res+=1;
        if i == sequence.len() {i=0}
    }
    res
}

fn analyze_part2(input: &str) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    
    //sequence --- RIGHT=TRUE, LEFT=FALSE
    let sequence: Vec<bool> = parts[0].chars().map(|f| {
        if f == 'L' {
            false
        }else if f == 'R' {
            true
        }else {
            panic!("found \'{f}\' in the L/R sequence")
        }
    }).collect();

    //fancy debug
    //println!("{}\n{}",parts[0], format!("{:?}",sequence.iter().map(|f|if *f {'R'}else{'L'}).collect::<Vec<char>>()).replace("', '", "").replace("['", ""));
    
    //map
    let mut map = HashMap::new();
    let mut starting_positions = Vec::new();
    for line in parts[1].lines() {
        let parts: Vec<&str> = line.split(" = ").collect();
        //parts[0] is the start
        if parts[0].ends_with('A') {
            starting_positions.push(parts[0])
        }
        let temp = &parts[1][1..][..parts[1].len()-2];
        let destinations: Vec<&str> = temp.split(", ").collect();
        //println!("{destinations:?}");

        map.insert(parts[0], destinations);
    }


    let mut destination_positions= Vec::new();
    for starting_position in starting_positions {

        let mut des_posses = 0;
        let mut current_position = starting_position;
        let mut i: usize = 0;
        let mut res = 1;
        
        loop {
            let current_move = sequence[i];
            let current_options = match map.get(current_position) {
                Some(t) => t,
                None => panic!("This location wasn't found")
            };

            //get where I moved
            if current_move { //0left 1right
                current_position = &current_options[1];
                //println!("moving right");
            }else{
                current_position = &current_options[0];
                //println!("moving left");
            }
            

            //check if it's one of the possible ends, or we looped back perfectly
            if current_position.chars().last().unwrap() == 'Z'{
                //println!("found {res}/{current_position} as winnig pos for the starting value of: {starting_position}");
                des_posses = res;
            }
            if des_posses != 0 {
                destination_positions.push(des_posses);
                break;
            }

            i+=1;
            res+=1;
            if i == sequence.len() {i=0}
        }
    }

    smallest_common_multiple(destination_positions)
}

fn smallest_common_multiple(numbers: Vec<u32>) -> u64{
    let mut max = numbers[0];
    for number in numbers[1..].iter(){
        if number > &max {
            max = *number;
        }
    }
    let max = max;

    //quick and dirty prime number finder
    let mut prime_numbers = Vec::new();
    'main: for potential_prime in 2..=max {
        for prime in &prime_numbers {
            if potential_prime % prime == 0 {
                continue 'main;
            }
        }
        prime_numbers.push(potential_prime);
    }

    let mut prime_map: HashMap<u32,u32> = HashMap::new();
    
    //factorization function
    for number in numbers {
        let mut numbers_primes = HashMap::new();
        prime_factorization(&prime_numbers, &mut numbers_primes, number);
        //println!("{number} factors: {numbers_primes:?}");

        for prime in numbers_primes {
            let entry = prime_map.entry(prime.0).or_insert(prime.1);
            if *entry < prime.1 {
                *entry = prime.1
            }
        }
    }

    println!("combined factors: {:?}", prime_map);
    let mut smc = 1;
    for (base,power) in prime_map {
        smc *= base.pow(power) as u64;
    }

    smc
}

fn prime_factorization(primes: &Vec<u32>,found_primes: &mut HashMap<u32,u32>, number: u32) {
    for prime in primes {
        if number % prime == 0 {
            let entry = found_primes.entry(*prime).or_insert(0);
            *entry += 1;
            let number = number / prime;

            if number <= 1{return;} //it's done

            return prime_factorization(primes, found_primes, number);
        }
    }

}