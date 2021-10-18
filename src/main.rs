use std::fs::File;
use std::io::prelude::*;

fn read_all(file: &mut File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read contents of file");
    contents
}
fn sanitize(line: &str) -> &str {
    let endings = ["\n", " ", "🐈", "😌", "💅", "💕", "💖"];
    let mut line = line;
    loop {
        let line2 = endings
            .iter()
            .fold(line, |line, ending| line.trim_end_matches(ending));
        if line2.len() == line.len() {
            break;
        }
        line = line2
    }
    line
}

fn catify(filename: &String) {
    let mut file = File::open(filename).expect("Unable to open the file");

    let mut found_title_line = false;
    let mut result = String::new();

    for line in read_all(&mut file).split_inclusive('\n') {
        if !found_title_line && line.len() > 1 && !line.starts_with('#') {
            found_title_line = true;
            result.push_str(sanitize(line));
        } else {
            result.push_str(line);
        }
    }

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
