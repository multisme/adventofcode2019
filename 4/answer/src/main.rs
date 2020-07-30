use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Range {
    start: i32,
    end: i32
}


fn read_input() -> std::string::String {
    //  let path = Path::new("../4.test0.txt"); //test file
    let path = Path::new("../4.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open the file {}: {}", display, why.description()),
        Ok(file) => file,
    };  
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}:  {}", display, why.description()),
        Ok(_) => (), 
    };
    s   
}

fn stov32(string: &str)->Vec<u32>{
    let value: Vec<u32> = string.chars().map(|x| x.to_digit(10).unwrap()).collect();
    println!("{:?}", value);
    value
}

fn make_starting_point(start: &Vec<u32>) -> Vec<u32>{

    let len = start.len();

    let mut begin = 0;
    let mut start = start.clone();

    for i in begin..len{
        begin = i;
    }
    for i in 1..len {
        if start[i] < start[i - 1]{
            let threshold = start[i - 1];
            for n in i..len{
                start[n] = threshold;
            }
            break;
        }
    }
    println!("make starting point{:?}", start);
    start
}

fn find_password(start: &mut Vec<u32>, end: &Vec<u32>) -> u32{

    let len = start.len() as i32;

    let mut count = 0;
    let mut n: i32 = len - 1;

    println!("0 ====> {:?} {:?}", count, start);
    while start[0] < end[0]{
        count +=1;
        start[n as usize] += 1;
        println!("0 ====> {:?} {:?}", count, start);
        if start[n as usize] >= 9{
            n -= 1;
            let mut step = n;
             while (step < len){
                    start[step as usize] = start[n as usize];
                    println!("step {:?}", step);
                    step += 1;
            }
        }
    }
    return count;
}

fn nothing(vect: Vec<Vec<u32>>){
}

fn main() {
    //Read input and split by lines
    let file_input = read_input();
    //Get the right data from the input
    let input: &str = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => ""
    };
    //Str range split
    let str_range: Vec<Vec<_>> = input.split('-').map(|x| stov32(x)).collect();
    println!("{:?}",str_range);
    let mut start = make_starting_point(&str_range[0]);
    //nothing(str_range);
    let count = find_password(&mut start, &str_range[1]);
    println!("{:?}",count);
    //let range: Range = Range{start: str_range[0], end: str_range[1]};
}
