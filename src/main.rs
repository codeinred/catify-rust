use std::fs::File;
use std::io::prelude::*;

fn read_all(file: &mut File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read contents of file");
    contents
}
fn catify(filename: &String) {
    let mut file = File::open(filename).expect("Unable to open the file");
    let contents = read_all(&mut file);

    let lines = contents.split_inclusive('\n');

    let mut found_title_line = false;

    let oper = |acc, line: &str| -> String {
        if !found_title_line && line.len() > 1 && !line.starts_with('#') {
            found_title_line = true;
            let result: String = acc + line;
            result.replace('\n', " 🐈\n")
        } else {
            acc + line
        }
    };
    let result = lines.fold(String::new(), oper);

    std::fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(filename)
        .expect("Able to open file")
        .write_all(result.as_bytes())
        .expect("I just wanted to write to this file :(");
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    catify(&filename);
}
