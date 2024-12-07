use std::{collections::HashMap, io::Read, path::Path};

use anyhow::Result;

use crate::common::read_file;

pub fn solution(ordering_path: &Path, update_path: &Path) -> Result<(u32, u32)> {
    let mut ord_file = read_file(ordering_path)?;
    let mut upd_file = read_file(update_path)?;

    let mut ordering = String::new();
    ord_file.read_to_string(&mut ordering)?;

    // Keys are page numbers. Values are all pages that must come after that page number
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in ordering.lines() {
        let pages: Vec<&str> = rule.split("|").collect();
        // Assumes pages is of length 2
        let before: u32 = pages[0].parse()?;
        let after: u32 = pages[1].parse()?;

        rules
            .entry(before)
            .and_modify(|vec| vec.push(after))
            .or_insert(vec![after]);
    }

    let mut updates = String::new();
    upd_file.read_to_string(&mut updates)?;

    let mut valid_middle_page_total = 0;
    let mut invalid_middle_page_total = 0;
    let mut invalid_updates: Vec<(Vec<u32>, usize, usize)> = Vec::new();

    for update in updates.lines() {
        let pages: Vec<u32> = update
            .split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let (is_valid, maybe_cause) = is_update_valid(&pages, &rules);

        // If the update is valid, find the middle page and add it to the total
        if is_valid {
            let middle_index = (pages.len() - 1) / 2;
            valid_middle_page_total += pages[middle_index];
        } else {
            let (before, after) = maybe_cause.unwrap();
            invalid_updates.push((pages, before, after));
        }
    }

    // Fix invalid updates
    for (mut pages, mut before, mut after) in invalid_updates {
        // For each invalid update, try to fix it by swapping wrong pages with the ones they were supposed to be after
        // until it is no longer wrong. Basically a poor man's sorting algorithm
        loop {
            pages.swap(before, after);

            let (is_valid, maybe_cause) = is_update_valid(&pages, &rules);
            if !is_valid {
                // If update still isn't valid, repeat with the new wrong pair of pages
                (before, after) = maybe_cause.unwrap();
            } else {
                // If it's valid, update the total and move on the next one
                let middle_index = (pages.len() - 1) / 2;
                invalid_middle_page_total += pages[middle_index];
                break;
            }
        }
    }

    return Ok((valid_middle_page_total, invalid_middle_page_total));
}

fn is_update_valid(
    pages: &Vec<u32>,
    rules: &HashMap<u32, Vec<u32>>,
) -> (bool, Option<(usize, usize)>) {
    let mut is_valid = true;
    let mut before: usize = 0;
    let mut after: usize = 0;

    for (i, curr_page) in pages.iter().enumerate() {
        // The hashmap contains what pages need to come after
        // So we check if any page in a value comes before
        // If that's the case, we invalidate the string
        let maybe_rules = rules.get(curr_page);
        if let None = maybe_rules {
            continue;
        }
        let must_be_after = maybe_rules.unwrap();

        // Check if any of the preceding pages must be after by the rules
        for j in 0..i {
            let a_preceding_page = &pages[j];
            if must_be_after.contains(a_preceding_page) {
                is_valid = false; // Invalidate the update
                before = j;
                after = i;
                break;
            }
        }

        // No need to check further pages if the update is already invalid
        if !is_valid {
            break;
        }
    }

    if is_valid {
        return (is_valid, None);
    } else {
        return (is_valid, Some((before, after)));
    }
}
