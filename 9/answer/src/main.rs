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
    intcode:  Vec<i64>,
    rip: usize,
    relative_index: usize,
    input: Vec<i64>,
    input_index: usize,
    output: i64,
    r#in: Option<Receiver<i64>>,
    output_channel: Option<Sender<i64>>
}

#[derive(Debug)]
#[derive(Clone)]
struct OutputSignal {
    phases: Vec<i64>,
    signal: i64
}

trait To10ext {
    fn parse_decimal(&self) -> Vec<char>;
}


impl To10ext for i64 {
    fn parse_decimal(&self) -> Vec<char> {
        let result: Vec<char> = (self + 100000).to_string() // add 10 000 to be sure to catch the empty zeros before the int
            .chars()
            .collect();
        result[1..].to_vec() //need to find a way to remove the first value
    }
}

impl Data {
fn new(intcode: Vec<i64>, input: Vec<i64>) -> Data {
    Data {
        intcode: intcode,
        rip : 0,
        relative_index: 0,
        input: input,
        input_index: 0,
        output: 0,
        r#in: None,
        output_channel: None
    }
}

fn deref(data: &mut Data, index: usize, positions_mode: char) -> i64 {
    let address = Data::get_address(data, index, positions_mode);
    if address > data.intcode.len() {
        data.intcode.extend(vec![0; address - data.intcode.len() + 2])
    }
    data.intcode[address]
}

fn get_address(data: &mut Data, index: usize, positions_mode: char) -> usize {
    match positions_mode {
        '0' => data.intcode[index] as usize,
        '1' => index,
        '2' => { (data.relative_index as i64 + data.intcode[index]) as usize},
         _  => 0
    }
}

fn add(data: &mut Data, index: &mut usize,  positions: Vec<char> ) -> () {
    let output = data.intcode[*index + 3] as usize;
    let val2 = Data::deref(data, *index + 2, positions[1]);
    let val1 = Data::deref(data, *index + 1, positions[2]);
    data.intcode[output] = val1 + val2;
    *index += 4;
}

fn multiply(data: &mut Data, index: &mut usize,  positions: Vec<char> ) ->(){
    //  println!("instructions {:?} {:?} {:?}", data.intcode[index], index, positions);
    let output = data.intcode[*index + 3] as usize;
    let val2 = Data::deref(data, *index + 2, positions[1]);
    let val1 = Data::deref(data, *index + 1, positions[2]);
    data.intcode[output] = val1 * val2;
    *index += 4;
}


fn display(data: &mut Data, index: &mut usize,  positions: Vec<char>) -> () {
    data.output = Data::deref(data, *index + 1, positions[2]);
    let ouput_entry = &data.output_channel;
    println!("{:?}", data.output);
    *index += 2;
    match ouput_entry {
        Some(rx) => { rx.send(data.output).unwrap(); }
        _ => return
    }
}


fn jump_if_true(data: &mut Data, index: &mut usize,  positions: Vec<char>) -> () {
    if Data::deref(data, *index + 1, positions[2]) != 0 {
        *index = Data::deref(data, *index + 2, positions[1]) as usize;
    } else {
        *index += 3;
    }
}

fn jump_if_false(data: &mut Data, index: &mut usize,  positions: Vec<char>) -> () {
    if Data::deref(data, *index + 1, positions[2]) == 0 {
        *index = Data::deref(data, *index + 2, positions[1]) as usize;
    } else {
        *index += 3;
    }
}


fn less_than(data: &mut Data, index: &mut usize,  positions: Vec<char>) ->(){

    let address = Data::get_address(data, *index + 3, positions[3]) as usize;
    let val2 = Data::deref(data, *index + 2, positions[1]);
    let val1 = Data::deref(data, *index + 1, positions[2]);
    data.intcode[address] = if val1 < val2 { 1 } else { 0 };
    *index += 4;
}


fn equal(data: &mut Data, index: &mut usize,  positions: Vec<char>) ->(){
    let address = Data::get_address(data, *index + 3, positions[3]) as usize;
    let val2 = Data::deref(data, *index + 2, positions[1]);
    let val1 = Data::deref(data, *index + 1, positions[2]);
    data.intcode[address] = if val1 == val2 { 1 } else { 0 };
    *index += 4;
}


fn _nothing(_data: &mut Data, _index: &mut usize,  _positions: Vec<char> ) ->(){
}


fn store(data: &mut Data, index: &mut usize,  positions: Vec<char> ) ->(){
    if data.input.len() <= data.input_index {
        let input = &data.r#in;
        match input{
            Some(rx) => { data.input.push(rx.recv().unwrap())},
            _ => return
        }
    }
    let address = Data::get_address(data, *index + 1, positions[2]);
    data.intcode[address] = data.input[data.input_index];
    data.input_index += 1;
    *index += 2;
}

fn stack_change(data: &mut Data, index: &mut usize, positions: Vec<char>) -> () {
    data.relative_index += Data::get_address(data, *index + 1, positions[2]);
    *index += 2;
}

fn run_data(data: &mut Data) -> usize {
    let mut index: usize = data.rip;
    let len = data.intcode.len();

    while index < len {

        let instructions = data.intcode[index].parse_decimal(); 
        match instructions.as_slice() {
            [_, _, _, _, '1'] => Data::add(data, &mut index, instructions),
            [_, _, _, _, '2'] => Data::multiply(data, &mut index, instructions),
            [_, _, _, _, '3'] => Data::store(data, &mut index, instructions),
            [_, _, _, _, '4'] => Data::display(data, &mut index, instructions),
            [_, _, _, _, '5'] => Data::jump_if_true(data, &mut index, instructions),
            [_, _, _, _, '6'] => Data::jump_if_false(data, &mut index, instructions),
            [_, _, _, _, '7'] => Data::less_than(data, &mut index, instructions),
            [_, _, _, _, '8'] => Data::equal(data, &mut index, instructions),
            [_, _, _, '0', '9'] => Data::stack_change(data, &mut index, instructions),
            [_, _, _, '9', '9'] => { break },
            _ => { println!("error {:?}, {:?}", instructions, index); break }
        }
    }
    index
}


fn connect_amps(input_amp: &mut Data, output_amp: &mut Data) -> () {
    let (tx, rx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    output_amp.output_channel = Some(tx);
    input_amp.r#in = Some(rx);
}
}

fn first_answer(intcode: &Vec<i64>) -> (){
      let mut results_1: Vec<OutputSignal> = Vec::new();
      //try all permutations
          let mut data = Data::new(intcode.clone(), vec![1]);
          data.input_index = 0;
          Data::run_data(&mut data);
      println!("First answer [phase, value]{:?}", data.output);
  }


fn main() {
    //Read input and split by lines
    let file_input = read_input("../9.2.txt");
    //Get the right data from the input
    let input: &str = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => ""
    };
    let intcode: Vec<i64> = input.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    first_answer(&intcode)
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


