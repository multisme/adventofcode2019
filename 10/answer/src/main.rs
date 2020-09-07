use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::Ordering;


#[derive(Debug)]
#[derive(Clone)]
struct asteroid{
    x: i32,
    y: i32,
    hit: usize
}

#[derive(Debug)]
#[derive(Clone)]
struct AstroMap{
    coords: Vec<asteroid>,
}


impl PartialEq for asteroid{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl asteroid{
    fn update_count(&mut self, count: usize){
        self.hit  = count; 
    }

    fn cmp_asteroid(&self, other: &asteroid) -> Ordering {
        if self.hit != other.hit{
            return self.hit.cmp(&other.hit)
        } else {
            let origin = asteroid {x: 0, y:0, hit: 0};
            return origin.compare_angle(self).cmp(&origin.compare_angle(other));
        }
    }

    // function that count how much of asteroids hid the present one
    fn count_concealers(&mut self, asteroids: &Vec<asteroid>) -> () {
        for object in asteroids{
            if self != object {
                if self.y * object.x - object.y * self.x == 0{
                    if self.x * object.x > 0 || self.y * object.y > 0 {
                        if self.x.abs() > object.x.abs() || self.y.abs() > object.y.abs() {
                            self.hit += 1;
                        }
                    }
                }
            }
        }
    }

    fn compare_angle(&self, coord: &asteroid) -> Ordering {
        
        if self.x * coord.x < 0 {
            return self.x.cmp(&coord.x);
        } else if self.x == 0 && coord.x == 0 {
            if self.y > 0 && coord.y < 0{
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        } else {
            let cross_product =  self.y * coord.x - self.x * coord.y; 
            if cross_product > 0 {
                return Ordering::Greater
            } else if cross_product == 0 {
                return Ordering::Equal
            } else {
                return Ordering::Less
            }
        }
    }

}

impl AstroMap {
    fn new() -> AstroMap {
        AstroMap {
            coords: Vec::new(),
        }
    }

    // Function that return a cartesian map where the center is the given asteroid (coord) 
    fn relative_map(&self, coord: &asteroid) -> AstroMap {
        let mut relative_map = AstroMap::new();
        for asteroid in &self.coords {
            if asteroid != coord{
                relative_map.coords.push(asteroid{x: asteroid.x - coord.x, y: asteroid.y - coord.y, hit: 0});
            }
        }
        //pSuivrintln!("{:?}", relative_map.coords);
        relative_map
    }


    // Function that return all the asteroid that are not hidden
    fn find_hidden_asteroids(&mut self) -> Vec<&asteroid> {
        self.coords.iter()
            .filter(|&x| hidden(x, &self.coords))
            .map(|x| x)
            .collect()
    }


    // Function that order asteroid by quadrant (360). The one is the backs get hit later
    fn order_asteroid(&mut self) -> Vec<asteroid> {
        let size = self.coords.len();
        let mut removed: Vec<asteroid> = Vec::new();

        let mut asteroid_left: Vec<asteroid> = self.coords.clone();
        let mut not_hit: Vec<asteroid> = Vec::new();
        for asteroid in asteroid_left.iter_mut(){
            asteroid.count_concealers(&self.coords);
        }
        asteroid_left.sort_by(|a, b| a.cmp_asteroid(b));
        asteroid_left
    }

    // Function that return the number of seen asteroids from a given one:
    fn visible_asteroids(&self, coord: &asteroid) -> usize {
        let hiddens_one = self.relative_map(coord)
            .find_hidden_asteroids()
            .len();
        // Number of asteroides - hidden_ones from the current asteroid - the one i am on
        return self.coords.len() - hiddens_one - 1
    }
}

// function that check if an asteroid is hidden
fn hidden(asteroid: &asteroid, asteroids: &Vec<asteroid>) -> bool {
    for object in asteroids{
        if asteroid != object {
            if asteroid.y * object.x - object.y * asteroid.x == 0{
                if asteroid.x * object.x > 0 || asteroid.y * object.y > 0{
                    if asteroid.x.abs() > object.x.abs() || asteroid.y.abs() > object.y.abs() {
                        return true
                    }
                }
            }
        }
    }
    return false
}


fn first_answer(map: &AstroMap) -> (usize, &asteroid){
    let mut line_of_sights: Vec<(usize, &asteroid)> = Vec::new();


    for asteroid in &map.coords{
        let number_seen = map.visible_asteroids(&asteroid);
        line_of_sights.push((number_seen, &asteroid));
    }

    let start = asteroid{x: 0, y: 0, hit: 0};
    *line_of_sights.iter().max_by_key(|x| x.0).unwrap()
}

fn second_answer(map: &mut AstroMap, start: &asteroid) -> u32 {
    let mut map = map.relative_map(&start);
    let ordered = map.order_asteroid();
  //  println!(" ordered {:?}", (ordered[199].x + start.x, ordered[199].y + start.y));
    for o in &ordered{
       println!(" ordered {:?}", asteroid{x:o.x, y: o.y, hit: o.hit});
     //   println!(" ordered {:?}", asteroid{x:o.x + start.x, y: o.y + start.y, hit: o.hit});
    }
    //((ordered[199].x + start.x) * 100) as u32 + (ordered[199].y + start.y) as u32
     0
}


fn main() {
    let file_input = read_input("../10.4.txt");
    //Get the right data from the input
    let mut map = AstroMap::new();

    for (index, line) in file_input.split_whitespace().enumerate() {
        let y = index;
        let new_coords = line.chars()
            .enumerate()
            .filter(|s| s.1 == '#')
            .map(|(i, _)| asteroid{x: i as i32, y: y as i32, hit: 0});
        map.coords.extend(new_coords);
    }
    let result1 = first_answer(&map);
    //let result2 = second_answer(&mut map, &result1.1);
    let result2 = 0;
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
