use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}


#[derive(Debug, Clone, Copy)]
struct Elf {
    idx: usize,
    recipe: u8
}

impl Elf {
    fn new(idx: usize, recipe: u8) -> Elf {
        Elf {
            idx,
            recipe
        }
    }
}

pub fn q1(min_recipes: usize) -> String {
    let mut recipe_scores: Vec<u8> = vec![3, 7];
    let mut elves: Vec<Elf> = vec![Elf::new(0, 3), Elf::new(1, 7)];

    loop {
        if recipe_scores.len() >= 10 + min_recipes {
            break;
        }

        let new_recipe_score: u8 = elves.iter().map(|elf| recipe_scores[elf.idx]).sum();

        let score_digits: Vec<u8> = new_recipe_score.to_string().chars().map(|d| d.to_digit(10).unwrap() as u8).collect();
        recipe_scores.extend(&score_digits);

        for elf in elves.iter_mut() {
            let new_idx: usize = (elf.idx + 1 + (recipe_scores[elf.idx] as usize)) % recipe_scores.len();
            elf.idx = new_idx;
        }
    }

    let x: Vec<String> = recipe_scores[min_recipes..min_recipes+10].iter().map(|digit| digit.to_string()).collect();
    x.join("")
}

fn sublist_contains(l: &[u8], sublist: &[u8]) -> Option<usize> {
    if l.len() < sublist.len() {
        return None;
    }

    for (idx, subslice) in l.windows(sublist.len()).enumerate() {
        if sublist == subslice {
            return Some(idx);
        }
    }

    None
}

pub fn q2(substring: String) -> usize {
    let sublist: Vec<u8> = substring.chars().map(|d| d.to_digit(10).unwrap() as u8).collect();
    let mut recipe_scores: Vec<u8> = vec![3, 7];
    let mut elves: Vec<Elf> = vec![Elf::new(0, 3), Elf::new(1, 7)];

    loop {
        let starting_check_idx = match recipe_scores.len() {
            n if n  < 10 => 0,
            _ => recipe_scores.len() - 10
        };
        if let Some(start_idx) = sublist_contains(&recipe_scores[starting_check_idx..], &sublist) {
            return starting_check_idx + start_idx;
        }

        let new_recipe_score: u8 = elves.iter().map(|elf| recipe_scores[elf.idx]).sum();

        let score_digits: Vec<u8> = new_recipe_score.to_string().chars().map(|d| d.to_digit(10).unwrap() as u8).collect();
        recipe_scores.extend(&score_digits);

        for elf in elves.iter_mut() {
            let new_idx: usize = (elf.idx + 1 + (recipe_scores[elf.idx] as usize)) % recipe_scores.len();
            elf.idx = new_idx;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_tests() {
        assert_eq!(q1(9), "5158916779");
        assert_eq!(q1(5), "0124515891");
        assert_eq!(q1(18), "9251071085");
        assert_eq!(q1(2018), "5941429882");
    }

    #[test]
    fn q2_tests() {
        assert_eq!(q2("51589".to_string()), 9);
        assert_eq!(q2("01245".to_string()), 5);
        assert_eq!(q2("92510".to_string()), 18);
        assert_eq!(q2("59414".to_string()), 2018);
    }
}
