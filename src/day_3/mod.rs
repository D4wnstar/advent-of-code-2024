use std::{fs, path::Path};

use anyhow::{bail, Result};
use fancy_regex::Regex;

// Using regexs feels a tiny bit like cheating but I didn't feel like making my own parser
pub fn solution(input_file: &Path, conditionals: bool) -> Result<u64> {
    if !input_file.exists() || !input_file.is_file() {
        bail!("Invalid file '{}'!", input_file.to_string_lossy());
    }

    let mut instr = fs::read_to_string(input_file)?.replace("\n", "");
    if conditionals {
        let cond_pattern = Regex::new(r"don't\(\).*?(?=do\(\)|\z)").unwrap();
        instr = cond_pattern.replace_all(&instr, "").to_string();
    }

    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for maybe_caps in pattern.captures_iter(&instr) {
        if let Ok(caps) = maybe_caps {
            let num1 = caps[1].parse::<u64>()?;
            let num2 = caps[2].parse::<u64>()?;
            total += num1 * num2;
        }
    }

    return Ok(total);
}
