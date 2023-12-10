use std::{fs, vec};

mod part2;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Index{
    line: usize,
    char: usize
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Pipe{
    index: Index,
    //from: u8, //can be easly calculated form prev_to
    to: u8,
    distance: u32
}

/* 
0: up
1: right
2: down
3: left
*/
trait CharParse {
    fn get_pipe(&self) -> Vec<u8>;
}

impl CharParse for char {
    fn get_pipe(&self) -> Vec<u8>{
        match self {
            '|' => vec![0,2],
            '-' => vec![1,3],
            'L' => vec![0,1],
            'J' => vec![0,3],
            '7' => vec![3,2],
            'F' => vec![1,2],
            '.' => vec![],
            _ => panic!("couldn't parse character ({self}) into a pipe")
        }
    }
}

trait VecParse {
    fn get_char(&self, index: &Index) -> char;
}

impl VecParse for Vec<&str> {
    fn get_char(&self, index: &Index) -> char {
        self[index.line].chars().collect::<Vec<char>>()[index.char]
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("thre should be an input.txt");

    let part1 = analyze_part1(&input);
    let part2 = part2::analyze_part2(&input);
    println!("part1: {part1}");
    println!("part2: {part2}");

    //I don't know how to do part2
}

fn analyze_part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let start_index = get_index(&lines).expect("there should be an \'S\'");
    println!("start index: {start_index:?}");

    let mut pipes: Vec<Pipe> = find_two_ways(start_index, &lines);
    println!("two staring pipes: {pipes:?}");

    loop {
        pipes = next_pipes(pipes, &lines);
        //println!("{pipes:?}");
        if pipes[0].index.line == pipes[1].index.line && pipes[0].index.char == pipes[1].index.char {
            break;
        }
    }

    pipes[0].distance
}

fn get_index(lines: &Vec<&str>) -> Option<Index>{ //Find S in the supplied lines (whole string)
    for (i,line) in lines.iter().enumerate() {
        for (j,char) in line.chars().enumerate(){
            if char == 'S' {
                return Some(Index {line: i, char: j});
            }
        }
    }
    return None;
}    

fn find_two_ways(start_index: Index, lines: &Vec<&str>) -> Vec<Pipe> {
    let mut found_pipes = Vec::new();
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
            directions.retain(|f| f != &from);
            found_pipes.push(Pipe { index, to: directions[0], distance: 1 })
        }

    }
    return found_pipes;
}

fn next_pipes(current_pipes: Vec<Pipe>, lines: &Vec<&str>) -> Vec<Pipe> {
    let mut retrun_pipes = Vec::new();
    for pipe in current_pipes {
        //modiy index based on .to
        let mut index = pipe.index;
        if pipe.to % 2 == 0 {
            //modify the lines
            if pipe.to == 0 {
                //one up
                index.line -= 1;
            }else{
                //one down
                index.line += 1;
            }
        }else{
            //modify the char
            if pipe.to == 1 {
                //one right
                index.char += 1;
            }else{
                //one left
                index.char -= 1;
            }
        }

        //parse char
        //let to;
        let mut directions = lines.get_char(&index).get_pipe();
        //print!("{directions:?} - {} = ", ((pipe.to+2)%4));
        directions.retain(|f| f != &((pipe.to+2)%4));
        //println!("{directions:?}");
        //to +2 % 4;


        let ret_pipe = Pipe { index, to: directions[0], distance: pipe.distance+1};
        retrun_pipes.push(ret_pipe);
        //build new pipe
    }
    retrun_pipes
}