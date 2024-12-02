use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use anyhow::{bail, Result};

fn get_report_list(input_file: &Path) -> Result<Vec<Vec<u32>>> {
    if !input_file.exists() || !input_file.is_file() {
        bail!("Invalid file '{}'!", input_file.to_string_lossy());
    }

    let mut list: Vec<Vec<u32>> = vec![];

    let file = fs::File::open(input_file).expect("Failed to read file. Probably invalid UTF-8");
    let buf_reader = io::BufReader::new(file);
    for line in buf_reader.lines() {
        let line = line?;
        let mut levels: Vec<u32> = vec![];
        for level in line.split_whitespace() {
            levels.push(level.parse::<u32>()?);
        }

        list.push(levels);
    }

    return Ok(list);
}

fn are_levels_safe(levels: &Vec<u32>) -> bool {
    let mut last_level = levels[0];
    let mut increasing = true;
    for (idx, level) in levels.iter().enumerate() {
        // Both conditions are about checking differences so first index can be skipped
        if idx == 0 {
            continue;
        }

        // This can panic if either level is >2^31, but all levels are <100 so it's safe
        let diff = (*level as i32) - (last_level as i32);
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

        if idx == 1 {
            // Increasing/decreasing condition can only be checked from the third number
            // The second number determines what we're looking for
            // Negative difference means it decreased
            if diff < 0 {
                increasing = false;
            }
        } else if idx > 1 {
            // Check if monotonicity if respected
            if increasing && diff < 0 {
                return false;
            } else if !increasing && diff > 0 {
                return false;
            }
        }

        last_level = *level;
    }

    return true;
}

pub fn solution_part1(input_file: &Path) -> Result<u32> {
    let list = get_report_list(input_file)?;
    let mut safe_reports = 0;

    for levels in list {
        if are_levels_safe(&levels) {
            safe_reports += 1;
        }
    }

    return Ok(safe_reports);
}

fn are_dampened_levels_safe(levels: &Vec<u32>) -> bool {
    if are_levels_safe(levels) {
        return true;
    }

    // If levels aren't safe, try removing each number in order and see if any combination works
    // Inefficient, but works. Surely there's a smarter algorithm, but I couldn't find one in an
    // hour or so and didn't feel like spending more time on this
    for (idx, _) in levels.iter().enumerate() {
        let mut dampened_levels = levels.clone();
        dampened_levels.remove(idx);
        if are_levels_safe(&dampened_levels) {
            return true;
        }
    }

    return false;
}

pub fn solution_part2(input_file: &Path) -> Result<u32> {
    let list = get_report_list(input_file)?;
    let mut safe_reports = 0;

    for levels in list {
        if are_dampened_levels_safe(&levels) {
            safe_reports += 1;
        }
    }

    return Ok(safe_reports);
}
