use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_input() -> std::string::String {
    //  let path = Path::new("../4.test0.txt"); //test file
    let path = Path::new("../4.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open the file {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}:  {}", display, why.to_string()),
        Ok(_) => (),
    };
    s
}

fn stov32(string: &str) -> Vec<u32> {
    let value: Vec<u32> = string.chars().map(|x| x.to_digit(10).unwrap()).collect();
    //println!("Parse value {:?}", value);
    value
}

fn make_starting_point(start: &Vec<u32>) -> Vec<u32> {
    let len = start.len();

    let mut start = start.clone();
    let mut i = 1;

    while i < len {
        if start[i] < start[i - 1] {
            let threshold = start[i - 1];
            for n in i..len {
                start[n] = threshold;
            }
            println!("start of the search {:?}", start);
            return start;
        }
        i += 1;
    }
    //println!("start of the search {:?}", start);
    start
}

fn find_password(start: &mut Vec<u32>, end: &Vec<u32>) -> Vec<Vec<u32>> {
    let len = start.len() as i32;

    let mut n: i32 = len - 1;
    let mut vec = Vec::new();

    while start[0] < end[0] {
        //   println!("1 ====> {:?}", start);
        vec.push(start.clone());
        if start[n as usize] >= 9 {
            while start[n as usize] >= 9 && n > 0 {
                n -= 1;
            }
            if n < 0 {
                break;
            }
            start[n as usize] += 1;
            let level = start[n as usize];
            while n < len {
                start[n as usize] = level;
                n += 1;
            }
            n = len - 1;
            continue;
        }
        start[n as usize] += 1;
    }
    vec
}

fn test_only_2_adjacent_digit(to_check: &Vec<u32>) -> bool {
    let mut i: usize = 0;
    let len = to_check.len() - 1;
    let mut res = vec![0; 10];
    while i < len {
        if to_check[i] == to_check[i + 1] {
            let val = to_check[i] as usize;
            res[val] += 1;
        }
        i += 1;
    }
    res.contains(&1)
}

fn test_adjacent_digit(to_check: &Vec<u32>) -> bool {
    let mut i: usize = 0;
    let len = to_check.len() - 1;
    while i < len {
        if to_check[i] == to_check[i + 1] {
            return true;
        }
        i += 1;
    }
    false
}

fn main() {
    //Read input and split by lines
    let file_input = read_input();
    //Get the right data from the input
    let input: &str = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => "",
    };
    //Str range split
    let str_range: Vec<Vec<_>> = input.split('-').map(|x| stov32(x)).collect();
    let mut start = make_starting_point(&str_range[0]);
    //nothing(str_range);
    let to_checks = find_password(&mut start, &str_range[1]);
    let count1 = to_checks.iter().filter(|x| test_adjacent_digit(x)).count();
    let count2 = to_checks
        .iter()
        .filter(|x| test_only_2_adjacent_digit(x))
        .count();
    println!("result one: {:?} result two: {:?}", count1, count2);
}
