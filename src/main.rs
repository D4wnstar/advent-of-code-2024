use std::{
    io::{self, Write},
    path::Path,
};

mod day_1;

fn main() {
    let available_puzzles = ["1b"];
    println!("Hello, welcome to Advent of Code 2024!");
    println!("Please select the puzzle you want to run.");
    println!("{:?}", available_puzzles);
    print!("> ");
    io::stdout().flush().expect("Can't flush stdio");

    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read user input");

    let input = input_str.trim();

    match input {
        "1b" => {
            // Solution to part 1 is 1580061
            let out = day_1::day_1_builtins::solution_part1(Path::new("./src/day_1/input.txt"));
            match out {
                Ok(distance) => println!("Distance: {distance}"),
                Err(e) => eprintln!("{e}"),
            }

            // Solution to part 2 is ???
            let out = day_1::day_1_builtins::solution_part2(Path::new("./src/day_1/input.txt"));
            match out {
                Ok(score) => println!("Similarity score: {score}"),
                Err(e) => eprintln!("{e}"),
            }
        }
        _ => {
            println!("Not solved yet!")
        }
    }
}
