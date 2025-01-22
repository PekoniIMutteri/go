//use std::io;

//const SIZE: usize = 5;

mod display;
mod inputs;

fn main() {
    let board = [
        [Stone::Black, Stone::Empty, Stone::Black, Stone::Black],
        [Stone::Empty, Stone::White, Stone::White, Stone::White],
        [Stone::Empty, Stone::Empty, Stone::White, Stone::Empty],
        [Stone::Empty, Stone::White, Stone::Empty, Stone::Empty],
    ];
    display::draw_board(&board);
    if let Some(coords) = inputs::user_input() {
        println!("{} {}", coords.x, coords.y);
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
}

#[derive(Clone, Copy)]
enum Stone {
    White,
    Black,
    Empty,
}



/* 

fn play<const N: usize>(board: &mut [[Stone; N]; N]) {
    let mut stone = Stone::Black;
    loop {
        draw_board(board);
        print!("\n");
        println!("next");
        let coords = inputs();
        if coords[0] == 250 || coords[1] == 250 || coords[0] >= N || coords[1] >= N {
            break;
        }
        board[coords[1]][coords[0]] = stone;
        match stone {
            Stone::Black => stone = Stone::White,
            Stone::White => stone = Stone::Black,
            _ => (),
        }
    }
    draw_board(board);
    println!("\nquit");
}

*/