use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum Case {
    Asteroid,
    EmptySpace
}

#[derive(Debug)]
#[derive(Clone)]
struct AstroMap{
    coords: Vec<(i32, i32)>,
}

impl AstroMap {
    fn new() -> AstroMap {
        AstroMap {
            coords: Vec::new(),
        }
    }
    
    fn relative_map(&self, coord: (i32, i32)) -> AstroMap {
        let mut relative_map = AstroMap::new();
        for asteroid in &self.coords {
            if *asteroid != coord{
                relative_map.coords.push((asteroid.0 - coord.0, asteroid.1 - coord.1));
            }
        }
       //println!("{:?}", relative_map.coords);
        relative_map
    }

    fn find_hidden_asteroids(&mut self) -> &AstroMap {
        self.coords = self.coords.iter()
            .filter(|&x| hidden(x, self.coords.clone()))
            .map(|x| *x)
            .collect();
        //println!("{:?}", self);
        self
    }

    fn visible_asteroids(&self, coord: (i32, i32)) -> usize {
        let hiddens_one = self.relative_map(coord)
            .find_hidden_asteroids()
            .coords.len();
        //println!("{:?} {:?} {:?}", self.coords.len(), hiddens_one + 1, coord);
        return self.coords.len() - hiddens_one - 1
    }
}

fn hidden(asteroid: &(i32, i32), asteroids: Vec<(i32, i32)>) -> bool {
     for object in asteroids{
        if *asteroid == object {
            return false
        }
        if asteroid.1 * object.0 - object.1 * asteroid.0 == 0{
                //println!("colinear {:?} {:?}", asteroid, object);
            if asteroid.0 * object.0 > 0 || asteroid.1 * object.1 > 0{
                if asteroid.0 > object.0 || asteroid.1 > object.1 {
                   // println!("i chosen colinear {:?} {:?}", asteroid, object);
                    return true
                }
            }
        }
    }
    return false
}

fn first_answer(map: &AstroMap) -> (usize, (i32, i32)){
    let mut line_of_sights: Vec<(usize, (i32, i32))> = Vec::new();

    
    for asteroid in map.coords.clone(){
        let number_seen = map.visible_asteroids(asteroid);
        line_of_sights.push((number_seen, asteroid));
    }
    
    let number_seen = map.visible_asteroids((8,5));
    println!("{:?}", number_seen);
   // (number_seen, (5,8))
   *line_of_sights.iter().max().unwrap()
}

fn main() {
    let file_input = read_input("../10.txt");
    //Get the right data from the input
    let mut map = AstroMap::new();
    
    for (index, line) in file_input.split_whitespace().enumerate() {
        let y = index;
        let new_coords = line.chars()
            .enumerate()
            .filter(|s| s.1 == '#')
            .map(|(i, _)| (y as i32, i as i32));
            //.collect::<Vec<(i32, i32)>>();
        map.coords.extend(new_coords);
    }
    let result = first_answer(&map);
    println!("{:?}", result);
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


