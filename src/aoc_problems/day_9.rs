use std::collections::VecDeque;


pub fn q1(player_num: u32, last_marble_pt: u32) -> u32 {
    // positive is clockwise!!
    let mut marble_circle: VecDeque<u32> = VecDeque::new();
    marble_circle.push_back(0);
    marble_circle.push_back(1);

    let mut elf_scores: Vec<u32> = vec![0; player_num as usize];

    let mut current_elf_num: u32 = 1;
    let mut current_marble_idx: usize = 0;
    let mut current_marble_num: u32 = 2;
    while current_marble_num <= last_marble_pt {
        match current_marble_num % 23 {
            0 => {
                let score = elf_scores.get_mut(current_elf_num as usize).unwrap();
                *score += current_marble_num;

                // if cfg!(debug_assertions) {
                //     println!("Elf {} gets points from ball {}", current_elf_num, current_marble_num);
                // }

                let modulo = marble_circle.len();
                let mut new_marble_idx = (current_marble_idx + modulo - 7) % modulo;
                let removed_marble: u32 = marble_circle.remove(new_marble_idx).unwrap();
                // if cfg!(debug_assertions) {
                //     println!(
                //         "Elf {} gets points from ball {} which was removed",
                //         current_elf_num, removed_marble
                //     );
                // }
                *score += removed_marble;

                current_marble_idx = new_marble_idx % (modulo - 1);
            },
            _ => {
                let modulo = marble_circle.len();
                let new_marble_idx = (current_marble_idx + 2 + modulo) % modulo;
                marble_circle.insert(new_marble_idx, current_marble_num);
                current_marble_idx = new_marble_idx;
            }
        }
        current_marble_num += 1;
        current_elf_num = (current_elf_num + 1) % player_num;
        // if cfg!(debug_assertions) {
        //     println!("Current: {} = {:?}", current_marble_num, marble_circle);
        // }
    }

    // if cfg!(debug_assertions) {
    //     println!("{:?}", elf_scores);
    // }

    *elf_scores.iter().max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(q1(9, 25), 32);
        assert_eq!(q1(10, 1618), 8317);
    }
}
