mod  read_file;

#[derive(Clone, Debug, Eq)]
struct Moon{
    pos: Vec<i32>,
    velocity: Vec<i32>
}

impl Moon{
    fn new(data: &str) -> Moon {
        let len = data.len() - 1;
        let positions = data[1..len].split(',')
            .map(|x| x.split('=').collect::<Vec<_>>())
            .filter(|x| x.len() > 1)
            .map(|x| {x[1].parse::<i32>().unwrap()})
            .collect::<Vec<i32>>();
        Moon {
            pos: positions.clone(),
            velocity: vec![0; 3]
        }
    }

    fn apply_gravity(&mut self, other_moon: &Vec<Moon>){
        for moon in other_moon{
            if *moon != *self{
            for i in 0..3{
                if self.pos[i] < moon.pos[i]{
                    self.velocity[i] += 1;
                } else if self.pos[i] > moon.pos[i]{
                    self.velocity[i] -= 1;
                }
            }
   //     println!("{:?} {:?}", self, moon);
            }
        }
        //println!("\n{:?}", self);
    }

    fn apply_velocity(&mut self){
        for i in 0..3{
            self.pos[i] += self.velocity[i];
        } 
    }

    fn compute_energy(&self) -> u32{
        let pot = self.pos.iter().fold(0, |acc, x| acc + x.abs());
        let kit = self.velocity.iter().fold(0, |acc, x| acc + x.abs());
        (pot * kit) as u32
    }

}
    fn compare(first: &Vec<Moon>, other: &Vec<Moon>) -> bool{
       for (a, b) in first.iter().zip(other){
           if a != b {
               return false
           }
       }
       return true
    }

impl PartialEq for Moon{
    fn eq(&self, other: &Moon) -> bool{
        for i in 0..3{
            if self.pos[i] != other.pos[i]{
                return false
            }
        }
        return true
    }
}

fn first_answer(orbits: &mut Vec<Moon>) -> u32{
    for _n in 0..1000 {
        let new_orbits = orbits.clone();
        for moon in orbits.iter_mut(){
            moon.apply_gravity(&new_orbits);
            moon.apply_velocity();
        }
    }
    let result = orbits.iter().fold(0, |acc, x| acc + x.compute_energy());
    println!("{}", result);
    result 
}


fn second_answer(orbits: &mut Vec<Moon>) -> u32{
    let mut n = 0;
    let start = orbits.clone();

    loop {
        let new_orbits = orbits.clone();
        for moon in orbits.iter_mut(){
            moon.apply_gravity(&new_orbits);
            moon.apply_velocity();
        }
        if compare(&start, &orbits){
           // println!("{:?}", start);
           // println!("{:?}", orbits);
            break
        }
        n += 1;
    }
    println!("{}", n);
    n
}

fn main() {
    let fileinput = read_file::read_input("../12.txt");
    let mut orbits = fileinput.lines().map(|x| Moon::new(x)).collect::<Vec<Moon>>();
    first_answer(&mut orbits.clone());
    second_answer(&mut orbits.clone());
}
