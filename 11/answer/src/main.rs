use std::cmp::Ordering;
use std::collections::BTreeMap;

use std::thread;
mod intcode;
mod read_file;

#[derive(Clone, Copy, Debug)]
enum Move {
    Left,
    Right,
}

#[derive(Clone, Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black = 0,
    White = 1,
}

impl Color {
    fn compute_color(value: i64) -> Color {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("color not existing"),
        }
    }
}

struct Robot {
    pos: Coord,
    path: BTreeMap<Coord, Color>,
}

#[derive(Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
    orientation: Orientation,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        Some((self.x, self.y).cmp(&(other.x, other.y)))
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Coord) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl Eq for Coord {}

impl Coord {
    fn compute_move(&mut self, m: Move) {
        match (&self.orientation, m) {
            (Orientation::West, Move::Left) | (Orientation::East, Move::Right) => {
                self.orientation = Orientation::South;
                self.y -= 1
            }
            (Orientation::West, Move::Right) | (Orientation::East, Move::Left) => {
                self.orientation = Orientation::North;
                self.y += 1
            }
            (Orientation::North, Move::Left) | (Orientation::South, Move::Right) => {
                self.orientation = Orientation::West;
                self.x -= 1
            }
            (Orientation::North, Move::Right) | (Orientation::South, Move::Left) => {
                self.orientation = Orientation::East;
                self.x += 1
            }
        }
    }
}

impl Robot {
    fn new() -> Robot {
        Robot {
            pos: Coord {
                x: 0,
                y: 0,
                orientation: Orientation::North,
            },
            path: BTreeMap::new(),
        }
    }

    fn apply_move(&mut self, m: Move, c: Color) -> () {
        self.path.insert(self.pos.clone(), c);
        self.pos.compute_move(m);
    }

    fn detect_color(&mut self) -> Color {
        match self.path.get(&self.pos) {
            Some(x) => *x,
            None => Color::Black,
        }
    }
}

impl Move {
    fn compute_move(m: i64) -> Move {
        match m {
            0 => Move::Left,
            1 => Move::Right,
            _ => panic!("move not existing"),
        }
    }
}

fn paint(automat: &mut Robot, intcode: &Vec<i64>, input: Vec<i64>) -> () {
    let mut paint = true;
    let mut color_to_paint = Color::Black;
    let mut data = intcode::Data::new(intcode.clone(), input);
    let rx = data.get_input_channel();
    let tx = data.get_output_channel();

    // Run the intcode
    let code = thread::spawn(move || intcode::Data::run_data(&mut data));

    // Get the result of the intcode and process it as either a move or a painting action
    for p in tx.iter() {
        if paint == false {
            let m = Move::compute_move(p);
            automat.apply_move(m, color_to_paint);
            rx.send(automat.detect_color() as i64).unwrap();
        } else {
            color_to_paint = Color::compute_color(p);
        }
        paint = !paint
    }
    code.join().unwrap();
}

fn first_answer(intcode: &Vec<i64>) -> usize {
    let mut automat = Robot::new();
    paint(&mut automat, &intcode, vec![0]);
    let result = automat.path.len();
    println!("{:?}", result);
    result
}

fn second_answer(intcode: &Vec<i64>) -> () {
    let mut automat = Robot::new();
    paint(&mut automat, &intcode, vec![1]);

    let len = automat.path.len() as i64;
    let mut result = String::new();
    let xs = &automat.path.iter().map(|x| x.0.x);
    let (startx, endx) = (xs.clone().min().unwrap(), xs.clone().max().unwrap());
    let ys = &automat.path.iter().map(|y| y.0.y);
    let (starty, endy) = (ys.clone().min().unwrap(), ys.clone().max().unwrap());
    let (mut i, mut j) = (startx - 1, starty - 1);

    while i < endx + 1 {
        while j < endy + 1 {
            match automat.path.get(&Coord {
                x: i,
                y: j,
                orientation: Orientation::North,
            }) {
                Some(x) => match x {
                    Color::Black => result.push(' '),
                    Color::White => result.push('x'),
                },
                _ => result.push(' '),
            }
            j += 1;
        }
        result.push('\n');
        j = starty - 1;
        i += 1;
    }
    println!("{}", result);
}

fn main() {
    let file_input = read_file::read_input("../11.txt");
    let input = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => "",
    };
    let intcode: Vec<i64> = input
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    first_answer(&intcode);
    second_answer(&intcode);
}
