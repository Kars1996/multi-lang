use std::io;
use std::process;

fn clear_console() {
    if cfg!(target_os = "windows") {
        print!("\x1Bc");
    } else {
        print!("\x1B[2J\x1B[H");
    }
}

fn print_board(board: &Vec<Vec<&str>>) {
    println!("  0 1 2");
    for (i, row) in board.iter().enumerate() {
        println!("{} {}", i, row.join(" "));
    }
}

fn is_move_valid(board: &Vec<Vec<&str>>, row: usize, col: usize) -> bool {
    board[row][col] == " "
}

fn is_board_full(board: &Vec<Vec<&str>>) -> bool {
    board.iter().all(|row| row.iter().all(|&cell| cell != " "))
}

fn check_winner(board: &Vec<Vec<&str>>, player: &str) -> bool {
    for i in 0..3 {
        if (0..3).all(|j| board[i][j] == player) || (0..3).all(|j| board[j][i] == player) {
            return true;
        }
    }

    if (0..3).all(|i| board[i][i] == player) || (0..3).all(|i| board[i][2 - i] == player) {
        return true;
    }

    false
}

fn get_available_moves(board: &Vec<Vec<&str>>) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == " " {
                moves.push((i, j));
            }
        }
    }
    moves
}

fn make_computer_move(board: &mut Vec<Vec<&str>>) -> (usize, usize) {
    let available_moves = get_available_moves(board);
    let random_move = available_moves.choose(&mut rand::thread_rng()).unwrap();
    *board.get_mut(random_move.0).unwrap().get_mut(random_move.1).unwrap() = "O";
    *random_move
}

fn make_player_move(board: &mut Vec<Vec<&str>>) -> (usize, usize) {
    loop {
        println!("Enter your move (row and column, e.g., 0 1): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: Vec<usize> = input.split_whitespace().filter_map(|s| s.parse().ok()).collect();

        if input.len() == 2 {
            let (row, col) = (input[0], input[1]);
            if 0 <= row && row < 3 && 0 <= col && col < 3 && is_move_valid(board, row, col) {
                *board.get_mut(row).unwrap().get_mut(col).unwrap() = "X";
                return (row, col);
            } else {
                println!("Invalid move. Try again.");
            }
        } else {
            println!("Invalid input. Please enter two integers separated by a space.");
        }
    }
}

fn play_game() {
    let mut board: Vec<Vec<&str>> = vec![vec![" "; 3]; 3];
    let mut player = "X";
    let computer = "O";
    clear_console();

    loop {
        print_board(&board);

        let (row, col) = if player == "X" {
            make_player_move(&mut board)
        } else {
            make_computer_move(&mut board)
        };

        if check_winner(&board, player) {
            clear_console();
            print_board(&board);
            println!("{} wins!", player);
            break;
        }

        if is_board_full(&board) {
            clear_console();
            print_board(&board);
            println!("It's a tie!");
            break;
        }

        player = if player == "X" { "O" } else { "X" };
        clear_console();
    }
}

fn main() {
    play_game();
}
