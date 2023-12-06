use std::fs;

struct Race{
    time: u32,
    distance: u32
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("There should be an input.txt");

    let part1 = analyze_part1(&input);
    let part2 = analyze_part2(&input);

    println!("part1: {}",part1);   
    println!("part2: {}",part2);   
}

fn analyze_part1(string: &str) -> u32{
    
    let line: Vec<&str> = string.lines().collect();
    let time: Vec<u32> = line[0][5..].split_whitespace().map(|f| f.parse::<u32>().expect("should be correctly formatted")).collect();
    let distance: Vec<u32> = line[1][9..].split_whitespace().map(|f| f.parse::<u32>().expect("should be correctly formatted")).collect();
    //println!("{time:?}");
    //println!("{distance:?}");

    let mut races :Vec<Race>= Vec::new();
    for i in 0..time.len(){
        races.push(Race { time: time[i], distance: distance[i] });
    }

    let mut opportunities_to_win = 1;

    for race in races {
        let time = race.time as f32;
        let min_distance = race.distance as f32;

        let mut max = 0.5* (time + (time.powi(2) - 4.0*min_distance).sqrt()); //quadralic formula for x1
        if max == max.floor() {
            max = max-1.0;
        }else{
            max = max.floor();
        }
        let max = max as u32;

        let mut min = 0.5* (time - (time.powi(2) - 4.0*min_distance).sqrt()); //quadralic formula for x2
        if min == min.ceil() {
            min = min+1.0;
        }else{
            min = min.ceil();
        }
        let min = min as u32;

        let number_of_ways = max-min+1;
        opportunities_to_win *= number_of_ways;
    }

    opportunities_to_win
}

fn analyze_part2(string: &str) -> u32{
    
    let line: Vec<&str> = string.lines().collect();
    let time: u32 = line[0][5..].replace(' ', "").parse().expect("should be correctly formatted");
    let distance: u64 = line[1][9..].replace(' ', "").parse().expect("should be correctly formatted");

    let time = time as f64;
    let min_distance = distance as f64;

    let mut max = 0.5* (time + (time.powi(2) - 4.0*min_distance).sqrt()); //quadralic formula for x1
    if max == max.floor() {
        max = max-1.0;
    }else{
        max = max.floor();
    }
    let max = max as u32;

    let mut min = 0.5* (time - (time.powi(2) - 4.0*min_distance).sqrt()); //quadralic formula for x2
    if min == min.ceil() {
        min = min+1.0;
    }else{
        min = min.ceil();
    }
    let min = min as u32;

    max-min+1
}
