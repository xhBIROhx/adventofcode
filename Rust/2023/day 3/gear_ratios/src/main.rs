use std::fs;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Debug)]
struct Number {
    line: usize,
    index: usize,
    size: u32,
    value: u32
}

#[derive(Eq, Hash, PartialEq)]
struct StarCoords {
    line: usize,
    index: usize
}

const SYMBOLS: [char;10] = ['*', '@', '$', '+', '#', '/', '&', '=', '-', '%'];

fn main() {
    let input = fs::read_to_string("input.txt").expect("There should be an input.txt");
    
    let part1 = analyze_part1(&input);
    let part2 = analyze_part2(&input);

    if let Some(result) = part1 {
        println!("Part1 result: {result}");
    }else{
        println!("something went wrong.");
    }
    if let Some(result) = part2 {
        println!("Part2 result: {result}");
    }else{
        println!("something went wrong.");
    }
}

fn analyze_part1(input: &str) -> Option<u32>{
    let data = scan_part1(input);
    let sum= check_surroundings_part1(input, data);
    Some(sum)
}

fn scan_part1(input: &str) -> Vec<Number> {
    let mut data = Vec::new();

    for (l,line) in input.lines().into_iter().enumerate() {
        
        let mut prev_numeric = false;
        let mut prev_len: u32 = 0;

        for (c,char) in line.chars().into_iter().enumerate() {
            if char.is_numeric(){
                if prev_numeric {
                }else{
                    prev_numeric = true;
                }
                prev_len += 1;
                if c == line.len()-1 {
                    //premature test maybe?

                    //save
                    //println!("to parse: {}", &line[c-prev_len as usize..c]);
                    let value = line[c-prev_len as usize+1..=c].parse().expect("should always be a number, as we did the checks above");
                    data.push(Number{line: l, index: c-prev_len as usize+1, size: prev_len, value});
                }
            }else{
                if prev_numeric {
                    //premature test maybe?

                    //save
                    let value = line[c-prev_len as usize..c].parse().expect("should always be a number, as we did the checks above");
                    data.push(Number{line: l, index: c-prev_len as usize, size: prev_len, value});
                }
                prev_numeric = false;
                prev_len = 0;
            }
        }
    }
    data
}

fn check_surroundings_part1(input: &str, data: Vec<Number>) -> u32{
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;

    for found in data{
        println!("testing {:?}",found);
        //one line above test
        if found.line != 0 && line_test_part1(lines[found.line-1], found.index, found.size) {
            sum += found.value;
            continue;
        }
        
        //the line test
        if line_test_part1(lines[found.line], found.index, found.size) {
            sum += found.value;
            continue;
        }

        //one line below test
        if found.line != lines.len()-1 && line_test_part1(lines[found.line+1], found.index, found.size) {
            sum += found.value;
            continue;
        }
    }
    sum
}

fn line_test_part1(line: &str, index: usize, size: u32) -> bool{ //true if summable (found symbol)
    let start_index = if index == 0 {
        index
    }else {
        index-1
    };
    let end_index = min(index+size as usize,line.len()-1);
    /* let end_index = if index+size as usize > line.len()-1 {
        line.len()-1
    }else{
        index+size as usize
    }; */
    let string = &line[start_index..=end_index];
    print!("testing part: {}",string);
    for symbol in SYMBOLS{
        if string.contains(symbol){
            println!(" ✅");
            return true;
        }
    }
    println!(" ❌");
    false
}

//PART 2

fn analyze_part2(input: &str) -> Option<u32>{
    let data = scan_part1(input);
    //println!("{:?}", data);
    let sum= check_surroundings_part2(input, data);
    Some(sum)
}

fn check_surroundings_part2(input: &str, data: Vec<Number>) -> u32{
    let lines: Vec<&str> = input.lines().collect();
    let mut sum: u32 = 0;
    let mut map: HashMap<StarCoords,Vec<u32>> = HashMap::new();

    for found in data{
        //one line above test
        if found.line != 0 {
            let line_index = found.line-1;
            line_test_part2(lines[line_index], found.index, found.size, &mut map, line_index, found.value)
        }
        
        //the line test
        line_test_part2(lines[found.line], found.index, found.size, &mut map, found.line, found.value);

        //one line below test
        if found.line != lines.len()-1{
            let line_index = found.line+1;
            line_test_part2(lines[line_index], found.index, found.size, &mut map, line_index, found.value)
        }
    }

    //print out:
    for value in map.values() {
        if value.len() == 2 {
            sum += value[0]*value[1];
        }
    }
    sum
}

fn line_test_part2(line: &str, index: usize, size: u32, map: &mut HashMap<StarCoords,Vec<u32>>, coord_line: usize, value: u32){ //true if summable (found symbol)
    let start_index = if index == 0 {
        index
    }else {
        index-1
    };
    let end_index = min(index+size as usize,line.len()-1);
    let string = &line[start_index..=end_index];

    //print!("testing part: {}",string);
    for (star_index, _) in string.match_indices('*') {
        let index = star_index+start_index;
        let entry = map.entry(StarCoords { line: coord_line, index}).or_insert(vec![]);
        entry.push(value);
    }

    /* if string.contains('*'){
        println!(" ✅");
        return true;
    }
    println!(" ❌");
    false */
}

/* 
per character is a digit
if yes scan the next characters until it's not

I know know the location and size of the number
check the characters before and after it maybe we can instantly eliminate it

if not eliminated:
    save it to a something with line number, location, size, amount

once we have all found numbers that are not directly adjacent interate over then one line above and below in hopes of finding a symbol
after this we can know if it is should be added to the sum or not.
*/

/* 
if it conatains a * in surroundings
save the value and the star's coords

hasmap!
hasmap with the coords to values

at the end when the value vector has exactly two values mutliply and sum
otherwise ignore
*/