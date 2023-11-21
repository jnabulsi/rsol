//TODO
//-scoreboard
//-check for remaining moves before ending the game
//check if a move will result in a change to the board before allowing it to happen

mod config;
mod movement;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

fn initialize_board() -> Vec<Vec<u32>> {
    vec![vec![0; config::constants::GRID_SIZE]; config::constants::GRID_SIZE]
}

fn generate_empty_tiles(board: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut empty_tiles = Vec::new();

    for i in 0..config::constants::GRID_SIZE {
        for j in 0..config::constants::GRID_SIZE {
            if board[i][j] == 0 {
                empty_tiles.push((i, j));
            }
        }
    }

    empty_tiles
}

fn choose_random_tile(empty_tiles: &mut Vec<(usize, usize)>) -> Option<(usize, usize, usize)> {
    empty_tiles.choose(&mut rand::thread_rng()).map(|&tile| {
        let index = empty_tiles.iter().position(|&t| t == tile).unwrap();
        (tile.0, tile.1, index)
    })
}

fn make_move(board: &mut Vec<Vec<u32>>, empty_tiles: &mut Vec<(usize, usize)>, direction: char) {
    match direction {
        'w' => movement::move_up(board, empty_tiles),
        's' => movement::move_down(board, empty_tiles),
        'a' => movement::move_left(board, empty_tiles),
        'd' => movement::move_right(board, empty_tiles),
        _ => println!("Invalid move! Please use 'w' (up), 's' (down), 'a' (left), or 'd' (right)."),
    }
}

fn main() {
    let mut board = initialize_board();
    let mut empty_tiles = generate_empty_tiles(&board);

    while let Some((row, col, index)) = choose_random_tile(&mut empty_tiles) {
        let value = if rand::thread_rng().gen_bool(0.9) {
            2
        } else {
            4
        };

        board[row][col] = value;

        // Remove the chosen tile from the list of empty tiles
        empty_tiles.remove(index);

        // Print the grid
        println!("------------");
        for row in &board {
            println!("{:?}", row);
        }
        println!("------------");

        //take user input
        let mut input = String::new();
        println!("Enter move (w/s/a/d): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Make the move based on user input
        if let Some(c) = input.chars().next() {
            make_move(&mut board, &mut empty_tiles, c);
        } else {
            println!("Invalid input!");
        }
    }
}
