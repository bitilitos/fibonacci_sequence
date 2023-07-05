use std::ops::Add;
use std::io;

fn main() {

    println!("Enter a number for the index of the fibonacci sequence.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could't read input!");

    let index:u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(num) => 255,
    };


}

fn return_fib_element(index:u8) -> u128 {
    match index {
        0 => 0,
        1 => 1,
        _ => calculate_fib_element(index)
    }
}

fn calculate_fib_element(index:u8) -> u128 {
    let mut last_fib_numbers:[u128;2] = [0, 1];
    for i in 2..index {
        if i%2==0 {
            last_fib_numbers[0] = last_fib_numbers[0].add(last_fib_numbers[1]);
        } else {
            last_fib_numbers[1] = last_fib_numbers[0].add(last_fib_numbers[1]);
        }
    }
    last_fib_numbers[0].add(last_fib_numbers[1])
}
