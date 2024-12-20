use std::{
    io::{self, Write},
    path::Path,
};

mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    let available_puzzles = ["1", "2", "3", "4", "5"];
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
        "4" => {
            let path = Path::new("./src/day_4/input.txt");
            // Solution to part 1 is 2447
            let finds = day_4::solution_part1(path).unwrap();
            println!("Number of finds: {finds}");

            // Solution to part 2 is 1868
            let finds = day_4::solution_part2(path).unwrap();
            println!("Number of X-MAS finds: {finds}");
        }
        "5" => {
            let ord_path = Path::new("./src/day_5/ordering.txt");
            let upd_path = Path::new("./src/day_5/updates.txt");
            // Solution to part 1 is 4814 and part 2 is 5448
            let (valid_total, invalid_total) = day_5::solution(ord_path, upd_path).unwrap();
            println!("Total of valid middle page numbers: {valid_total}");
            println!("Total of invalid middle page numbers: {invalid_total}");
        }
        _ => {
            println!("Not solved yet!")
        }
    }
}
