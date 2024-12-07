use std::{fs, io::Read, path::Path};

use anyhow::{bail, Result};

pub fn solution_part1(input_file: &Path) -> Result<u32> {
    if !input_file.exists() || !input_file.is_file() {
        bail!("Invalid file '{}'!", input_file.to_string_lossy());
    }

    let mut file = fs::File::open(input_file).expect("Failed to read file. Probably invalid UTF-8");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Read all characters into a matrix
    let mut chars: Vec<Vec<char>> = Vec::new();
    let mut temp: Vec<char> = Vec::new();

    bytes.iter().for_each(|b| {
        // Skip use \r and \n as to signal a new line
        if *b == 0xD || *b == 0xA {
            if temp.len() > 0 {
                chars.push(temp.clone());
                temp.clear();
            }
        } else {
            let c = char::from_u32(*b as u32).unwrap();
            temp.push(c);
        }
    });
    if temp.len() > 0 {
        chars.push(temp.clone());
        temp.clear();
    }

    let mut finds = 0;

    let rows = chars.len();
    let cols = chars[0].len();

    let word_len = 4;
    let xmas = ['X', 'M', 'A', 'S'];
    let samx = ['S', 'A', 'M', 'X'];

    /*
        0 1 2 3 4 5 6
      0 X M A S M S S
      1 M M A X A X A
      2 M S M A S S X
      3 X X A M M X A
      4 X A S M M A S
      5 X A S M A S M
      6 X A S M X A M
    */

    // Find all horizontal words
    for row in &chars {
        for i in 3..cols {
            let word = &row[(i - 3)..=i];
            if word == xmas || word == samx {
                finds += 1;
            }
        }
    }

    // Find all vertical words
    for j in 0..cols {
        for i in 3..rows {
            let word = &chars[(i - 3)..=i]
                .iter()
                .map(|row| row[j])
                .collect::<Vec<char>>();
            if *word == xmas || *word == samx {
                finds += 1;
            }
        }
    }

    // Find all diagonal top-left -> bottom-right words
    for start_row in 0..=(rows - word_len) {
        for j in 0..=(cols - word_len - start_row) {
            let word = [
                chars[start_row + j][j],
                chars[start_row + j + 1][j + 1],
                chars[start_row + j + 2][j + 2],
                chars[start_row + j + 3][j + 3],
            ];
            if word == xmas || word == samx {
                finds += 1;
            }
        }
    }

    for start_col in 1..=(cols - word_len) {
        for i in 0..=(rows - word_len - start_col) {
            let word = [
                chars[i][start_col + i],
                chars[i + 1][start_col + i + 1],
                chars[i + 2][start_col + i + 2],
                chars[i + 3][start_col + i + 3],
            ];
            if word == xmas || word == samx {
                finds += 1;
            }
        }
    }

    // Find all diagonal top-right -> bottom-left words
    for start_row in 0..=(rows - word_len) {
        for j in ((word_len - 1 + start_row)..cols).rev() {
            let col_offset = cols - j - 1;
            let word = [
                chars[start_row + col_offset][j],
                chars[start_row + col_offset + 1][j - 1],
                chars[start_row + col_offset + 2][j - 2],
                chars[start_row + col_offset + 3][j - 3],
            ];
            if word == xmas || word == samx {
                finds += 1;
            }
        }
    }

    for start_col in ((word_len - 1)..(cols - 1)).rev() {
        let col_offset = cols - start_col - 1;
        for i in 0..=(rows - word_len - col_offset) {
            let word = [
                chars[i][start_col - i],
                chars[i + 1][start_col - i - 1],
                chars[i + 2][start_col - i - 2],
                chars[i + 3][start_col - i - 3],
            ];
            if word == xmas || word == samx {
                finds += 1;
            }
        }
    }

    return Ok(finds);
}

pub fn solution_part2(input_file: &Path) -> Result<u32> {
    if !input_file.exists() || !input_file.is_file() {
        bail!("Invalid file '{}'!", input_file.to_string_lossy());
    }

    let mut file = fs::File::open(input_file).expect("Failed to read file. Probably invalid UTF-8");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Read all characters into a matrix
    let mut chars: Vec<Vec<char>> = Vec::new();
    let mut temp: Vec<char> = Vec::new();

    bytes.iter().for_each(|b| {
        // Skip use \r and \n as to signal a new line
        if *b == 0xD || *b == 0xA {
            if temp.len() > 0 {
                chars.push(temp.clone());
                temp.clear();
            }
        } else {
            let c = char::from_u32(*b as u32).unwrap();
            temp.push(c);
        }
    });
    if temp.len() > 0 {
        chars.push(temp.clone());
        temp.clear();
    }

    let mut finds = 0;

    let rows = chars.len();
    let cols = chars[0].len();

    // Center must be A
    // Corners need to be M or S, in diagonal pairs
    // M|S . M|S
    //  .  A  .
    // S|M . S|M

    /*
        0 1 2 3 4 5 6
      0 X M A S M S S
      1 M M A X A X A
      2 M S M A S S X
      3 X X A M M X A
      4 X A S M M A S
      5 X A S M A S M
      6 X A S M X A M
    */

    // Scan matrix 3 rows and columns at a time
    for i in 0..=(rows - 3) {
        for j in 0..=(cols - 3) {
            let center = &chars[i + 1][j + 1];
            if *center != 'A' {
                continue;
            }

            let top_left = &chars[i][j];
            let bottom_right = &chars[i + 2][j + 2];
            if *top_left == *bottom_right
                || (*top_left != 'M' && *top_left != 'S')
                || (*bottom_right != 'M' && *bottom_right != 'S')
            {
                continue;
            }

            let top_right = &chars[i][j + 2];
            let bottom_left = &chars[i + 2][j];
            if *top_right == *bottom_left
                || (*top_right != 'M' && *top_right != 'S')
                || (*bottom_left != 'M' && *bottom_left != 'S')
            {
                continue;
            }

            finds += 1;
        }
    }

    return Ok(finds);
}
