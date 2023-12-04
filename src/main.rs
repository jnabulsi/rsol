//TODO
//-scoreboard
//check if a move will result in a change to the board before allowing it to happen
//enter is valid input for some reason

mod board;
mod config;
mod tile;
use board::Board;
use config::constants::*;
use config::Direction;
use std::io;

fn make_move(board: &mut Board, direction: char) {
    match direction {
        'w' => board.move_tile(Direction::Up),
        's' => board.move_tile(Direction::Down),
        'a' => board.move_tile(Direction::Left),
        'd' => board.move_tile(Direction::Right),
        _ => println!("Invalid move! Please use 'w' (up), 's' (down), 'a' (left), or 'd' (right)."),
    }
}

fn main() {
    let mut board = Board::new(GRID_ROW, GRID_COL);

    while !board.empty_tiles.is_empty() {
        //pick a random tile
        let random_tile = Board::choose_random_tile(&mut board);

        if let Some(tile) = random_tile {
            //populate it with 2 or 4
            board.populate_tile(&tile);

            //remove the tile that is now non 0
            board.remove_tile(&tile);

            // Print the grid
            Board::print_board(&board);
        } else {
            // Handle the case when choose_random_tile returns None
            println!("No empty tile found");
        }

        Board::generate_empty_tiles(&board.tiles);
        Board::print_empty_tiles(&board);
        //take user input
        let mut input = String::new();
        println!("Enter move (w/s/a/d): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Make the move based on user input
        //let mut input: String = "w".to_string();
        if let Some(c) = input.chars().next() {
            make_move(&mut board, c);
        } else {
            println!("Invalid input!");
        }
    }
}
