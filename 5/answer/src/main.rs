use std::fs::File;
use std::io::prelude::*;
use std::path::Path;



fn add(memory: &mut Vec<i32>, index: &mut usize,  positions: Vec<char> ) ->(){
          //  println!("instructions {:?} {:?} {:?}", memory[index], index, positions);
            let output = memory[*index + 3] as usize;
            let mark = memory[*index + 2];
            let val2 = if positions[1] == '0' { memory[mark as usize] } else {mark} ;
            let mark = memory[*index + 1];
            let val1 = if positions[2] == '0' { memory[mark as usize] } else {mark} ;
                memory[output as usize] = val1 + val2;
            *index += 4;
}

fn display(memory: &mut Vec<i32>, index: &mut usize,  positions: Vec<char>) ->(){
            if positions[2] == '1'{
                let address = *index + 1;
                println!("{:?}", memory[address]);
            } else {
                let address = memory[(*index + 1)] as usize;
                println!("{:?}", memory[address]);
            }
            *index += 2;
}

fn multiply(memory: &mut Vec<i32>, index: &mut usize,  positions: Vec<char> ) ->(){
          //  println!("instructions {:?} {:?} {:?}", memory[index], index, positions);
            let output = memory[*index + 3] as usize;
            let mark = memory[*index + 2];
            let val2 = if positions[1] == '0' { memory[mark as usize] } else {mark} ;
            let mark = memory[*index + 1];
            let val1 = if positions[2] == '0' { memory[mark as usize] } else {mark} ;
            memory[output as usize] = val1 * val2;
            *index += 4;
}

fn _nothing(_memory: &mut Vec<i32>, _index: &mut usize,  _positions: Vec<char> ) ->(){
}


fn store(memory: &mut Vec<i32>, index: &mut usize,  positions: Vec<char> ) ->(){
            if positions[2] == '1'{
                let address = *index + 1;
                memory[address] = 1;
            } else {
                let address = memory[(*index + 1)] as usize;
                memory[address] = 1; 
            }
            *index += 2;
}

trait To10ext {
    fn parse_decimal(&self) -> Vec<char>;
}

impl To10ext for i32 {
    fn parse_decimal(&self) -> Vec<char> {
        let result: Vec<char> = (self + 100000).to_string() // add 10 000 to be sure to catch the empty zeros before the int
            .chars()
            .collect();
        result[1..].to_vec() //need to find a way to remove the first value
    }
}

fn convert_to_intrucion(memory:&mut Vec<i32>) -> (){
    let mut index: usize = 0;
    let len = memory.len();

    while index < len {

        let positions = memory[index].parse_decimal(); 
        let instruction = positions[4];
        if instruction == '3' {
            store(memory, &mut index, positions);
        } else if instruction == '4' {
            display(memory, &mut index, positions);
        } else if instruction == '9' {
            return 
        } else if instruction == '2'{
            multiply(memory, &mut index, positions);
        } else if instruction == '1'{
            add(memory, &mut index, positions);
        }
    }
}

fn read_input() -> std::string::String {
    //  let path = Path::new("../4.test0.txt"); //test file
    let path = Path::new("../5.txt");
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

fn main() {
    //Read input and split by lines
    let file_input = read_input();
    //Get the right data from the input
    let input: &str = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => ""
    };
    //println!(" The input is: {:?}", input);
    let mut memory: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    convert_to_intrucion(&mut memory);
}
