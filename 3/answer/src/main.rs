use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
//use std::collections::BTreeMap;
use std::collections::BTreeMap;

fn read_input() -> std::string::String {
//  let path = Path::new("../3.test0.txt"); //test file
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

fn compute_paths(wire: &str, mut start: (i32, i32)) -> BTreeMap<(i32, i32), i32> {

    let moves = wire.split(',');
    let mut coords: BTreeMap<(i32, i32), i32> = BTreeMap::new();

    /*
     * For chaque turn, compute the path walked by the straight line before and store them in a
     * BtreeMap
     */
    let mut index: i32 = 0;
    for m in moves {
        let direction = m.chars().nth(0).unwrap();
        let steps = m[1..].parse::<i32>().unwrap();

        // Get for each path the coordinates of the points of that path and the amount of steps to
        // get to each of those coordinates
        
        //Managage up turn
        if direction == 'U'{
            let new_start = (start.0 + steps, start.1);
            let path = (start.0..new_start.0).enumerate().map(|(y, x)| {
                ((x, start.1), y as i32 + index)
                }
            );
            coords.extend(path);
            start = new_start;
        //Manage down turn
        } else if direction == 'D'{
            let new_start = (start.0 - steps, start.1);
            let path = (new_start.0..start.0).enumerate().map(|(y, x)| {
                ((x, start.1), steps - y as i32 + index)
            }
            );
            coords.extend(path);
            start = new_start;
        //Manage left turn
        } else if direction == 'L'{
            let new_start = (start.0, start.1 - steps);
            let path = (new_start.1..start.1).enumerate().map(|(y, x)| {
                ((start.0, x), steps - y as i32 + index)
            }
            );
            coords.extend(path);
            start = new_start;
        //Manage right turn
        } else if direction == 'R'{
            let new_start = (start.0, start.1 + steps);
            let path = (start.1..new_start.1).enumerate().map(|(y, x)| {
                ((start.0, x), y as i32 + index)
                }
            );
            coords.extend(path);
            start = new_start;
        }
        index += steps;
        coords.insert(start, index);
        // coords.insert(start);
    //    println!("start {:?} {:?} {:?} {:?}", start, direction, steps, index);
    }
    coords
}

fn main() {

    //Read input and split by lines
    let s = read_input();
    let split: Vec<&str> = s.split_whitespace().collect();

    let wire1 = split[0]; //Get turn coordinates on the first path
    let wire2 = split[1]; //Get turn coordinates on the second path

    //Computes the full paths
    let mut path1 = compute_paths(wire1, (0, 0));
    let mut path2 = compute_paths(wire2, (0, 0));
    
    //remove the start of the path
    path1.remove(&(0,0));
    path2.remove(&(0,0));

    // Get all the coordinates where the path crosses
    let crossings: BTreeMap<&(i32, i32), &i32> = path1
        .iter()
        .filter(|&(x, _y)| path2.contains_key(x))
        .map(|(x, y)| ((x,y)))
        .collect();
   
    // Get all the manhattan distances
    let distance: i32 = crossings
        .iter()
        .map(|(x, _y)| x.0.abs() + x.1.abs()) //Compute manhattan distances
        .min().unwrap();    //Find min of vector
    let res = distance;

    let distance: i32 = crossings
        .iter()
        .map(|(x, y)| (*y + path2.get(x).unwrap())) //Compute number of steps
        .min().unwrap();    //Find min of vector
    let res2 = distance;
    println!("result: answer 1 {} answer 2 {}", res, res2);
}
