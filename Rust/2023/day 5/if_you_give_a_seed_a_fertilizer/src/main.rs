use std::fs;
use std::collections::HashMap;

struct Range {
    destination_start: u32,
    source_start: u32,
    lenght: u32
}

#[derive(Debug)]
struct SeedRange {
    start: u32,
    end: u32
}

impl SeedRange {
    fn copy(&self) -> Self{
        SeedRange {start: self.start, end: self.end}
    }
}

struct Transformation{
    //from: String, //let's make this the hash's key, for easier access.
    to: String,
    entries: Vec<Range>
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("there should be an input.txt");

    let part1 = analyze_part1(&input);
    let part2 = analyze_part2(&input);
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn analyze_part1(input: &str) -> u32{
    let mut map = HashMap::new();
    let parts: Vec<&str>= input.split("\n\n").collect();

    //get seeds
    let seeds: Vec<u32> = parts[0][7..].split(' ').collect::<Vec<&str>>().iter().map(|f| f.parse::<u32>().expect("seeds must be numbers only")).collect();
    println!("{seeds:?}");

    //get transformations and ranges:
    for trans_block in &parts[1..] {
        let trans_lines: Vec<&str> = trans_block.lines().collect();
        let ingredients = trans_lines[0].split_whitespace().collect::<Vec<&str>>()[0];
        let ingredients: Vec<&str>= ingredients.split('-').collect();
        //println!("ingr: {ingredients:?}");
        //this has the [0] as from and [2] as to

        for range_line in &trans_lines[1..] {
            let data: Vec<u32> = range_line.split(' ').collect::<Vec<&str>>().iter().map(|f| f.parse::<u32>().expect("should be formatted correctly")).collect();
            let entry = map.entry(ingredients[0]).or_insert( Transformation{to: ingredients[2].to_string(), entries: Vec::new()});
            entry.entries.push(Range { destination_start: data[0], source_start: data[1], lenght: data[2] });
        }
    }

    let locations = transmutate("seed".to_string(), seeds, &map);

    //min
    let mut min = locations[0];
    for location in locations[1..].into_iter() {
        if location < &min {
            min = *location;
        }
    }

    min //for debug
}

fn transmutate(from: String, elements: Vec<u32>, almanac: &HashMap<&str, Transformation>) -> Vec<u32> { // -> locations
    let mut transformed = Vec::new();

    let logic = match almanac.get(&from[..]) {
        Some(t) => t,
        _ => return elements //we won't be able to find "location"
    };
    println!("transforming: {}",elements.len());
    println!("from: {from} to {}",logic.to);

    'element: for element in &elements {
        for range in &logic.entries {
            if element >= &range.source_start && (*element as u64) < range.source_start as u64+range.lenght as u64 {
                let diff = element-range.source_start;
                transformed.push(diff+range.destination_start);
                continue 'element;
            }
        }
        //panic!("{element}") //we lost a seed //IF we didn't I just forgot to read the fucking puzzle fully
        transformed.push(*element);
    }
    //println!("{}={}",elements.len(),transformed.len());
    //if from == "seed" {panic!()}
    transmutate(logic.to.clone(), transformed, almanac)
}

// --- PART 2 ---

fn analyze_part2(input: &str) -> u32{
    let mut map = HashMap::new();
    let parts: Vec<&str>= input.split("\n\n").collect();

    //get seeds RANGES
    let seeds: Vec<u32> = parts[0][7..].split(' ').collect::<Vec<&str>>().iter().map(|f| f.parse::<u32>().expect("seeds must be numbers only")).collect();
    let mut prev_seed: Option<u32> = None;
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    for seed in seeds {
        if let Some(prev_seed_t ) = prev_seed {
            seed_ranges.push(SeedRange { start: prev_seed_t, end: prev_seed_t-1+seed }); //inclusive
            prev_seed = None;
        }else{
            prev_seed = Some(seed);
        }
    }

    println!("{seed_ranges:?}");

    //get transformations and ranges:
    for trans_block in &parts[1..] {
        let trans_lines: Vec<&str> = trans_block.lines().collect();
        let ingredients = trans_lines[0].split_whitespace().collect::<Vec<&str>>()[0];
        let ingredients: Vec<&str>= ingredients.split('-').collect();

        for range_line in &trans_lines[1..] {
            let data: Vec<u32> = range_line.split(' ').collect::<Vec<&str>>().iter().map(|f| f.parse::<u32>().expect("should be formatted correctly")).collect();
            let entry = map.entry(ingredients[0]).or_insert( Transformation{to: ingredients[2].to_string(), entries: Vec::new()});
            entry.entries.push(Range { destination_start: data[0], source_start: data[1], lenght: data[2] });
        }
    }

    let locations = transmutate_part2("seed".to_string(), seed_ranges, &map);

    //min
    let mut min = locations[0].start;
    for location in locations[1..].into_iter().map(|f| f.start) {
        if location < min {
            min = location;
        }
    }

    min //for debug
}

