use std::collections::BTreeMap;
use std::cmp::Ordering;

use std::thread;
mod intcode;
mod read_file;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Move{
    Left,
    Right
}

#[derive(Clone)]
#[derive(Debug)]
enum Orientation {
    North,
    East,
    South,
    West
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Color{
    Black = 0,
    White = 1
}

impl Color{
    fn compute_color(value: i64) -> Color{
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("color not existing")
        }
    }
}

struct Robot {
    pos: Coord,
    path: BTreeMap<Coord, Color>
}

#[derive(Clone)]
#[derive(Debug)]
struct  Coord{
    x: i64,
    y: i64,
    orientation: Orientation
}

impl PartialEq for Coord{
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Coord{
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        Some((self.x, self.y).cmp(&(other.x, other.y)))
    }
}

impl Ord for Coord{
    fn cmp(&self, other: &Coord) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}


impl Eq for Coord{
}

impl Coord{
    fn compute_move(&mut self, m: Move){
        match (&self.orientation, m) {
            (Orientation::West, Move::Left) | (Orientation::East, Move::Right) =>  { self.orientation = Orientation::South; self.y -= 1 }
            (Orientation::West, Move::Right) | (Orientation::East, Move::Left) => { self.orientation = Orientation::North; self.y += 1 }
            (Orientation::North, Move::Left) | (Orientation::South, Move::Right) => { self.orientation = Orientation::West; self.x -= 1 }
            (Orientation::North, Move::Right) | (Orientation::South, Move::Left) => {self.orientation = Orientation::East; self.x += 1}
        }
    }
}

impl Robot {
    fn new() -> Robot {
        Robot {
            pos: Coord {x: 0, y: 0, orientation: Orientation::North},
            path: BTreeMap::new()
        }
    }

    fn apply_move(&mut self, m: Move, c: Color) -> () {
        self.path.insert(self.pos.clone(), c);
        self.pos.compute_move(m);
    }

    fn detect_color(&mut self) -> Color{
        match self.path.get(&self.pos){
            Some(x) => *x,
            None => Color::Black,
        }
    }
}

fn first_answer(intcode: Vec<i64>) -> usize {
    let mut data = intcode::Data::new(intcode, vec![0]);
    let rx = data.get_input_channel();
    let tx = data.get_output_channel();
    let  code = thread::spawn(move || {
        intcode::Data::run_data(&mut data)
    });
    let mut paint = true;
    let mut color_to_paint = Color::Black;
    let mut automat = Robot::new();

    for p in tx.iter(){
       // println!("{:?}", p);
        if paint == false {
           let m =  match p {
               0 => Move::Left,
               1 => Move::Right,
               _ => return 0
            };
         //  println!(" move {:?}", m);
          // println!("detected color {:?}", automat.detect_color());
            automat.apply_move(m, color_to_paint);
            rx.send(automat.detect_color() as i64).unwrap();
        } else {
          // println!("color_to_paint {:?}", automat.detect_color());
            color_to_paint = Color::compute_color(p);
        }
        paint =!paint
    }
    code.join().unwrap();
    automat.path.len()
}

fn main() {
    let file_input = read_file::read_input("../11.txt");
    let input = match file_input.split_whitespace().next(){
        Some(s) =>s,
        None => ""
    };
    let intcode: Vec<i64> = input.split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let result = first_answer(intcode);
    println!("{:?}", result);
}
