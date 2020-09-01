use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
// Used for Permutation
use itertools::Itertools;
// Nultithreading
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;


#[derive(Debug)]
struct Data {
    // The 'a defines a lifetime
    memory:  Vec<i32>,
    memory_index: usize,
    input: Vec<i32>,
    input_index: usize,
    output: i32,
    input_channel: Option<Receiver<i32>>,
    output_channel: Option<Sender<i32>>
}

#[derive(Debug)]
#[derive(Clone)]
struct OutputSignal {
    phases: Vec<i32>,
    signal: i32
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
        //     println!("{:?}", data.memory[address]);
        data.output = data.memory[address]
    } else {
        let address = data.memory[*index + 1] as usize;
        //     println!("{:?}", data.memory[address]);
        data.output = data.memory[address]
    }
    let ouput_entry = &data.output_channel;
    *index += 2;
    match ouput_entry {
        Some(rx) => { rx.send(data.output); }
        _ => return
    }
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
    if data.input.len() <= data.input_index {
        let input = &data.input_channel;
        match input{
            Some(rx) => { data.input.push(rx.recv().unwrap())},
            _ => return
        }
    }
    if positions[2] == '1'{
        let address = *index + 1;
        data.memory[address] = data.input[data.input_index];
    } else {
        let address = data.memory[(*index + 1)] as usize;
        data.memory[address] = data.input[data.input_index]; 
    }
    data.input_index += 1;
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


fn run_data(data: &mut Data) -> usize {
    let mut index: usize = data.memory_index;
    let len = data.memory.len();

    while index < len {

        let instructions = data.memory[index].parse_decimal(); 
        match instructions.as_slice() {
            [_, _, _, _, '1'] => add(data, &mut index, instructions),
            [_, _, _, _, '2'] => multiply(data, &mut index, instructions),
            [_, _, _, _, '3'] => store(data, &mut index, instructions),
            [_, _, _, _, '4'] => display(data, &mut index, instructions),
            [_, _, _, _, '5'] => jump_if_true(data, &mut index, instructions),
            [_, _, _, _, '6'] => jump_if_false(data, &mut index, instructions),
            [_, _, _, _, '7'] => less_than(data, &mut index, instructions),
            [_, _, _, _, '8'] => equal(data, &mut index, instructions),
            [_, _, _, '9', '9'] => { break },
            _ => { println!("error"); break }
        }
    }
    index
}


fn connect_amps(input_amp: &mut Data, output_amp: &mut Data) -> () {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    output_amp.output_channel = Some(tx);
    input_amp.input_channel = Some(rx);
}


fn main() {
    //Read input and split by lines
    let file_input = read_input("../7.txt");
    //Get the right data from the input
    let input: &str = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => ""
    };
    //println!(" The input is: {:?}", input);
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


