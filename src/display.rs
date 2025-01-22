use super::Stone;

pub fn draw_board<const N: usize>(board: &[[Stone; N]; N]) {
    draw_stone_line(&board[N - 1]);
    for index in 1..N {
        draw_empty_line(N);
        draw_stone_line(&board[N - 1 - index]);
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
