use crate::config::Direction;
use crate::Board;
use std::io::*;

//solves by each turn trying moves w, a, d, s in that order
//going for the first one that works
pub fn corner_solve(board: &mut Board) {
    let mut valid_move = false;
    let directions_to_try = vec![
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    for &direction in &directions_to_try {
        valid_move = board.move_tile(direction);

        if valid_move {
            // If a valid move is made, break out of the loop
            break;
        }
    }

    if !valid_move {
        println!("No valid moves left. Game over!");
    }
}

pub fn user_solve(board: &mut Board) {
    let mut valid_move = false;

    while !valid_move {
        //take user input
        let mut input = String::new();
        println!("Enter move (w/s/a/d): ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut input).expect("Failed to read line");

        if let Some(direction) = match input.trim() {
            "w" => Some(Direction::Up),
            "s" => Some(Direction::Down),
            "a" => Some(Direction::Left),
            "d" => Some(Direction::Right),
            _ => None,
        } {
            valid_move = board.move_tile(direction);

            if valid_move {
                // If a valid move is made, break out of the loop
                break;
            }
        } else {
            println!("Invalid input!");
        }
    }
}
