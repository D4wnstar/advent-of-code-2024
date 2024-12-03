use std::{
    io::{self, Write},
    path::Path,
};

mod day_1;
mod day_2;
mod day_3;

fn main() {
    let available_puzzles = ["1", "2", "3"];
    println!("Hello, welcome to Advent of Code 2024!");
    println!("Please select the puzzle you want to run.");
    println!("{:?}", available_puzzles);
    print!("> ");
    io::stdout().flush().expect("Can't flush stdout");

    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read user input");

    let input = input_str.trim();

    match input {
        "1" => {
            let path = Path::new("./src/day_1/input.txt");
            // Solution to part 1 is 1580061
            let distance = day_1::solution_part1(path).unwrap();
            println!("Distance: {distance}");

            // Solution to part 2 is 23046913
            let score = day_1::solution_part2(path).unwrap();
            println!("Similarity score: {score}");
        }
        "2" => {
            let path = Path::new("./src/day_2/input.txt");
            // Solution to part 1 is 299
            let reports = day_2::solution_part1(path).unwrap();
            println!("Number of safe reports: {reports}");

            // Solution to part 2 is 364
            let reports = day_2::solution_part2(path).unwrap();
            println!("Number of safe reports (dampened): {reports}");
        }
        "3" => {
            let path = Path::new("./src/day_3/input.txt");
            // Solution to part 1 is 155955228
            let reports = day_3::solution(path, false).unwrap();
            println!("Number of safe reports: {reports}");

            // Solution to part 2 is 100189366
            let reports = day_3::solution(path, true).unwrap();
            println!("Number of safe reports (with conditionals): {reports}");
        }
        _ => {
            println!("Not solved yet!")
        }
    }
}
