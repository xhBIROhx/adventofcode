use std::fs;
use std::collections::HashMap;

fn main() {
    
    let input = fs::read_to_string("input.txt").expect("there should be an input.txt next to the executeable");
    
    let part1 = analyze_part1(&input);
    let part2 = analyze_part2(&input);

    if let Some(result) = part1 {
        println!("part1 solution: {}", result)
    }else{
        println!("Something went wrong");
    }
    if let Some(result) = part2 {
        println!("part2 solution: {}", result)
    }else{
        println!("Something went wrong");
    }
}

fn analyze_part1(s: &str) -> Option<u32>{
    let max_cubes: HashMap<&str, u32> = HashMap::from([("red",12),("green",13),("blue",14)]);
    let mut sum = 0;

    for line in s.lines(){
        let parts: Vec<&str> = line.split(": ").collect();

        //get gameID
        let game: Vec<&str> = parts[0].split(" ").collect();
        let game: u32 = game[1].parse().expect("should always be a number");

        //get pulls
        let pulls: Vec<&str> = parts[1].split("; ").collect();
        if pull_analyzer(pulls, &max_cubes){
            sum += game;
        }
    }
    Some(sum)
}

fn pull_analyzer(pulls: Vec<&str>,max_cubes: &HashMap<&str, u32>) -> bool { //true if it is possible, false if it is impossible 
    for pull in pulls{
        let cubes: Vec<&str> = pull.split(", ").collect();
        for cube in cubes{
            let cube: Vec<&str> = cube.split(" ").collect();
            let amount: u32 = cube[0].parse().expect("should always be a number");
            let color = cube[1];
            let max = max_cubes.get(color).expect("there should be a stored value for all colors");
            if amount > *max {
                return false;
            }
        }
    }
    true
}

fn analyze_part2(s: &str) -> Option<u32>{
    let mut min_cubes;
    let mut sum = 0;

    for line in s.lines(){
        min_cubes = HashMap::new();
        let parts: Vec<&str> = line.split(": ").collect();
        
        //get pulls
        let pulls: Vec<&str> = parts[1].split("; ").collect();
        for pull in pulls{
            let cubes: Vec<&str> = pull.split(", ").collect();
            for cube in cubes{
                let cube: Vec<&str> = cube.split(" ").collect();
                let amount: u32 = cube[0].parse().expect("should always be a number");
                let color = cube[1];
                let entry = min_cubes.entry(color).or_insert(amount);
                if *entry < amount {
                    *entry = amount
                }
            }
        }
        
        //get answer
        let power = {
            let mut power = 1;
            for values in min_cubes.values(){
                power *= values;
            }
            power
        };
        sum += power;
    }
    Some(sum)
}