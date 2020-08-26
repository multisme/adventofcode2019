use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


struct Data<'a> {
    // The 'a defines a lifetime
    memory: &'a mut Vec<i32>,
    input: i32
}

fn add(data: &mut Data, index: &mut usize,  positions: Vec<char> ) ->(){
    //  println!("instructions {:?} {:?} {:?}", memory[index], index, positions);
    let output = data.memory[*index + 3] as usize;
    let mark = data.memory[*index + 2];
    let val2 = if positions[1] == '0' { data.memory[mark as usize] } else {mark} ;
    let mark = data.memory[*index + 1];
    let val1 = if positions[2] == '0' { data.memory[mark as usize] } else {mark} ;
    data.memory[output as usize] = val1 + val2;
    *index += 4;
}


fn display(data: &mut Data, index: &mut usize,  positions: Vec<char>) ->(){
    if positions[2] == '1'{
        let address = *index + 1;
        println!("{:?}", data.memory[address]);
    } else {
        let address = data.memory[*index + 1] as usize;
        println!("{:?}", data.memory[address]);
    }
    *index += 2;
}


fn jump_if_true(data: &mut Data, index: &mut usize,  positions: Vec<char>) ->(){
    let address = if positions[2] == '0' { data.memory[*index + 1] as usize } else { *index + 1 };
    if data.memory[address] != 0 {
        let address = if positions[1] == '0' { data.memory[*index + 2] as usize} else { *index + 2 };
        *index = data.memory[address] as usize;
    } else {
        *index += 3;
    }
}


fn jump_if_false(data: &mut Data, index: &mut usize,  positions: Vec<char>) ->(){
    let address = if positions[2] == '0' { data.memory[*index + 1] as usize} else { *index + 1 };
    if data.memory[address] == 0 {
        let address = if positions[1] == '0' { data.memory[*index + 2] as usize} else { *index + 2 };
        *index = data.memory[address] as usize;
    } else {
        *index += 3;
    }
}


fn less_than(data: &mut Data, index: &mut usize,  positions: Vec<char>) ->(){

    let address = if positions[3] == '0'{ data.memory[*index + 3] as usize } else { *index + 3 };
    let mark = data.memory[*index + 2];
    let val2 = if positions[1] == '0' { data.memory[mark as usize] } else {mark} ;
    let mark = data.memory[*index + 1];
    let val1 = if positions[2] == '0' { data.memory[mark as usize] } else {mark} ;
    data.memory[address] = if val1 < val2 { 1 } else { 0 };
    *index += 4;
}


fn equal(data: &mut Data, index: &mut usize,  positions: Vec<char>) ->(){
    let address = if positions[3] == '0'{ data.memory[*index + 3] as usize } else { *index + 3 };
    let mark = data.memory[*index + 2];
    let val2 = if positions[1] == '0' { data.memory[mark as usize] } else {mark} ;
    let mark = data.memory[*index + 1];
    let val1 = if positions[2] == '0' { data.memory[mark as usize] } else {mark} ;
    data.memory[address] = if val1 == val2 { 1 } else { 0 };
    *index += 4;
}


fn multiply(data: &mut Data, index: &mut usize,  positions: Vec<char> ) ->(){
    //  println!("instructions {:?} {:?} {:?}", data.memory[index], index, positions);
    let output = data.memory[*index + 3] as usize;
    let mark = data.memory[*index + 2];
    let val2 = if positions[1] == '0' { data.memory[mark as usize] } else {mark} ;
    let mark = data.memory[*index + 1];
    let val1 = if positions[2] == '0' { data.memory[mark as usize] } else {mark} ;
    data.memory[output as usize] = val1 * val2;
    *index += 4;
}


fn _nothing(_data: &mut Data, _index: &mut usize,  _positions: Vec<char> ) ->(){
}


fn store(data: &mut Data, index: &mut usize,  positions: Vec<char> ) ->(){
    if positions[2] == '1'{
        let address = *index + 1;
        data.memory[address] = data.input;
    } else {
        let address = data.memory[(*index + 1)] as usize;
        data.memory[address] = data.input; 
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


fn run_data(data: &mut Data) -> (){
    let mut index: usize = 0;
    let len = data.memory.len();

    while index < len {

        let positions = data.memory[index].parse_decimal(); 
        let instruction = positions[4];
        if instruction == '1'{
            add(data, &mut index, positions);
        } else if instruction == '2'{
            multiply(data, &mut index, positions);
        }else if instruction == '3' {
            store(data, &mut index, positions);
        } else if instruction == '4' {
            display(data, &mut index, positions);
        } else if instruction == '5'{
            jump_if_true(data, &mut index, positions);
        } else if instruction == '6'{
            jump_if_false(data, &mut index, positions);
        } else if instruction == '7'{
            less_than(data, &mut index, positions);
        } else if instruction == '8'{
            equal(data, &mut index, positions);
        } else if instruction == '9' {
            return 
        } else{
            println!("error");
            return;
        }
//            println!("{:?} {:?}", data, index);
    }
}


fn read_input(file_location: &str) -> std::string::String {
    //  let path = Path::new("../4.test0.txt"); //test file
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


fn main() {
    //Read input and split by lines
    let file_input = read_input("../5.1.txt");
    //Get the right data from the input
    let input: &str = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => ""
    };
    //println!(" The input is: {:?}", input);
    let memory: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    // Input is 1
    let mut data = Data {memory: &mut memory.clone(), input: 1};
    run_data(&mut data);
    //Input is 5
    let mut data = Data {memory: &mut memory.clone(), input: 5};
    run_data(&mut data);
}
