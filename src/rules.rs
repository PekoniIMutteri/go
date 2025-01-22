use super::{Coords, Stone};

pub fn valid_move<const N: usize>(coords: &Coords, board: &[[Stone; N]; N]) -> bool {
    board[coords.y][coords.x] == Stone::Empty
}
