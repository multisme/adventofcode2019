use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    count: u32,
    before: Vec<&'a str>,
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

fn _print_graph(graph: &HashMap<&str, Node>) {
    for (name, node) in graph {
        println!("{:?}, {:?}, {:?}", name, node.name, node.count);
    }
}

fn compute_path_to_center<'a>(star_chart: &'a HashMap<&str, Node>, name: &str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut end = false;
    let mut current_node = star_chart.get(name).unwrap();

    while end == false {
        match star_chart.get(current_node.name) {
            Some(previous) => {
                path.push(current_node.name);
                current_node = previous;
            }
            _ => end = true,
        }
    }
    return path;
}

fn count_orbits(star_chart: &mut HashMap<&str, Node>, name: &str) -> u32 {
    match star_chart.get(name) {
        Some(previous) => {
            let name = previous.name;
            count_orbits(star_chart, name) + 1
        }
        _ => 0,
    }
}

fn compute_chart_orbits(star_chart: &mut HashMap<&str, Node>) -> u32 {
    let mut final_count: u32 = 0;
    for (name, node) in &mut star_chart.clone().to_owned() {
        if node.count == 0 {
            let count = count_orbits(star_chart, name);
            node.count = count;
            final_count += count;
        }
    }
    return final_count;
}

fn compute_path_spaceship(star_chart: &mut HashMap<&str, Node>) -> usize {
    let start = compute_path_to_center(star_chart, "YOU");
    let end = compute_path_to_center(star_chart, "SAN");
    for (index, object) in start.iter().enumerate() {
        for (index2, others) in end.iter().enumerate() {
            if others == object {
                return (index) + (index2);
            }
        }
    }
    0
}

fn get_data(input: std::str::SplitWhitespace) -> HashMap<&str, Node> {
    let mut graph: HashMap<&str, Node> = HashMap::new();
    let mut direct_orbit: Vec<&str>;
    for current in input {
        direct_orbit = current.split(")").collect();
        let object = direct_orbit[0];
        let satellite = direct_orbit[1];
        graph.insert(
            satellite,
            Node {
                name: object,
                count: 0,
                before: Vec::new(),
            },
        );
    }
    graph
}

fn main() {
    //Read input and split by lines

    let file_input = read_input("../6.txt");
    //Get the lines from the input
    let input = file_input.split_whitespace();
    //Parse that data to appropriate DataFormat
    let mut graph = get_data(input);
    //Compute number of Orbits
    let final_count = compute_chart_orbits(&mut graph);
    //print_graph(&graph);
    let path_size = compute_path_spaceship(&mut graph);
    println!("{:?} {:?}", final_count, path_size);
    // println!(" The input is: {:?}", input);
}
