use std::fs::File;
use std::io::prelude::*;

fn read_all(file : &mut File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read contents of file");
    contents
}
fn catify(file : &String) {
    let mut file = File::open(file).expect("Unable to open the file");
    let contents = read_all(&mut file);
    println!("Contents of file: {}", contents);

}
fn main() {
    let args : Vec<String> = std::env::args().collect();
   
    if args.len() == 1 {
        println!("Usage: {} <filename>", args[0]);
        return 
    }

    let filename = &args[1];
    
    catify(&filename);   
}
