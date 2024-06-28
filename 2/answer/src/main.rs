use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_input() -> std::string::String {
    let path = Path::new("../2.txt");
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
    s.trim().to_string()
}

fn compute(mut ins: Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut done = false;
    let mut i = 0;

    ins[1] = noun;
    ins[2] = verb;
    while !done {
        if ins[i] == 99 {
            done = true;
        } else if ins[i] == 1 {
            let pos1 = ins[i + 1] as usize;
            let pos2 = ins[i + 2] as usize;
            let pos3 = ins[i + 3] as usize;
            ins[pos3] = ins[pos1] + ins[pos2];
        } else if ins[i] == 2 {
            let pos1 = ins[i + 1] as usize;
            let pos2 = ins[i + 2] as usize;
            let pos3 = ins[i + 3] as usize;
            ins[pos3] = ins[pos1] * ins[pos2];
        }
        i += 4;
    }
    ins[0]
}

fn main() {
    let s = read_input();
    let mut split: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    let res = compute(split.clone(), 12, 2);

    let mut res2 = 0;
    for n in 0..99 {
        for v in 0..99 {
            if compute(split.clone(), n, v) == 19690720 {
                res2 = 100 * n + v;
                break;
            }
        }
    }
    println!("result: answer 1 {} answer 2 {}", res, res2);
}
