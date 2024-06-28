use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
struct asteroid {
    x: i32,
    y: i32,
    hit: usize,
}

#[derive(Debug, Clone)]
struct AstroMap {
    coords: Vec<asteroid>,
}

impl PartialEq for asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl asteroid {
    fn update_count(&mut self, count: usize) {
        self.hit = count;
    }

    // function that count how much of asteroids hid the present one
    fn count_concealers(&mut self, asteroids: &Vec<asteroid>) -> () {
        for object in asteroids {
            if self != object {
                if self.y * object.x - object.y * self.x == 0 {
                    if self.x * object.x > 0 || self.y * object.y > 0 {
                        if self.x.abs() > object.x.abs() || self.y.abs() > object.y.abs() {
                            self.hit += 1;
                        }
                    }
                }
            }
        }
    }

    // Compare asteroids positions on lazer path
    fn compare_positions(coord1: &(f64, asteroid), coord2: &(f64, asteroid)) -> Ordering {
        if coord1.1.hit != coord2.1.hit {
            coord1.1.hit.cmp(&coord2.1.hit)
        } else {
            coord2.0.partial_cmp(&coord1.0).unwrap()
        }
    }

    //  Compute thr angle with the help of atan2
    fn compute_angless(&self) -> f64 {
        (self.x as f64).atan2(self.y as f64)
    }
}

impl AstroMap {
    fn new() -> AstroMap {
        AstroMap { coords: Vec::new() }
    }

    // Function that return a cartesian map where the center is the given asteroid (coord)
    fn relative_map(&self, coord: &asteroid) -> AstroMap {
        let mut relative_map = AstroMap::new();
        for asteroid in &self.coords {
            if asteroid != coord {
                relative_map.coords.push(asteroid {
                    x: asteroid.x - coord.x,
                    y: asteroid.y - coord.y,
                    hit: 0,
                });
            }
        }
        relative_map
    }

    // Function that return all the asteroid that are not hidden
    fn find_hidden_asteroids(&mut self) -> Vec<&asteroid> {
        self.coords
            .iter()
            .filter(|&x| hidden(x, &self.coords))
            .map(|x| x)
            .collect()
    }

    // Function that order asteroid by quadrant (360). The one is the backs get hit later
    fn order_asteroids(&mut self) -> Vec<(f64, asteroid)> {
        let mut asteroid_left: Vec<(f64, asteroid)> = Vec::new();
        for asteroid in self.coords.clone().iter_mut() {
            asteroid.count_concealers(&mut self.coords);
            let angle = asteroid.compute_angless();
            asteroid_left.push((angle, asteroid.clone()));
        }
        asteroid_left.sort_by(|a, b| asteroid::compare_positions(&a, &b));
        asteroid_left
    }

    // Function that return the number of seen asteroids from a given one:
    fn visible_asteroids(&self, coord: &asteroid) -> usize {
        let hiddens_one = self.relative_map(coord).find_hidden_asteroids().len();
        // Number of asteroides - hidden_ones from the current asteroid - the one i am on
        return self.coords.len() - hiddens_one - 1;
    }
}

// function that check if an asteroid is hidden
fn hidden(asteroid: &asteroid, asteroids: &Vec<asteroid>) -> bool {
    for object in asteroids {
        if asteroid != object {
            if asteroid.y * object.x - object.y * asteroid.x == 0 {
                if asteroid.x * object.x > 0 || asteroid.y * object.y > 0 {
                    if asteroid.x.abs() > object.x.abs() || asteroid.y.abs() > object.y.abs() {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

fn first_answer(map: &AstroMap) -> (usize, &asteroid) {
    let mut line_of_sights: Vec<(usize, &asteroid)> = Vec::new();

    for asteroid in &map.coords {
        let number_seen = map.visible_asteroids(&asteroid);
        line_of_sights.push((number_seen, &asteroid));
    }

    *line_of_sights.iter().max_by_key(|x| x.0).unwrap()
}

fn second_answer(map: &mut AstroMap, start: &asteroid) -> u32 {
    let mut map = map.relative_map(&start);
    let ordered = map.order_asteroids();
    ((ordered[199].1.x + start.x) * 100) as u32 + (ordered[199].1.y + start.y) as u32
}

fn main() {
    let file_input = read_input("../10.txt");
    //Get the right data from the input
    let mut map = AstroMap::new();

    for (index, line) in file_input.split_whitespace().enumerate() {
        let y = index;
        let new_coords = line
            .chars()
            .enumerate()
            .filter(|s| s.1 == '#')
            .map(|(i, _)| asteroid {
                x: i as i32,
                y: y as i32,
                hit: 0,
            });
        map.coords.extend(new_coords);
    }
    let result1 = first_answer(&map);
    let result2 = second_answer(&mut map.clone(), &result1.1);
    println!("{:?} {:?}", result1, result2);
}

fn read_input(file_location: &str) -> std::string::String {
    let path = Path::new(file_location);
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
