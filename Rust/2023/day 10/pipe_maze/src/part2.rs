use crate::*;
use std::collections::HashSet;

trait Print {
    fn print(&self);
}

impl Print for Vec<Vec<u8>> {
    fn print(&self) {
        for line in self {
            print!("\n");
            for digit in line{
                print!("{digit}");
            }
        }
        println!("\n")
    }
}

pub fn analyze_part2(input: &str) -> u32 {
    let mut lines: Vec<&str> = input.lines().collect();

    let start_index = get_index(&lines).expect("there should be an \'S\'");
    println!("start index: {start_index:?}");

    let start_values = find_two_ways_p2(&start_index, &lines);
    let mut pipes = start_values.0;
    println!("two staring pipes: {pipes:?}");
    let start_directions = {
        let mut start_directions = Vec::new();
        for value in start_values.1 {
            start_directions.push((value+2)%4)
        }
        [start_directions[0],start_directions[1]]
    };
    println!("start directions: {start_directions:?}");

    //replace S with the corresponding pipe
    let start_string = {
        let start_line = lines[start_index.line];
        let mut start_chars = start_line.chars().collect::<Vec<char>>();
        let start_character = match start_directions { //2301
            [0,1] => '|',//20
            [1,3] => '-',//31
            [1,0] => 'L',//01
            [0,3] => 'J',//30
            [3,2] => '7',//23
            [1,2] => 'F',//21
            _ => panic!("couldn't reverse parse {start_directions:?}")
        };
        start_chars[start_index.char] = start_character;
        start_chars.iter().collect::<String>()

    };
    //print!("to be replaced line:\n{}",lines[start_index.line]);
    lines[start_index.line] = &start_string[..];
    //println!("\nwith:\n{}",start_string);
    //end of replaceing 'S'

    let mut coords = HashSet::new();
    //add start:
    coords.insert(start_index);

    coords.extend(pipes.iter().map(|f| f.index.clone()));

    loop {
        pipes = next_pipes(pipes, &lines);
        //println!("{pipes:?}");
        if pipes[0].index.line == pipes[1].index.line && pipes[0].index.char == pipes[1].index.char {
            coords.insert(pipes[0].index.clone());
            break;
        }
        for i in &pipes {
            coords.insert(i.index.clone());
        }
    }

    //do a lines/chars min/max to crop the map down if it takes to long to compute

    let mut map = get_larger_map(&lines, coords);
    //println!("{map:?}");

    let count = filter_and_count_0s(&mut map);


    count/9 // rustc stop cry
}

fn find_two_ways_p2(start_index: &Index, lines: &Vec<&str>) -> (Vec<Pipe>, [u8;2]) {
    let mut found_pipes = Vec::new();
    let mut both_directions = Vec::new();
    for i in 0..4 {
        let mut index = start_index.clone();
        let from;
        if i % 2 == 0 {
            //modify the lines
            if i == 0 {
                //one up
                index.line -= 1;
                from = 2;
            }else{
                //one down
                index.line += 1;
                from = 0;
            }
        }else{
            //modify the char
            if i == 1 {
                //one right
                index.char += 1;
                from = 3;
            }else{
                //one left
                index.char -= 1;
                from = 1;
            }
        }

        let mut directions = lines.get_char(&index).get_pipe();
        if directions.contains(&from) {
            both_directions.push(from);
            directions.retain(|f| f != &from);
            found_pipes.push(Pipe { index, to: directions[0], distance: 1 })
        }

    }
    return (found_pipes,[both_directions[0],both_directions[1]]);
}

fn get_larger_map(map: &Vec<&str>, coords: HashSet<Index>) -> Vec<Vec<u8>>{
    /* 
    0: undiscovered, to be replaced, to be counted
    1: walls
    2: wall boundries, to be replaced, not to be counted
    3: = " ", discovered, replaced, uncounted.

    at the end count 0's and /9
    */

    let mut return_map = Vec::new();
    for (i,line) in map.iter().enumerate() {
        let mut line1 = Vec::new();
        let mut line2 = Vec::new();
        let mut line3 = Vec::new();
        for (j,char) in line.chars().enumerate() {
            match coords.get(&Index { line: i, char: j }) {
                Some(_) => match char {
                    '|' => {
                        line1.extend(vec![2,1,2]);
                        line2.extend(vec![2,1,2]);
                        line3.extend(vec![2,1,2]);
                    },
                    '-' => {
                        line1.extend(vec![2,2,2]);
                        line2.extend(vec![1,1,1]);
                        line3.extend(vec![2,2,2]);
                    },
                    'L' => {
                        line1.extend(vec![2,1,2]);
                        line2.extend(vec![2,1,1]);
                        line3.extend(vec![2,2,2]);
                    },
                    'J' => {
                        line1.extend(vec![2,1,2]);
                        line2.extend(vec![1,1,2]);
                        line3.extend(vec![2,2,2]);
                    },
                    '7' => {
                        line1.extend(vec![2,2,2]);
                        line2.extend(vec![1,1,2]);
                        line3.extend(vec![2,1,2]);
                    },
                    'F' => {
                        line1.extend(vec![2,2,2]);
                        line2.extend(vec![2,1,1]);
                        line3.extend(vec![2,1,2]);
                    },
                    _ => panic!("couldn't extend {char}")
                },
                None => {
                    line1.extend(vec![0,0,0]);
                    line2.extend(vec![0,0,0]);
                    line3.extend(vec![0,0,0]);
                }
            }
        }
        return_map.push(line1);
        return_map.push(line2);
        return_map.push(line3);
    }
    return_map

    //vec![vec![1]] // rustc stop cry
}

fn filter_and_count_0s(map: &mut Vec<Vec<u8>>) -> u32{
    let index = Index { line: 0, char: 0};
    let mut count = 0;
    let mut indexes = Some(vec![index]);

    //replace
    loop {
        if let Some(index) = indexes {
            indexes = recursive_replace(map,index);
        }else{
            break;
        }
        //uncomment this for fancy print, lags like hell, not even that fancy
        //map.print();
    }
    for line in map {
        for digit in line {
            if digit == &mut 0 {
                count +=1;
            }
        }
    }
    //println!("{map:?}");
    count // rustc stop cry
} 

fn recursive_replace(map: &mut Vec<Vec<u8>>, index: Vec<Index>) -> Option<Vec<Index>>{
    let mut to_be_scanned = Vec::new();

    for index in index {
        map[index.line][index.char] = match map[index.line][index.char] {
            0 => 3,
            2 => 3,
            _ => continue
        };
    
        to_be_scanned.extend(all_ajacent_coords(index,map.len()-1,map[0].len()-1))
    }
    
    if to_be_scanned.len() > 0 {
        Some(to_be_scanned)
    }else{
        None
    }
}

fn all_ajacent_coords(index: Index, line_max: usize, char_max: usize) -> Vec<Index>{
    let mut ret_index = Vec::new();
    if index.line != 0 {
        //up
        ret_index.push(Index { line: index.line-1, char: index.char});
    }
    if index.line != line_max {
        //down
        ret_index.push(Index { line: index.line+1, char: index.char});
    }
    if index.char != 0 {
        //left
        ret_index.push(Index { line: index.line, char: index.char-1});
    }
    if index.char != char_max {
        //right
        ret_index.push(Index { line: index.line, char: index.char+1});
    }

    ret_index
}
