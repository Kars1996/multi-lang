use rand::Rng;

#[derive(Debug)]
enum TicTacToeError {
    InvalidInput,
    SpaceAlreadySelected,
    InvalidSpace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayerSymbol {
    X,
    O,
}

struct Player<'a> {
    name: &'a str,
    symbol: PlayerSymbol,
    score: u32,
    match_wins: u32,
}

impl<'a> Player<'a> {
    fn new(name: &'a str, symbol: PlayerSymbol) -> Player<'a> {
        Player {
            name,
            symbol,
            score: 0,
            match_wins: 0,
        }
    }

    fn win(&mut self) {
        self.score += 1;
    }

    fn match_win(&mut self) {
        self.match_wins += 1;
    }

    fn reset_match(&mut self) {
        self.match_wins = 0;
    }

    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_symbol(&self) -> PlayerSymbol {
        self.symbol
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn get_match_wins(&self) -> u32 {
        self.match_wins
    }
}

struct ComputerPlayer {
    difficulty: &'static str,
    strategy: &'static str,
    tactic: &'static str,
    last_move: &'static str,
    reiterate: bool,
}

impl ComputerPlayer {
    fn new(difficulty: &'static str) -> ComputerPlayer {
        ComputerPlayer {
            difficulty,
            strategy: "",
            tactic: "",
            last_move: "",
            reiterate: false,
        }
    }
}

struct TicTacToe<'a> {
    board: Vec<Vec<&'a str>>,
    players: Vec<Player<'a>>,
    computer_player: ComputerPlayer,
    selected: Vec<&'a str>,
}

impl<'a> TicTacToe<'a> {
    fn new(players: Vec<Player<'a>>, computer_player: ComputerPlayer) -> TicTacToe<'a> {
        let board = vec![vec![" "; 3]; 3];
        TicTacToe {
            board,
            players,
            computer_player,
            selected: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.board = vec![vec![" "; 3]; 3];
        self.selected.clear();
    }

    fn draw(&self) {
        println!();
        for row in &self.board {
            println!("   {}", row.join(" | "));
            println!("  -----------");
        }
        println!();
    }

    fn place(&mut self, symbol: PlayerSymbol, space: &'a str) -> Result<(), TicTacToeError> {
        if self.selected.contains(&space) {
            return Err(TicTacToeError::SpaceAlreadySelected);
        }

        let (row, col) = match space {
            "UL" => (0, 0),
            "UM" => (0, 1),
            "UR" => (0, 2),
            "CL" => (1, 0),
            "CM" => (1, 1),
            "CR" => (1, 2),
            "BL" => (2, 0),
            "BM" => (2, 1),
            "BR" => (2, 2),
            _ => return Err(TicTacToeError::InvalidSpace),
        };

        self.board[row][col] = match symbol {
            PlayerSymbol::X => "X",
            PlayerSymbol::O => "O",
        };

        self.selected.push(space);

        Ok(())
    }

    fn check_winner(&self, symbol: &'a str) -> bool {
        for row in &self.board {
            if row.iter().all(|&cell| cell == symbol) {
                return true;
            }
        }

        for col in 0..3 {
            if (0..3).all(|row| self.board[row][col] == symbol) {
                return true;
            }
        }

        if (0..3).all(|i| self.board[i][i] == symbol) || (0..3).all(|i| self.board[i][2 - i] == symbol) {
            return true;
        }

        false
    }

    fn check_draw(&self) -> bool {
        self.selected.len() == 9
    }
}

fn next_turn(turn_list: &'a [&'a str], current_turn: &'a str) -> &'a str {
    if let Some(current_index) = turn_list.iter().position(|&turn| turn == current_turn) {
        if current_index + 1 == turn_list.len() {
            turn_list[0]
        } else {
            turn_list[current_index + 1]
        }
    } else {
        current_turn
    }
}

fn debug(statement: &str) {
    println!("{}", statement);
}

fn main() {
    let mut rng = rand::thread_rng();

    let players = vec![
        Player::new("Player 1", PlayerSymbol::O),
        Player::new("Computer", PlayerSymbol::X),
    ];
    let computer_player = ComputerPlayer::new("Easy");
    let mut tic_tac_toe = TicTacToe::new(players, computer_player);

    let turn_list = ["Player 1", "Computer"];
    let mut turn = rng.gen_range(0..turn_list.len());

    let first_turn = turn;

    while tic_tac_toe.players.iter().any(|player| player.match_wins < 2) {
        tic_tac_toe.draw();

        let current_turn = &turn_list[turn];
        if current_turn != "Computer" {
            println!("{} please select a space.", current_turn);
            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).expect("Failed to read line");
            let selection = selection.trim().to_uppercase();

            if selection == "END" {
                break;
            }

            let result = match tic_tac_toe.place(PlayerSymbol::O, &selection) {
                Ok(_) => Ok(()),
                Err(TicTacToeError::SpaceAlreadySelected) => Err(TicTacToeError::InvalidInput),
                Err(TicTacToeError::InvalidSpace) => Err(TicTacToeError::InvalidInput),
            };

            if result.is_err() {
                continue;
            }
        } else {
            println!("Computer Turn");
            let computer_selection = "Some logic to determine computer's move"; // Replace with actual logic
            let result = tic_tac_toe.place(PlayerSymbol::X, computer_selection);
            if result.is_err() {
                continue;
            }
        }

        if tic_tac_toe.check_winner("X") {
            tic_tac_toe.draw();
            tic_tac_toe.selected.clear();
            let winner = turn_list[turn].clone();
            let loser = next_turn(&turn_list, turn);
            if let Some(player) = tic_tac_toe.players.iter_mut().find(|player| player.get_name() == winner) {
                if player.get_match_wins() == 1 {
                    println!("{} wins!", player.get_name());
                    player.win();
                    player.reset_match();
                    tic_tac_toe.players.iter_mut().for_each(|p| p.reset_match());
                    tic_tac_toe.clear();
                    turn = first_turn;
                } else if player.get_match_wins() == 0 {
                    println!("{} won a match! Beginning next round.", player.get_name());
                    player.match_win();
                }
            }
        } else if tic_tac_toe.check_draw() {
            tic_tac_toe.draw();
            println!("Draw! Beginning next round.");
            tic_tac_toe.players.iter_mut().for_each(|p| p.reset_match());
            tic_tac_toe.clear();
            turn = first_turn;
        } else {
            turn = (turn + 1) % turn_list.len();
        }
    }

    println!("\n\t\tCurrent Score\n");
    for player in &tic_tac_toe.players {
        println!("\t{}\t\t\t{}", player.get_name(), player.get_score());
    }
    println!("\n==========================================\n");
}
