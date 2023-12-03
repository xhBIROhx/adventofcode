use std::fs::File;
use std::io::{BufReader, BufRead};
static NUMBERS_COMBINED: [&str;18] = ["1","2","3","4","5","6","7","8","9","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

//I tougt that I'd scan from the back, I believe it'd be more efficient, but whatever.
//static NUMBERS_COMBINED_REV: [&str;18] = ["1","2","3","4","5","6","7","8","9","eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];

fn main(){
    let file1 = File::open("input.txt").expect("file not found");
    let file2 = File::open("input.txt").expect("file not found");

    let value1 = analyze_part_1(file1);
    println!(" --- PART 2 ---");
    let value2 = analyze_part_2(file2);

    match value1 {
        Some(result) => println!("the part 1 sum is: {}", result),
        _ => {}
    }
    match value2 {
        Some(result) => println!("the part 2 sum is: {}", result),
        _ => {}
    }
}

fn analyze_part_1(file:File) -> Option<u32> {
    let reader = BufReader::new(file);
    let mut value_sum = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(t) => t,
            _ => continue
        };
        
        let mut a: Option<u32> = None;
        let mut b: Option<u32> = None;
        //get first digit
        for char in line.chars(){
            match char.to_digit(10){
                Some(v) => {
                    a = Some(v);
                    break;
                },
                _ => continue
            }
        }

        //get last digit
        for char in line.chars().rev(){
            match char.to_digit(10){
                Some(v) => {
                    b = Some(v);
                    break;
                },
                _ => continue
            }
        }
        if let (Some(a),Some(b)) = (a,b){
            let val = a*10+b;
            println!("{}: {}", line,val);
            value_sum += val;
        }else{
            println!("{}: {}", line,"ERROR");
            return None;
        }
    }
    Some(value_sum)
}

fn analyze_part_2(file:File) -> Option<u32> {
    let reader = BufReader::new(file);
    let mut value_sum = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(t) => t,
            _ => continue
        };

        let mut index = vec![];
        //get locations
        //DOES NOT GET DUPLICATES
        for (i,string_numbers) in NUMBERS_COMBINED.iter().enumerate(){
            for (location, _) in line.match_indices(string_numbers) {

                println!("found \"{}\" at {} in: \"{}\", gave it: {}",string_numbers,location,line,if i>8 {i as u32-9+1} else {i as u32 +1});
                
                index.push((location,if i>8 {i as u32-9+1} else {i as u32 +1}));

            }
        }

        index.sort();
        println!("final: {:?}",index);
        let a = Some(index[0].1);
        let b = Some(index.pop().unwrap().1);
        println!("left: {:?}, right: {:?}", a, b);

        if let (Some(a),Some(b)) = (a,b){
            let val = a*10+b;
            println!("{}: {}", line,val);
            value_sum += val;
        }else{
            println!("{}: {}", line,"ERROR");
            return None;
        }
    }
    Some(value_sum)
}