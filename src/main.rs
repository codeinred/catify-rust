use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn read_all(file: &mut File) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Strip any existing spaces, newlines, or emoji endings.
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

// Choose two distinct emojis and append them to the strings. The emojis should
// be preceeded by a space and followed by a newline.
fn emote(str: &mut String) {
    let mut rng = rand::thread_rng();
    let endings = ["🐈", "😌", "💅", "💕", "💖"];
    let i1 = rng.gen_range(0..endings.len());
    let i2 = rng.gen_range(1..endings.len());
    // if i1 == i2, then we shift i2 over. This ensures that i1 != i2.
    let i2 = if i1 == i2 { i2 - 1 } else { i2 };

    str.push(' ');
    str.push_str(endings[i1]);
    str.push_str(endings[i2]);
    str.push('\n');
}

fn catify(filename: &String) -> Result<(), std::io::Error> {
    let mut file = File::open(filename)?;

    let mut found_title_line = false;
    let mut result = String::new();

    for line in read_all(&mut file)?.split_inclusive('\n') {
        // Lines that are empty, or that start with a '#', should be ignored
        if !found_title_line && line.len() > 1 && !line.starts_with('#') {
            found_title_line = true;
            result.push_str(sanitize(line));
            emote(&mut result);
        } else {
            result.push_str(line);
        }
    }

    std::fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(filename)?
        .write_all(result.as_bytes())?;

    Ok(())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    match catify(&filename) {
        Ok(_) => (),
        Err(err) => {
            println!(
                "Catify encountered error when processing '{}': {}",
                filename, err
            );
            std::process::exit(1);
        }
    }
}
