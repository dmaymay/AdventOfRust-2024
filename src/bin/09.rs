use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let chars: Vec<char> = input.chars().collect();
    let mut id: u64 = 0;
    let mut encoding: Vec<String> = Vec::new();

    // Build the initial encoding
    for pair in chars.chunks(2) {
        if pair.len() == 2 {
            for _ in 0..(pair[0].to_digit(10)?) {
                encoding.push(id.to_string());
            }
            for _ in 0..(pair[1].to_digit(10)?) {
                encoding.push(".".to_string());
            }
        } else {
            for _ in 0..(pair[0].to_digit(10)?) {
                encoding.push(id.to_string());
            }
        }
        id += 1;
    }

    // Get free space positions and digit positions
    let mut free_positions = VecDeque::new();
    let mut digit_positions = Vec::new();

    for (i, block) in encoding.iter().enumerate() {
        if block == "." {
            free_positions.push_back(i);
        } else {
            digit_positions.push(i);
        }
    }

    // Compacting the file system
    while let (Some(&gap_idx), Some(&digit_idx)) = (free_positions.front(), digit_positions.last())
    {
        if gap_idx < digit_idx {
            let digit_block = encoding[digit_idx].clone();
            encoding[digit_idx] = ".".to_string();
            encoding[gap_idx] = digit_block;

            free_positions.pop_front();
            digit_positions.pop();
        } else {
            break;
        }
    }

    // Remove trailing periods
    while encoding.last().map(|s| s.as_str()) == Some(".") {
        encoding.pop();
    }

    // Compute the checksum
    let checksum: u64 = encoding
        .iter()
        .enumerate()
        .filter_map(|(pos, ch)| {
            if let Ok(val) = ch.parse::<u64>() {
                Some(pos as u64 * val)
            } else {
                None
            }
        })
        .sum();

    Some(checksum)
}

#[derive(Debug, Clone)]
struct FileBlock {
    file_ids: Vec<u64>,
    free_space: u64,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut encoding: Vec<FileBlock> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut id_ord: u64 = 0;

    // intial encoding
    for pair in chars.chunks(2) {
        if pair.len() == 2 {
            let file_length = pair[0].to_digit(10).unwrap_or(0) as usize;
            let mut file_ids = Vec::new();
            for _ in 0..file_length {
                file_ids.push(id_ord);
            }

            let free_space = pair[1].to_digit(10).unwrap_or(0) as u64;
            encoding.push(FileBlock {
                file_ids,
                free_space,
            });
        } else {
            let file_length = pair[0].to_digit(10).unwrap_or(0) as usize;
            let mut file_ids = Vec::new();
            for _ in 0..file_length {
                file_ids.push(id_ord);
            }
            encoding.push(FileBlock {
                file_ids,
                free_space: 0,
            });
        }
        id_ord += 1;
    }

    /*     // merge empty blocks (did nothing but made sense at the time)
    let mut i = 0;
    while i + 1 < encoding.len() {
        // If block (i+1) is fully empty, merge it into block i
        if encoding[i + 1].file_ids.is_empty() {
            encoding[i].free_space += encoding[i + 1].free_space;
            encoding.remove(i + 1);
        } else {
            i += 1;
        }
    } */

    // compress files
    let duplicate = encoding.clone();
    for (index, file_block) in duplicate.iter().rev().enumerate() {
        let rev_idx = duplicate.len() - 1 - index;
        
        // original chunk are all the same ID repeated.
        // block_size = how many blocks that file occupies.
        let block_size = file_block.file_ids.len() as u64;

        // look for free_space on left
        for left_idx in 0..rev_idx {
            if encoding[left_idx].free_space >= block_size {
                // move the entire set of idss from the old block to the new
                let the_id = file_block.file_ids[0];
                for _ in 0..block_size {
                    encoding[left_idx].file_ids.push(the_id);
                }
                // reduce left block free_space
                encoding[left_idx].free_space -= block_size;

                // remove from old location
                encoding[rev_idx].file_ids.drain(0..block_size as usize);

                if rev_idx > 0 {
                    // push free space to the left block
                    encoding[rev_idx - 1].free_space += block_size;
                } else {
                    // unless first block
                    encoding[rev_idx].free_space += block_size;
                }
                // optionally re-merge after the move
                let mut j = 0;
                while j + 1 < encoding.len() {
                    if encoding[j + 1].file_ids.is_empty() {
                        encoding[j].free_space += encoding[j + 1].free_space;
                        encoding.remove(j + 1);
                    } else {
                        j += 1;
                    }
                }

                break;
            }
        }
    }

    // checksum
    let mut pos = 0;
    let mut sum = 0u64;

    for block in &encoding {
        for &id in &block.file_ids {
            sum += pos as u64 * id;
            pos += 1;
        }
        for _ in 0..block.free_space {
            pos += 1;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
