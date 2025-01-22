mod display;
mod inputs;
mod rules;

const SIZE: usize = 4;

fn main() {
    let mut board = [[Stone::Empty; SIZE]; SIZE];
    let mut playing = true;
    let mut playing_stone = Stone::Black;
    while playing {
        display::draw_board(&board);
        if let Some(coords) = inputs::user_input(SIZE) {
            if rules::valid_move(&coords, &board) {
                coords.change_board(playing_stone, &mut board);
                match playing_stone {
                    Stone::Black => playing_stone = Stone::White,
                    Stone::White => playing_stone = Stone::Black,
                    _ => println!("error empty stone"),
                }
            } else {
                println!("invalid move, please try again");
            }
        } else {
            println!("wrong input\nquitting");
            playing = false
        }
    }
}

struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Coords {
            x,
            y,
        }
    }
    fn is_outside(&self, size: usize) -> bool {
        self.x >= size || self.y >= size
    }
    fn change_board(self, stone: Stone, board: &mut [[Stone; SIZE]; SIZE]) {
        board[self.y][self.x] = stone;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Stone {
    White,
    Black,
    Empty,
}
