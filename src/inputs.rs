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
    let mut input: Coords = Coords::new(0, 0);
    if let Some(num) = next_num() {
        input.x = num;
        if let Some(num) = next_num() {
            input.y = num;
            return Some(input);
        }
    }
    None
}
