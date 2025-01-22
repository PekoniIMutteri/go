//use std::io;

//const SIZE: usize = 5;

mod display;

fn main() {
    let board = [
        [Stone::Black, Stone::Empty],
        [Stone::Empty, Stone::White],
    ];
    display::draw_board(&board);
}

#[derive(Clone, Copy)]
enum Stone {
    White,
    Black,
    Empty,
}



/* 

fn main() {
    let mut board = [[Stone::Empty; SIZE]; SIZE];
    play(&mut board);
}

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

fn inputs() -> [usize; 2] {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error reading");
    let mut substr_iter = line.trim().split_whitespace();
    let mut next_num = || ->  usize {
        let next = substr_iter.next();
        if let Some(string) = next {
            let parsed = string.parse();
            if !parsed.is_err() {
                return parsed.unwrap();
            }
        }
        250
    };
    let mut inputs = [250; 2];
    inputs[0] = next_num();
    if inputs[0] != 250 {
        inputs[1] = next_num();
    }
    inputs
}

*/