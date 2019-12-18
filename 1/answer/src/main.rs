use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_input() -> std::string::String {
    let path = Path::new("../1.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open the file {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}:  {}", display, why.description()),
        Ok(_) => ()
    };
    s
}

fn additionnal_fuel(weight: i32) -> i32  {

    let mut additional_ful = weight / 3 - 2;
    let mut res2: i32 = 0;
    while additional_ful > 0 {
        res2 += additional_ful;
        additional_ful = additional_ful / 3 - 2;
    }
    res2
}

fn main() {
    let s = read_input();
    let split: Vec<i32> = s.split_whitespace().
        map(|x| x.parse::<i32>().unwrap())
        .collect();
    let res = split.iter().fold(0, |a, b| a + b / 3  - 2);

    // Second answer

    let mut res2: i32 = split.iter().map(|x| additionnal_fuel(*x)).sum();
    println!("result: answer 1 {} answer 2 {}", res, res2);
}
