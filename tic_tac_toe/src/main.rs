fn start_game() {
    let board: [[i8; 3]; 3] = [[0, 0, 0]; 3];
    print_board(board);
}

fn print_board(board: [[i8; 3]; 3]) {
    println!("{} {} {}", format(board[0][0]), format(board[0][1]), format(board[0][2]));
    println!("{} {} {}", format(board[1][0]), format(board[1][1]), format(board[1][2]));
    println!("{} {} {}", format(board[2][0]), format(board[2][1]), format(board[2][2]));
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