use std::collections::HashMap;

advent_of_code::solution!(21);

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

- Get shortest path for every combination in numeric keypad
- Get shortest path for every combination in directional keypad

<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
v<<A>>^A<A>AvA<^AA>A<vAAA>^A
<A^A>^^AvvvA
029A

029A
<A^A^^>AvvvA
v<<A>>^A<A>A<AAv>A^Av<AAA>^A
v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<<A>>^AAv<A>A^A<A>Av<A<A>>^AAAvA<^A>A



029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A



 */

#[derive(Debug)]
struct NumericKeypad {
    pos: HashMap<char, (usize, usize)>,
    short_path: HashMap<((usize, usize), (usize, usize)), String>,
}

fn direction_to_str(moves: (i32, i32), numeric: bool) -> String {
    let mut move_string = String::new();
    let r_str = if moves.0 > 0 { "v" } else { "^" };
    let c_str = if moves.1 > 0 { ">" } else { "<" };
    let order = if numeric { 1 } else { -1 };

    if moves.0 * order <= 0 {
        move_string.push_str(&r_str.repeat(moves.0.abs() as usize));
        move_string.push_str(&c_str.repeat(moves.1.abs() as usize));
    } else {
        move_string.push_str(&c_str.repeat(moves.1.abs() as usize));
        move_string.push_str(&r_str.repeat(moves.0.abs() as usize));
    }
    move_string
}

impl NumericKeypad {
    fn positions() -> HashMap<char, (usize, usize)> {
        let mut map = HashMap::new();
        map.insert('7', (0, 0));
        map.insert('8', (0, 1));
        map.insert('9', (0, 2));
        map.insert('4', (1, 0));
        map.insert('5', (1, 1));
        map.insert('6', (1, 2));
        map.insert('1', (2, 0));
        map.insert('2', (2, 1));
        map.insert('3', (2, 2));
        map.insert('X', (3, 0));
        map.insert('0', (3, 1));
        map.insert('A', (3, 2));
        map
    }

    fn get_short_map(&self) -> HashMap<((usize, usize), (usize, usize)), String> {
        let mut numerical_map: HashMap<((usize, usize), (usize, usize)), String> = HashMap::new();
        let characters = vec!['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for (char, pos) in &self.pos {
            for chr in characters.iter() {
                if chr != char {
                    let r_diff = self.pos.get(&chr).unwrap().0 as i32 - pos.0 as i32;
                    let c_diff = self.pos.get(&chr).unwrap().1 as i32 - pos.1 as i32;
                    let movement_string = direction_to_str((r_diff, c_diff), true);
                    numerical_map.insert(
                        (*pos, *self.pos.get(&chr).unwrap()),
                        movement_string.clone(),
                    );
                }
            }
        }
        numerical_map
    }

    fn numeric_to_directional(&self, code:String) -> String {
        let mut numeric_string: String = "".to_owned();
        let mut start_char = 'A';
    
        for chr in code.chars() {
            let start_pos = self.pos.get(&start_char).unwrap();
            let end_pos = self.pos.get(&chr).unwrap();
            numeric_string.push_str(
                self
                    .short_path
                    .get(&(*start_pos, *end_pos))
                    .unwrap_or(&String::new()),
            );
            numeric_string.push_str("A");
            start_char = chr;
        }
        /* println!("{}",numeric_string); */
        numeric_string
    }
}

#[derive(Debug)]
struct DirectionalKeypad {
    pos: HashMap<char, (usize, usize)>,
    short_path: HashMap<((usize, usize), (usize, usize)), String>,
}

impl DirectionalKeypad {

    fn positions() -> HashMap<char, (usize, usize)> {
        let mut map = HashMap::new();

        map.insert('X', (0, 0));
        map.insert('^', (0, 1));
        map.insert('A', (0, 2));
        map.insert('<', (1, 0));
        map.insert('v', (1, 1));
        map.insert('>', (1, 2));
        map
    }

    fn get_short_map(&self) -> HashMap<((usize, usize), (usize, usize)), String> {
        let mut directional_map: HashMap<((usize, usize), (usize, usize)), String> = HashMap::new();
        let characters = vec!['A', '<', '>', '^', 'v'];
        for (char, pos) in &self.pos {
            for chr in characters.iter() {
                if chr != char {
                    let r_diff = self.pos.get(&chr).unwrap().0 as i32 - pos.0 as i32;
                    let c_diff = self.pos.get(&chr).unwrap().1 as i32 - pos.1 as i32;
                    let movement_string = direction_to_str((r_diff, c_diff), false);
                    directional_map.insert(
                        (*pos, *self.pos.get(&chr).unwrap()),
                        movement_string.clone(),
                    );
                }
            }
        }
        directional_map
    }

    fn directional_to_directional(&self,input_string: String, recurse: usize) -> String {
        let mut input = input_string;
        for _ in 0..recurse {
            let chars: Vec<char> = input.chars().collect();
            let mut directional_string = String::new();
            let mut current_char = 'A';
    
            for i in 0..chars.len() {
                let next_char = chars[i];
                if current_char == next_char {
                    directional_string.push_str("A");
                    continue;
                }
    
                let start_pos = self.pos.get(&current_char).unwrap();
                let end_pos = self.pos.get(&next_char).unwrap();
    
                if let Some(moves) = self.short_path.get(&(*start_pos, *end_pos)) {
                    directional_string.push_str(moves);
                    directional_string.push_str("A");
                }
                current_char = next_char;
            }
            input = directional_string.clone();
            /* println!("{:?}",directional_string); */
        }
        input
    }

}


pub fn part_one(input: &str) -> Option<u32> {
    let mut numeric_pad = NumericKeypad {
        pos: NumericKeypad::positions(),
        short_path: HashMap::new(),
    };

    let mut directional_pad = DirectionalKeypad {
        pos: DirectionalKeypad::positions(),
        short_path: HashMap::new(),
    };

    numeric_pad.short_path = numeric_pad.get_short_map();
    directional_pad.short_path = directional_pad.get_short_map();

    let mut result: u32 = 0;
    for code in input.lines(){
        /* println!("{}",code); */
        let numeric_string = numeric_pad.numeric_to_directional(code.to_string());
        let directional_string = directional_pad.directional_to_directional(numeric_string, 2);

        let numeric_chunk = code.replace("A", "").parse::<u32>().unwrap_or(0);
        result += directional_string.len() as u32 * numeric_chunk
    }
    
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
