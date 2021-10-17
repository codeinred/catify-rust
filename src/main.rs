use std::io;

fn fib(n: u64, first: u64, second: u64) -> u64 {
    if n == 0 {
        first
    } else {
        fib(n - 1, second, first + second)
    }
}

fn main() {

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let n = guess.trim().parse::<u64>().expect("Unable to parse number");
    println!("fib({}) = {}", n, fib(n, 1, 1));
}
