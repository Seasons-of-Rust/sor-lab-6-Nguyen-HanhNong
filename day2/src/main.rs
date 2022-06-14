use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut part_one_numbers: Vec<i32> =
        BufReader::new(File::open("day2/src/input.txt").expect("file not found"))
            .lines() // Go through each line
            .next() // Only take the first line
            .unwrap() // Unwrap the option of whether there was a next line
            .unwrap() // Unwrap the string result of the lines
            .split(',') // Split by commas
            .map(|number| {
                number
                    .parse() // Parse the number
                    .expect("could not parse number") // Panic with a message if it fails
            })
            .collect();

    program_alarm_part_one(&mut part_one_numbers, 0, 12, 2);
    program_alarm_part_two();
}

fn program_alarm_part_one(list_of_numbers: &mut [i32], mode: u32, noun: i32, verb: i32) -> i32 {
    list_of_numbers[1] = noun;
    list_of_numbers[2] = verb;

    let mut starting_position = 0;

    loop {
        let first_read_position: i32 =
            list_of_numbers[list_of_numbers[starting_position + 1] as usize];
        let second_read_position: i32 =
            list_of_numbers[list_of_numbers[starting_position + 2] as usize];
        let storing_position: i32 = list_of_numbers[starting_position + 3];

        match list_of_numbers[starting_position] {
            1 => {
                list_of_numbers[storing_position as usize] =
                    first_read_position + second_read_position
            }
            2 => {
                list_of_numbers[storing_position as usize] =
                    first_read_position * second_read_position
            }
            99 => break,
            _ => panic!("Unknown number received!"),
        }

        starting_position += 4;
    }

    if mode == 0 {
        println!(
            "The value that is left at position 0 after the program halts is {}",
            list_of_numbers[0]
        );
    }
    list_of_numbers[0]
}

fn program_alarm_part_two() {
    for i in 0..100 {
        for j in 0..100 {
            let mut part_two_numbers: Vec<i32> =
                BufReader::new(File::open("day2/src/input.txt").expect("file not found"))
                    .lines() // Go through each line
                    .next() // Only take the first line
                    .unwrap() // Unwrap the option of whether there was a next line
                    .unwrap() // Unwrap the string result of the lines
                    .split(',') // Split by commas
                    .map(|number| {
                        number
                            .parse() // Parse the number
                            .expect("could not parse number") // Panic with a message if it fails
                    })
                    .collect();

            let result: i32 = program_alarm_part_one(&mut part_two_numbers, 1, i, j);

            if result == 19690720 {
                println!("These two numbers, {}, {}, result in the desired number, 19690720, when multiplied together.", i, j);
                println!("The answer you need to submit on the website is 9820!");
            }
        }
    }
}
