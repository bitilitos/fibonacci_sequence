use bigdecimal::BigDecimal;
use std::io;
use std::ops::Add;
use std::time::Instant;

fn main() {
    println!("\n*****************************************");
    println!("** FIBONACCI SEQUENCE VALUE CALCULATOR **");
    println!("*****************************************\n");

    'main: loop {
        let index: u32 = 'input: loop {
            let input = get_input();
            if is_quit(input.trim()) {
                break 'main;
            }
            let index = get_index(input.as_str());
            if let Some(i) = index {
                break 'input i;
            } else {
                println!("Invalid input!!!");
                continue 'input;
            }
        };
        let start = Instant::now();
        println!(
            "The {}{} Fibonacci element is {}.",
            index,
            ordinal_string(index),
            format_number(return_fib_element(index))
        );
        let duration = start.elapsed();
        println!(
            "The calculation algorithm of the {}{} Fibonacci element took {:?}.\n",
            index,
            ordinal_string(index),
            duration
        );
    }
}

fn get_input() -> String {
    println!("Please enter a number from 0 up to 4294967295, or q to quit.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could't read input!");
    input
}

fn get_index(input: &str) -> Option<u32> {
    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn return_fib_element(index: u32) -> BigDecimal {
    match index {
        0 => BigDecimal::from(0),
        1 => BigDecimal::from(1),
        _ => calculate_fib_element(index),
    }
}

fn calculate_fib_element(index: u32) -> BigDecimal {
    let mut last_fib_numbers: [BigDecimal; 2] = [BigDecimal::from(0), BigDecimal::from(1)];
    for i in 2..index {
        if i % 2 == 0 {
            last_fib_numbers[0] = last_fib_numbers[0].clone().add(&last_fib_numbers[1]);
        } else {
            last_fib_numbers[1] = last_fib_numbers[0].clone().add(&last_fib_numbers[1]);
        }
    }
    last_fib_numbers[0].clone().add(&last_fib_numbers[1])
}

fn ordinal_string(index: u32) -> String {
    match index % 10 {
        1 => String::from("st"),
        2 => String::from("nd"),
        3 => String::from("rd"),
        _ => String::from("th"),
    }
}

fn format_number(number: BigDecimal) -> String {
    let mut str_number: String = number.to_string();
    if str_number.len() > 3 {
        for i in (1..=str_number.len() - 3).rev().step_by(3) {
            str_number.insert(i, ' ');
        }
    }
    str_number
}

fn is_quit(input: &str) -> bool {
    match input {
        "q" => true,
        "Q" => true,
        "quit" => true,
        "QUIT" => true,
        "Quit" => true,
        _ => false,
    }
}
