use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_input() -> std::string::String {
    let path = Path::new("../3.txt");
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

fn compute_paths(wire: &str, start: (i32, i32, i32)) -> BTreeSet<(i32, i32, i32)> {
    let moves = wire.split(',');
    let mut coords: BTreeSet<(i32, i32, i32)> = BTreeSet::new();
    let mut total_moves: i32 = 0;
    let mut start = (0, 0, 0);
    //coords.push(start);
    for m in moves {
        let direction = m.chars().nth(0).unwrap();
        let steps = m[1..].parse::<i32>().unwrap();
        if direction == 'U' {
            let new_start = (start.0 + steps, start.1, total_moves);
            let path = (start.0..new_start.0).map(|x| {
                total_moves += 1;
                (x, start.1, total_moves)
            });
            coords.extend(path);
            start = new_start;
        } else if direction == 'D' {
            let new_start = (start.0 - steps, start.1, total_moves);
            let path = (new_start.0..start.0).map(|x| {
                total_moves += 1;
                (x, start.1, total_moves)
            });
            coords.extend(path);
            start = new_start;
        } else if direction == 'L' {
            let new_start = (start.0, start.1 - steps, total_moves);
            let path = (new_start.1..start.1).map(|x| {
                total_moves += 1;
                (start.0, x, total_moves)
            });
            coords.extend(path);
            start = new_start;
        } else if direction == 'R' {
            let new_start = (start.0, start.1 + steps, total_moves);
            let path = (start.1..new_start.1).map(|x| {
                total_moves += 1;
                (start.0, x, total_moves)
            });
            coords.extend(path);
            start = new_start;
        }
        coords.insert(start);
        //  println!("start {:?} {:?} {:?}", start, direction, steps);
    }
    coords
}

fn main() {
    let s = read_input();
    let split: Vec<&str> = s.split_whitespace().collect();
    let wire1 = split[0];
    let wire2 = split[1];
    let path1 = compute_paths(wire1, (0, 0, 0));
    let path2 = compute_paths(wire2, (0, 0, 0));
    let mut distances: Vec<_> = path1
        .iter()
        .filter(|y| path2.contains(y))
        .map(|y| y.0.abs() + y.1.abs())
        .collect();

    //  println!("{:?} \n\n {:?}", path1, path2);
    distances.sort();
    println!("distances {:?}", distances);
    // Second answer
    //    println!("result: answer 1 {} answer 2 {}", res, res2);
}
