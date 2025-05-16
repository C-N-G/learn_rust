use std::io;

fn main() {
    let mut board: [[i32; 4]; 4]= [[0, 0, 0, 0]; 4];
    board[0] = [9, 3, 4, 5];
    board[1][0] = 6;
    board[2][0] = 7;
    board[3][0] = 8;
    let mut is_first_player_turn = true; 
    loop {
        print_board(board);
        let mut game_move = String::new();
        io::stdin()
            .read_line(&mut game_move)
            .expect("Failed to read line");
        let game_move: String = match game_move.trim().parse() {
            Ok(pos) => pos,
            Err(_) => continue,
        };
        println!("your move: {game_move}");
        let move_string: &str = &game_move;
        let move_to_place = if is_first_player_turn { 1 } else { 2 };
        let mut made_invalid_move = false;
        match move_string {
            "a1" => board[1][1] = move_to_place,
            "a2" => board[1][2] = move_to_place,
            "a3" => board[1][3] = move_to_place,
            "b1" => board[2][1] = move_to_place,
            "b2" => board[2][2] = move_to_place,
            "b3" => board[2][3] = move_to_place,
            "c1" => board[3][1] = move_to_place,
            "c2" => board[3][2] = move_to_place,
            "c3" => board[3][3] = move_to_place,
            _ => {
                println!("invalid move {game_move}");
                made_invalid_move = true;
            }
        };

        if !made_invalid_move {
            is_first_player_turn = !is_first_player_turn;
        }
    }
}

fn print_board(board: [[i32; 4]; 4]) {
    for row in board {
        for pos in row {
            print!("{}", format(pos));
        }
        println!("");
    }
}

// struct Board {
//     positions: [[i32; 4]; 4]
// }

// enum Position {
//     Naught,
//     Cross,
//     Empty,
//     Marker(String)
// }

fn format(value: i32) -> String {
    match value {
        0 => String::from("[ ]"),
        1 => String::from("[X]"),
        2 => String::from("[O]"),
        3 => String::from(" 1 "),
        4 => String::from(" 2 "),
        5 => String::from(" 3 "),
        6 => String::from(" a "),
        7 => String::from(" b "),
        8 => String::from(" c "),
        _ => String::from("   ")
    }
}