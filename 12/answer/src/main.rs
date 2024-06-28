use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
mod read_file;

#[derive(Clone, Debug, Eq)]
struct Moon {
    pos: Vec<i32>,
    velocity: Vec<i32>,
}

struct Cyclicity {
    cycle: i64,
    index: usize,
}

impl Moon {
    fn new(data: &str) -> Moon {
        let len = data.len() - 1;
        let positions = data[1..len]
            .split(',')
            .map(|x| x.split('=').collect::<Vec<_>>())
            .filter(|x| x.len() > 1)
            .map(|x| x[1].parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Moon {
            pos: positions.clone(),
            velocity: vec![0; 3],
        }
    }

    fn apply_gravity(&mut self, other_moon: &Vec<Moon>) {
        for moon in other_moon {
            if *moon != *self {
                for i in 0..3 {
                    if self.pos[i] < moon.pos[i] {
                        self.velocity[i] += 1;
                    } else if self.pos[i] > moon.pos[i] {
                        self.velocity[i] -= 1;
                    }
                }
            }
        }
    }

    fn apply_velocity(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.velocity[i];
        }
    }

    fn compute_energy(&self) -> u32 {
        let pot = self.pos.iter().fold(0, |acc, x| acc + x.abs());
        let kit = self.velocity.iter().fold(0, |acc, x| acc + x.abs());
        (pot * kit) as u32
    }
}
fn compare_partial(first: &Vec<Moon>, other: &Vec<Moon>, index: usize) -> bool {
    for (a, b) in first.iter().zip(other) {
        if a.pos[index] != b.pos[index] {
            return false;
        }
        if a.velocity[index] != b.velocity[index] {
            return false;
        }
    }
    return true;
}

impl PartialEq for Moon {
    fn eq(&self, other: &Moon) -> bool {
        for i in 0..3 {
            if self.pos[i] != other.pos[i] {
                return false;
            }
        }
        return true;
    }
}

fn pgcd(a: i64, b: i64) -> i64 {
    let leftover = a % b;
    match leftover {
        0 => b,
        1 => 1,
        _ => pgcd(b, leftover),
    }
}

fn pgcm(a: i64, b: i64) -> i64 {
    let divisor = pgcd(a, b);
    a * b / divisor
}

fn first_answer(orbits: &mut Vec<Moon>) -> u32 {
    for _n in 0..1000 {
        let new_orbits = orbits.clone();
        for moon in orbits.iter_mut() {
            moon.apply_gravity(&new_orbits);
            moon.apply_velocity();
        }
    }
    let result = orbits.iter().fold(0, |acc, x| acc + x.compute_energy());
    println!("{}", result);
    result
}

fn astro_simulator(orbits: &mut Vec<Moon>, tx: Sender<Cyclicity>) {
    let start = orbits.clone();
    let mut n = 1;

    loop {
        let new_orbits = orbits.clone();
        for moon in orbits.iter_mut() {
            moon.apply_gravity(&new_orbits);
            moon.apply_velocity();
        }
        for i in 0..3 {
            if compare_partial(&start, &orbits, i) {
                tx.send(Cyclicity { cycle: n, index: i });
            }
        }
        n += 1;
    }
}

fn compute_possibilities(rx: Receiver<Cyclicity>) -> () {}

fn second_answer(orbits: &mut Vec<Moon>) -> i64 {
    let start = orbits.clone();
    let (mut n, mut count) = (1, 0);
    let mut cycles = vec![0; 3];
    let (tx, rx): (Sender<Cyclicity>, Receiver<Cyclicity>) = mpsc::channel();

    loop {
        let new_orbits = orbits.clone();

        for moon in orbits.iter_mut() {
            moon.apply_gravity(&new_orbits);
            moon.apply_velocity();
        }

        for i in 0..3 {
            if compare_partial(&start, &orbits, i) && cycles[i] == 0 {
                cycles[i] = n;
                /*
                            println!("{:?} {:?}", i, n);
                            println!("{:?}", start);
                            println!("{:?}", orbits);
                */
                count += 1;
            }
        }

        n += 1;

        if start == *orbits || count >= 3 {
            break;
        }
    }
    println!("{:?}", pgcm(pgcm(cycles[0], cycles[1]), cycles[2]));
    n
}

fn main() {
    let fileinput = read_file::read_input("../12.txt");
    let mut orbits = fileinput
        .lines()
        .map(|x| Moon::new(x))
        .collect::<Vec<Moon>>();
    first_answer(&mut orbits.clone());
    second_answer(&mut orbits.clone());
}
