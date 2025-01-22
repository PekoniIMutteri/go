use super::Stone;

pub fn draw_board<const N: usize>(board: &[[Stone; N]; N]) {
    draw_stone_line(&board[0]);
    for index in 1..N {
        draw_empty_line(N);
        draw_stone_line(&board[index]);
    }
    print!("\n");
}

fn draw_stone_line<const N: usize>(line: &[Stone; N]) {
    draw_stone(&line[0]);
    for index in 1..N {
        print!("---");
        draw_stone(&line[index]);
    }
    print!("\n");
}

fn draw_stone(stone: &Stone) {
    match stone {
        Stone::Black => print!("C"),
        Stone::White => print!("0"),
        Stone::Empty => print!(" "),
    }
}

fn draw_empty_line(lenght: usize) {
    for _ in 0..lenght {
        print!("|   ");
    }
    print!("\n");
}


/*
pub fn draw_board<const N: usize>(board: &[[Stone; N]; N]) {
    let mut start = true;
    for line in board {
        if !start {
            print!("|");
            for _ in 1..N {
                print!("   |");
            }
            print!("\n");
        } else {
            start = false
        }
        draw_line(line);
    }
}

fn draw_line<const N: usize>(line: &[Stone; N]) {
    let mut start = true;
    for stone in line {
        if !start {
            print!("---");
        } else {
            start = false;
        }
        match stone {
            Stone::White => print!("C"),
            Stone::Black => print!("0"),
            Stone::Empty => print!(" "),
        }
    }
    print!("\n");
}
*/

