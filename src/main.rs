use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

fn move_tiles(
    board: &mut Vec<Vec<u32>>,
    empty_tiles: &mut Vec<(usize, usize)>,
    direction: Direction,
) {
    let (di, dj) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };
    //loop through board starting from second row
    for i in 1..GRID_SIZE {
        for j in 0..GRID_SIZE {
            //if there is a number
            if board[i][j] != 0 {
                //if 0 above the number move the number up
                let mut k = 1;
                while i >= k && board[i - k][j] == 0 {
                    board[i - k][j] = board[i - k + 1][j];
                    board[i - k + 1][j] = 0;

                    //add now empty tile to list of empty tiles and remove the one now populated
                    empty_tiles.push((i - k + 1, j));
                    empty_tiles.retain(|&tile| tile != (i - k, j));
                    k += 1;
                }

                //if above is same combine values
                if i >= k && board[i - k][j] == board[i - k + 1][j] {
                    board[i - k][j] += board[i - k + 1][j];
                    board[i - k + 1][j] = 0;

                    //add now empty tile to list of empty tiles
                    empty_tiles.push((i - k + 1, j));
                }
            }
        }
    }
}

fn make_move(board: &mut Vec<Vec<u32>>, empty_tiles: &mut Vec<(usize, usize)>, direction: char) {
    match direction {
        'w' => move_tiles(board, empty_tiles, Direction::Up),
        's' => move_tiles(board, empty_tiles, Direction::Down),
        'a' => move_tiles(board, empty_tiles, Direction::Left),
        'd' => move_tiles(board, empty_tiles, Direction::Right),
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
