use std::{fs, collections::HashSet};

struct Index {
    row: usize,
    column: usize
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("there should be an input.txt");
    
    let part1 = analyze_part1(&input);
    println!(" --- part 2 ---");
    let part2 = analyze_part2(&input);
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn analyze_part1(input: &str) -> u32 {
    let mut sum = 0;

    //get things
    let (row_hash, column_hash, galaxies) = galaxies(input);

    for i in 0..galaxies.len(){
        for j in i+1..galaxies.len() {
            let galaxy1 = &galaxies[i];
            let galaxy2 = &galaxies[j];

            let distance = {
                let mut distance = galaxy1.row.abs_diff(galaxy2.row) + galaxy1.column.abs_diff(galaxy2.column);
                //expansion logic
                let row_range = if galaxy1.row < galaxy2.row {galaxy1.row+1..galaxy2.row} else {galaxy2.row+1..galaxy1.row};
                let column_range = if galaxy1.column < galaxy2.column {galaxy1.column+1..galaxy2.column} else {galaxy2.column+1..galaxy1.column};
                for row in row_range { //every whitch way
                    if row_hash.get(&row).is_none() { // this row index couldn't be found in the rows of all the galaxies
                        distance +=1
                    }
                }
                for column in column_range {
                    if column_hash.get(&column).is_none() { // this column index couldn't be found in the columns of all the galaxies
                        distance +=1
                    }
                }

                distance
            };
            println!("distance: {distance}");
            sum += distance;
        }
    }

    sum as u32
    //0 //stfu rustc
}

fn analyze_part2(input: &str) -> u64 {
    let mut sum = 0;

    //get things
    let (row_hash, column_hash, galaxies) = galaxies(input);

    for i in 0..galaxies.len(){
        for j in i+1..galaxies.len() {
            let galaxy1 = &galaxies[i];
            let galaxy2 = &galaxies[j];

            let distance = {
                let mut distance = galaxy1.row.abs_diff(galaxy2.row) + galaxy1.column.abs_diff(galaxy2.column);
                //expansion logic
                let row_range = if galaxy1.row < galaxy2.row {galaxy1.row+1..galaxy2.row} else {galaxy2.row+1..galaxy1.row};
                let column_range = if galaxy1.column < galaxy2.column {galaxy1.column+1..galaxy2.column} else {galaxy2.column+1..galaxy1.column};
                for row in row_range { //every whitch way
                    if row_hash.get(&row).is_none() { // this row index couldn't be found in the rows of all the galaxies
                        distance +=999999
                    }
                }
                for column in column_range {
                    if column_hash.get(&column).is_none() { // this column index couldn't be found in the columns of all the galaxies
                        distance +=999999
                    }
                }

                distance
            };
            println!("distance: {distance}");
            sum += distance;
        }
    }

    sum as u64
    //0 //stfu rustc
}


/* 
index all galaxies
index expanded rows & columns

iterate over all pairs, double for I guess
    for i in 0..len
        for j in i+1..len
            logic

logic is simple, difference between the row index & the column index
plus some number to make it actually return what it should :p

add to it 1 any time if any expanded index is in boundary of the start and des indexes
*/

fn galaxies(input: &str) -> (HashSet<usize>,HashSet<usize>,Vec<Index>) {
    let (mut row_hash, mut column_hash, mut galaxies) = (HashSet::new(), HashSet::new(), Vec::new());
    
    //iterate thru the entire map
    for (row,line) in input.lines().enumerate() {
        for (column,char) in line.chars().enumerate() {
            if char == '#' {
                row_hash.insert(row);
                column_hash.insert(column);
                galaxies.push(Index { row, column });
            }
        }
    }

    (row_hash, column_hash, galaxies)
}