use std::fs;

trait Calc {
    fn sum(self) -> i32;
    fn seq_div(self) -> i32;
}

impl Calc for Vec<i32> {
    fn sum(self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum +=i;
        }
        sum
    }

    fn seq_div(self) -> i32 {
        let mut vector = self;
        vector.reverse();
        let mut div = vector[0];
        for i in &vector[1..] {
            println!("{i}-{div}={}",i-div);
            div = i-div;
        }
        div
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("there should be an input.txt");

    let part1 = analyze_part1(&input);
    let part2 = analyze_part2(&input);
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn analyze_part1(input: &str) -> i32{
    let mut sum = 0;
    
    for line in input.lines() {
        let mut numbers: Vec<i32> = line.split(' ').map(|f| f.parse::<i32>().expect("should be correctly formatted")).collect();
        let mut last_numbers: Vec<i32> = Vec::new();

        loop {
            last_numbers.push(*numbers.last().unwrap());
            numbers = sub_sequence(numbers);

            //search for non 0, meaning we need to continue
            if only_zeros(&numbers) {
                break;
            }
        }

        println!("last numbers: {last_numbers:?}");
        let extrapolated = last_numbers.sum();
        println!("extrapolated value: {extrapolated}");
        sum += extrapolated;
    }

    sum  
}

fn sub_sequence(prev_numbers: Vec<i32>) -> Vec<i32>{
    let mut new_numbers = Vec::new();
    for i in 0..prev_numbers.len()-1 {
        new_numbers.push(prev_numbers[i+1]-prev_numbers[i]);
    }
    new_numbers
}

fn only_zeros(vector: &Vec<i32>) -> bool{
    for i in vector {
        if i != &0 {return false;}
    }
    return true;
}

fn analyze_part2(input: &str) -> i32{
    let mut sum = 0;
    
    for line in input.lines() {
        let mut numbers: Vec<i32> = line.split(' ').map(|f| f.parse::<i32>().expect("should be correctly formatted")).collect();
        let mut first_numbers: Vec<i32> = Vec::new();

        loop {
            first_numbers.push(*numbers.first().unwrap());
            numbers = sub_sequence(numbers);

            //search for non 0, meaning we need to continue
            if only_zeros(&numbers) {
                break;
            }
        }

        println!("first numbers: {first_numbers:?}");
        let extrapolated = first_numbers.seq_div();
        println!("extrapolated value: {extrapolated}");
        sum += extrapolated;
    }

    sum  
}
