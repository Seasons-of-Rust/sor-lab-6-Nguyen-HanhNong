use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numbers: Vec<i32> =
        BufReader::new(File::open("day1/src/input.txt").expect("file not found"))
            .lines() // Go through each line
            .map(|line| {
                line.expect("could not parse line") // Unwrap the result of the line
                    .parse() // Try to parse it to what we expect (i32 from the annotation)
                    .expect("could not parse number") // Unwrap the result of the parse
            })
            .collect();

    let mut total_naive_fuel: i32 = 0;
    let mut total_real_fuel: i32 = 0;

    for mass_amount in numbers {
        total_naive_fuel += get_naive_fuel_amount(&mass_amount);
        total_real_fuel += get_real_fuel_amount(&mass_amount);
    }

    println!(
        "The naive total amount of fuel needed for all the modules of the spacecraft is {}!",
        total_naive_fuel
    );

    println!(
        "The real total amount of fuel needed for all the modules of the spacecraft is {}!",
        total_real_fuel
    );
}

fn get_naive_fuel_amount(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn get_real_fuel_amount(mass: &i32) -> i32 {
    let mut temp_number: i32 = *mass;
    let mut fuel_counter: i32 = 0;

    loop {
        if get_naive_fuel_amount(&temp_number) <= 0 {
            if temp_number == 0 {
                fuel_counter = temp_number;
            }
            break;
        } else {
            temp_number = get_naive_fuel_amount(&temp_number);
            fuel_counter += temp_number;
        }
    }
    fuel_counter
}
