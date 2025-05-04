fn main() {
    let board: [[i8; 3]; 3] = [[0, 0, 0]; 3];
    print_board(board);
}

fn print_board(board: [[i8; 3]; 3]) {
    for row in board {
        for pos in row {
            print!("{}", format(pos));
        }
        println!("");
    }
}

fn format(value: i8) -> &'static str {
    if value == 1 {
        return "[X]";
    } else if value == 2 {
        return "[Y]";
    } else {
        return "[ ]";
    }
}