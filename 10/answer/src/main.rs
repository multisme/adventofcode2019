use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


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
}

impl AstroMap {
    fn new() -> AstroMap {
        AstroMap {
            coords: Vec::new(),
        }
    }

    fn relative_map(&self, coord: &asteroid) -> AstroMap {
        let mut relative_map = AstroMap::new();
        for asteroid in &self.coords {
            if asteroid != coord{
                relative_map.coords.push(asteroid{x: asteroid.x - coord.x, y: asteroid.y - coord.y, hit: 0});
            }
        }
        //println!("{:?}", relative_map.coords);
        relative_map
    }

    fn find_hidden_asteroids(&mut self) -> Vec<&asteroid> {
        self.coords.iter()
            .filter(|&x| hidden(x, &self.coords))
            .map(|x| x)
            .collect()
    }

    fn removed_asteroid(&mut self) -> Vec<&asteroid> {
        let size = self.coords.len();
        let mut removed: Vec<&asteroid> = Vec::new();
        let mut count: usize = 1;

        for asteroid in &mut self.coords.iter(){
            if !hidden(asteroid, &self.coords){
                removed.push(&asteroid);
                //println!("{:?}", asteroid);
                count +=1;
            }
            if count >= size{
                break
            }
        }
        removed 
    }

    fn visible_asteroids(&self, coord: &asteroid) -> usize {
        let hiddens_one = self.relative_map(coord)
            .find_hidden_asteroids()
            .len();
        // Number of asteroides - hidden_ones from the current asteroid - the one i am on
        return self.coords.len() - hiddens_one - 1
    }
}


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

/*
   impl Ord for (usize, asteroid){
   ²    fn cmp(&self, other: &(usize, asteroid)){
   self.0.cmp(other.0)
   }
   }
   */

fn first_answer(map: &AstroMap) -> (usize, asteroid){
    let mut line_of_sights: Vec<(usize, asteroid)> = Vec::new();

    
       for asteroid in map.coords.clone(){
       let number_seen = map.visible_asteroids(&asteroid);
       line_of_sights.push((number_seen, asteroid.clone()));
       }
       
    let number_seen = map.visible_asteroids(&asteroid{x: 11, y: 13, hit: 0});
    let start = asteroid{x: 0, y: 0, hit: 0};
    line_of_sights.iter().fold((0, start), |acc, x| if acc.0 > x.0 {acc} else {x.clone()})
}

fn second_answer(map: &mut AstroMap) -> u32 {
    let start = asteroid{x: 5, y: 8, hit: 0 };
    let mut map = map.relative_map(&start);
    let removed = map.removed_asteroid();
    //println!("{:?}", removed);
    //((removed[199].x + 37) * 100) as u32 + (removed[199].y + 25) as u32
    //
    0
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
            .map(|(i, _)| asteroid{x: i as i32, y: y as i32, hit: 0});
        //.collect::<Vec<asteroid>>();
        map.coords.extend(new_coords);
    }
    let result1 = first_answer(&map);
    let result2 = 0;//second_answer(&mut map);
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
