use super::Coords;
use std::io;

pub fn user_input() -> Option<Coords> {
    let mut line = String::new();
    let input_result = io::stdin().read_line(&mut line);
    if input_result.is_err() {
        return None;
    }
    let mut substr_iter = line.trim().split_whitespace();
    let mut next_num = || -> Option<usize> {
        let next = substr_iter.next();
        if let Some(string) = next {
            let parsed = string.parse();
            if let Ok(num) = parsed {
                return Some(num);
            }
        }
        None
    };
    if let Some(num_x) = next_num() {
        if num_x > 0 {
            if let Some(num_y) = next_num() {
                if num_y > 0 {
                    return Some(Coords::new(num_x - 1, num_y - 1));
                }
            }
        }
    }
    None
}
