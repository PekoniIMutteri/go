mod display;
mod inputs;

const SIZE: usize = 4;

fn main() {
    let mut board = [
        [Stone::Black, Stone::Empty, Stone::Black, Stone::Black],
        [Stone::Empty, Stone::White, Stone::White, Stone::White],
        [Stone::Empty, Stone::Empty, Stone::White, Stone::Empty],
        [Stone::Empty, Stone::White, Stone::Empty, Stone::Empty],
    ];
    display::draw_board(&board);
    if let Some(coords) = inputs::user_input(SIZE) {
        board[coords.y][coords.x] = Stone::Black;
        display::draw_board(&board);
    } else {
        println!("error");
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
}

#[derive(Clone, Copy)]
enum Stone {
    White,
    Black,
    Empty,
}
