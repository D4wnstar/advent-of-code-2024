use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
    iter::zip,
    path::Path,
};

use anyhow::{bail, Result};

fn make_lists(input_file: &Path) -> Result<(Vec<u32>, Vec<u32>)> {
    if !input_file.exists() || !input_file.is_file() {
        bail!("Invalid file '{}'!", input_file.to_string_lossy());
    }

    let mut list_1: Vec<u32> = vec![];
    let mut list_2: Vec<u32> = vec![];

    let file = fs::File::open(input_file).expect("Failed to read file. Probably invalid UTF-8");
    let buf_reader = io::BufReader::new(file);
    for line in buf_reader.lines() {
        let line = line?;
        let ids: Vec<&str> = line.split_whitespace().collect();
        if ids.len() != 2 {
            eprintln!("Found line which does not contain two IDs: '{line}'");
            continue;
        }

        list_1.push(ids[0].parse::<u32>()?);
        list_2.push(ids[1].parse::<u32>()?);
    }

    return Ok((list_1, list_2));
}

pub fn solution_part1(input_file: &Path) -> Result<u32> {
    let (mut list_1, mut list_2) = make_lists(input_file)?;

    list_1.sort_unstable();
    list_2.sort_unstable();

    let total_distance = zip(list_1, list_2).fold(0, |acc, ids| acc + ids.0.abs_diff(ids.1));

    return Ok(total_distance);
}

pub fn solution_part2(input_file: &Path) -> Result<u32> {
    let (list_1, list_2) = make_lists(input_file)?;

    let mut similarity_score = 0;

    // Cache values so we don't have to recalculate them for the same ID
    let mut cache: HashMap<u32, u32> = HashMap::new();

    for id1 in list_1 {
        // If we already counted the number of occurences for a given ID, get it from cache
        // else add it but iterating through the second list
        let count = cache.entry(id1).or_insert_with(|| {
            list_2
                .iter()
                .fold(0, |acc, id2| if id1 == *id2 { acc + 1 } else { acc })
        });

        // Then calculate the similarity score according to the puzzle
        similarity_score += id1 * *count
    }

    return Ok(similarity_score);
}
