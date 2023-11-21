//movment.rs
use crate::config;

pub fn move_up(board: &mut Vec<Vec<u32>>, empty_tiles: &mut Vec<(usize, usize)>) {
    //loop through board starting from second row
    for i in 1..config::constants::GRID_SIZE {
        for j in 0..config::constants::GRID_SIZE {
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

pub fn move_down(board: &mut Vec<Vec<u32>>, empty_tiles: &mut Vec<(usize, usize)>) {
    // Loop through the board starting from the second-to-last row and going upwards
    for i in (0..config::constants::GRID_SIZE - 1).rev() {
        for j in 0..config::constants::GRID_SIZE {
            // If there is a number
            if board[i][j] != 0 {
                // If 0 below the number, move the number down
                let mut k = 1;
                while i + k < config::constants::GRID_SIZE && board[i + k][j] == 0 {
                    board[i + k][j] = board[i + k - 1][j];
                    board[i + k - 1][j] = 0;

                    // Add now empty tile to the list of empty tiles and remove the one now populated
                    empty_tiles.push((i + k - 1, j));
                    empty_tiles.retain(|&tile| tile != (i + k, j));
                    k += 1;
                }

                // If below is the same, combine values
                if i + k < config::constants::GRID_SIZE && board[i + k][j] == board[i + k - 1][j] {
                    board[i + k][j] += board[i + k - 1][j];
                    board[i + k - 1][j] = 0;

                    // Add now empty tile to the list of empty tiles
                    empty_tiles.push((i + k - 1, j));
                }
            }
        }
    }
}

pub fn move_left(board: &mut Vec<Vec<u32>>, empty_tiles: &mut Vec<(usize, usize)>) {
    // Loop through the board starting from the second column and going to the right
    for j in 1..config::constants::GRID_SIZE {
        for i in 0..config::constants::GRID_SIZE {
            // If there is a number
            if board[i][j] != 0 {
                // If 0 to the left of the number, move the number left
                let mut k = 1;
                while j >= k && board[i][j - k] == 0 {
                    board[i][j - k] = board[i][j - k + 1];
                    board[i][j - k + 1] = 0;

                    // Add now empty tile to the list of empty tiles and remove the one now populated
                    empty_tiles.push((i, j - k + 1));
                    empty_tiles.retain(|&tile| tile != (i, j - k));
                    k += 1;
                }

                // If left is the same, combine values
                if j >= k && board[i][j - k] == board[i][j - k + 1] {
                    board[i][j - k] += board[i][j - k + 1];
                    board[i][j - k + 1] = 0;

                    // Add now empty tile to the list of empty tiles
                    empty_tiles.push((i, j - k + 1));
                }
            }
        }
    }
}

pub fn move_right(board: &mut Vec<Vec<u32>>, empty_tiles: &mut Vec<(usize, usize)>) {
    // Loop through the board starting from the second-to-last column and going to the left
    for j in (0..config::constants::GRID_SIZE - 1).rev() {
        for i in 0..config::constants::GRID_SIZE {
            // If there is a number
            if board[i][j] != 0 {
                // If 0 to the right of the number, move the number right
                let mut k = 1;
                while j + k < config::constants::GRID_SIZE && board[i][j + k] == 0 {
                    board[i][j + k] = board[i][j + k - 1];
                    board[i][j + k - 1] = 0;

                    // Add now empty tile to the list of empty tiles and remove the one now populated
                    empty_tiles.push((i, j + k - 1));
                    empty_tiles.retain(|&tile| tile != (i, j + k));
                    k += 1;
                }

                // If right is the same, combine values
                if j + k < config::constants::GRID_SIZE && board[i][j + k] == board[i][j + k - 1] {
                    board[i][j + k] += board[i][j + k - 1];
                    board[i][j + k - 1] = 0;

                    // Add now empty tile to the list of empty tiles
                    empty_tiles.push((i, j + k - 1));
                }
            }
        }
    }
}