fn transmutate_part2(from: String, elements: Vec<SeedRange>, almanac: &HashMap<&str, Transformation>) -> Vec<SeedRange> { // -> locations
    let mut transformed = Vec::new();

    let logic = match almanac.get(&from[..]) {
        Some(t) => t,
        _ => return elements //we won't be able to find "location"
    };
    println!("transforming: {}",elements.len());
    println!("from: {from} to {}",logic.to);

    for element in &elements {
        let results = range_check_logic_part2(element,&logic.entries);
        transformed.extend(results);
        //push it into transformed
    }
    //println!("{}={}",elements.len(),transformed.len());
    //if from == "seed" {panic!()}
    transmutate_part2(logic.to.clone(), transformed, almanac)
}

fn range_check_logic_part2(element: &SeedRange,logic: &Vec<Range>) -> Vec<SeedRange>{
    for range in logic {
        let range_end = {range.source_start as u64 +range.lenght as u64-1} as u32;

        if element.start < range.source_start { //start before range start
            if element.end < range.source_start{ //end as well
                continue;
            }
            if element.end >= range.source_start && element.end <= range_end { //end between
                let element1 = SeedRange {start: element.start, end: range.source_start-1};
                let element2 = SeedRange {start: range.source_start, end: element.end};
                //we need to split it.
                
                let mut return_elements: Vec<SeedRange> = Vec::new();
                return_elements.extend(range_check_logic_part2(&element1, logic));
                return_elements.extend(range_check_logic_part2(&element2, logic));
                return return_elements;
            }
            if element.end > range_end { //end after range end
                let element1 = SeedRange {start: element.start, end: range.source_start-1};
                let element2 = SeedRange {start: range.source_start, end: range_end};
                let element3 = SeedRange {start: range_end+1, end: element.end};
                //we need to split it into 3 pices.

                let mut return_elements: Vec<SeedRange> = Vec::new();
                return_elements.extend(range_check_logic_part2(&element1, logic));
                return_elements.extend(range_check_logic_part2(&element2, logic));
                return_elements.extend(range_check_logic_part2(&element3, logic));
                return return_elements;
            }
        }

        if element.start >= range.source_start && element.start <= range_end{ //start between
            if element.end >= range.source_start && element.end <= range_end { //end as well
                let start_diff = element.start-range.source_start;
                let end_diff = element.end-range.source_start;
                return vec![SeedRange{start: start_diff+range.destination_start, end: end_diff+range.destination_start}];
                //we are golden
            }
            if element.end > range_end { //end outside of range
                let element1 = SeedRange {start: element.start, end: range_end};
                let element2 = SeedRange {start: range_end+1, end: element.end};
                //we need to split it.
                
                let mut return_elements: Vec<SeedRange> = Vec::new();
                return_elements.extend(range_check_logic_part2(&element1, logic));
                return_elements.extend(range_check_logic_part2(&element2, logic));
                return return_elements;
            }
        }

        if element.start > range_end { //start after range end
            continue;
        }
    }
    //panic!("{element}") //we lost a seed //IF we didn't I just forgot to read the fucking puzzle fully
    return vec![{element.copy()}];
}
/* 
check if it's in range
like

into from len

if range start < to find
if range start + range lenght -1 < to find
akkor csak kell a destination and the source range start deference and calculate it

*/

/* 
if the start range is <= seed start range
then we found the one. however! this does not mean that the entire range is exhausted, we need to split it up.


when we found the range that fits in the transform range, shift both the start and the end ranges respectively.
*/