use rand::Rng;
use rand::seq::SliceRandom;

const GRID_SIZE: usize = 4;

fn initialize_board() -> Vec<Vec<u32>> {
    vec![vec![0; GRID_SIZE]; GRID_SIZE]
}

fn generate_empty_tiles(board: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut empty_tiles = Vec::new();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
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

fn main() {
    let mut board = initialize_board();
    let mut empty_tiles = generate_empty_tiles(&board);

    while let Some((row, col, index)) = choose_random_tile(&mut empty_tiles) {
        let value = if rand::thread_rng().gen_bool(0.9) { 2 } else { 4 };
        board[row][col] = value;

        // Remove the chosen tile from the list of empty tiles
        empty_tiles.remove(index);

        // Print the grid
        println!("------------");
        for row in &board {
            println!("{:?}", row);
        }
        println!("------------");
    }
}

