use std::io;
use rand::Rng;

fn main() {
    let mut move_stack: Vec<(usize, usize)> = Vec::new();
    let mut board: Vec<Vec<char>> = vec![vec![' '; 3]; 3];

    // Starting Player
    let mut current_player: char = starting_player_randomizer();

    println!("{} player starts first!\n", &current_player);
    loop {
        // Game Title
        println!(".: TIC TAC TOE :.");

        // Game Board
        display_board(&board);

        // Check if board is full.
        if move_stack.len() == 9 {
            println!("It's a draw!");
            break;
        }

        // Current Player
        println!("Current turn: {}", current_player);

        // Command
        let command = get_input();
        let clean_command: String = command.trim().to_lowercase();
        if clean_command == "undo" {
            undo_move(&mut move_stack, &mut board);
        } else {
            match parse_move(&clean_command) {
                Some((x, y)) => {
                    if make_move(&mut board, x, y, current_player) {
                        move_stack.push((x, y));
                        if check_winner(&board, current_player) {
                            println!("{} wins!", current_player);
                            break;
                        }
                        current_player = switch_player(current_player);
                    }
                }
                None => {
                    // Invalid move, prompt user to try again
                    println!("Invalid input. Please enter valid coordinates.");
                }
            }
        }
    }
}

fn starting_player_randomizer() -> char {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=1);
    match random_number {
        0 => 'X',
        1 => 'O',
        _ => unreachable!(),
    }
}

// Function: Getting input from terminal
fn get_input() -> String {
    print!("Please enter your command (e.g., '1 2' for move or 'undo'): ");
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    println!();
    command
}

// Function: Display board onto terminal
fn display_board(board: &Vec<Vec<char>>) {
    println!();
    for row in board {
        println!("     -------");
        print!("     ");
        for &cell in row {
            print!("|{}", cell);
        }
        print!("|\n");
    }
    println!("     -------\n");
}

// Function: Undo move
fn undo_move(move_stack: &mut Vec<(usize, usize)>, board: &mut Vec<Vec<char>>) {
    if let Some((x, y)) = move_stack.pop() {
        board[x][y] = ' ';
        println!("Move undone.");
    } else {
        println!("No more moves to undo!");
    }
}

// Function: Making move after validating move
fn make_move(board: &mut Vec<Vec<char>>, x: usize, y: usize, current_player: char) -> bool {
    if x >= 3 || y >= 3 {
        println!("Choose index in between 0 and 2 (inclusive)");
        return false;
    }

    if board[x][y] == ' ' {
        board[x][y] = current_player;
        true
    } else {
        println!("This spot is already taken! Try again.");
        false
    }
}

// Function: Checking for winner
fn check_winner(board: &Vec<Vec<char>>, current_player: char) -> bool {
    for i in 0..3 {
        // Check rows and columns
        if (board[i][0] == current_player && board[i][1] == current_player && board[i][2] == current_player) ||
           (board[0][i] == current_player && board[1][i] == current_player && board[2][i] == current_player) {
            return true;
        }
    }
    // Check diagonals
    if (board[0][0] == current_player && board[1][1] == current_player && board[2][2] == current_player) ||
       (board[0][2] == current_player && board[1][1] == current_player && board[2][0] == current_player) {
        return true;
    }
    false
}

// Function: Parsing input by user into move commands
fn parse_move(command: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    if parts.len() != 2 {
        println!("Invalid input. Please enter coordinates in the format 'x y'.");
        return None;
    }

    let x = match parts[0].parse::<usize>() {
        Ok(num) if num < 3 => num,
        _ => {
            println!("Invalid x coordinate. Please enter a number between 0 and 2.");
            return None;
        }
    };

    let y = match parts[1].parse::<usize>() {
        Ok(num) if num < 3 => num,
        _ => {
            println!("Invalid y coordinate. Please enter a number between 0 and 2.");
            return None;
        }
    };

    Some((x, y))
}

// Function: Switching player turn
fn switch_player(current_player: char) -> char {
    match current_player {
        'X' => 'O',
        'O' => 'X',
        _ => unreachable!(), // This specific condition should never occur, and if it does, it will cause a panic.
    }
}
