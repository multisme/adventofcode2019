use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::from_utf8;
use std::iter::{self, Sum};

#[derive(Debug)]
#[derive(Clone)]
struct Layer{
    pixels: String
}

#[derive(Debug)]
struct Image{
    height: usize,
    width: usize,
    layers: Vec<Layer>
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

impl Layer{
    fn count_pixels(&self, pixel: char) -> usize{
        self.pixels.chars()
            .filter(|&p| p == pixel)
            .count()
    }

    fn add_layer(&self, underneath: &Layer) -> Layer {
        let new_pixels: String = self.pixels.chars().zip(underneath.pixels.chars())
            .map(|(a, b)| if a == '2' {b} else {a})
            .collect();
            Layer{pixels: new_pixels}
    }
}

impl Image{
    fn new(width: usize, height: usize, data: &str) -> Image {
        let pixels_number = width * height;
        let layers = data.as_bytes()
            .chunks(pixels_number)
            .map(|chunk| Layer { pixels: from_utf8(chunk).unwrap().to_string()})
            .collect::<Vec<Layer>>();
        Image {height: height, width: width, layers: layers}
    } 

    fn print_layer(&self, layer: &Layer) -> (){
        layer.pixels.as_bytes()
            .chunks(self.width)
            .for_each(|chunk| println!("{:?}", from_utf8(chunk).unwrap()
                                                    .to_string()
                                                    .replace("0", " "))
            );
    }
}


fn first_answer(image: &Image) -> usize {
    let mut layers = image.layers.clone();
    layers.sort_by_key(|layer| layer.count_pixels('0'));
    return layers[0].count_pixels('1') * layers[0].count_pixels('2')
}


fn second_answer(image: &Image) -> Layer {
    image.layers.iter().fold(image.layers[0].clone(), |a, b| a.add_layer(&b))
}

fn main() {
    let file_input = read_input("../8.txt");

    let input: &str = match file_input.split_whitespace().next() {
        Some(s) => s,
        None => ""
    };
    let image = Image::new(25, 6, input);
    let result = first_answer(&image);
    let result2 = second_answer(&image);
    println!("{:?}", result);
    image.print_layer(&result2);
}
