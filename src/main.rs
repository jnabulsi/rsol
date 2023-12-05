//TODO
//-scoreboard
//-change how the constants work to be iterators
//
mod board;
mod config;
mod solve;
mod tile;
use board::Board;
use config::constants::*;
use solve::{corner_solve, user_solve};

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
        //functions for debug purpose
        //Board::generate_empty_tiles(&board.tiles);
        //Board::print_empty_tiles(&board);
        //corner_solve(&mut board);
        user_solve(&mut board);
    }
}
