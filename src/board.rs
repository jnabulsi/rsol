use crate::config::constants::*;
use crate::config::Direction;
use crate::tile::Tile;
use rand::Rng;
use std::collections::HashSet;

pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub empty_tiles: HashSet<(usize, usize)>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        let tiles: Vec<Vec<Tile>> = (0..rows)
            .map(|_| (0..cols).map(|_| Tile::new()).collect())
            .collect();
        let empty_tiles = Self::generate_empty_tiles(&tiles);
        Self { tiles, empty_tiles }
    }

    pub fn generate_empty_tiles(tiles: &Vec<Vec<Tile>>) -> HashSet<(usize, usize)> {
        let mut empty_tiles = HashSet::new();

        for i in 0..tiles.len() {
            for j in 0..tiles[i].len() {
                if tiles[i][j].value == 0 {
                    empty_tiles.insert((i, j));
                }
            }
        }
        empty_tiles
    }

    pub fn print_empty_tiles(&self) {
        println!("Empty Tiles:");

        for &(row, col) in &self.empty_tiles {
            println!("Row: {}, Col: {}", row, col);
        }
    }

    pub fn choose_random_tile(&mut self) -> Option<(usize, usize)> {
        let random_index = rand::thread_rng().gen_range(0..self.empty_tiles.len());
        if let Some(random_tile) = self.empty_tiles.iter().nth(random_index) {
            Some(*random_tile)
        } else {
            None
        }
    }

    pub fn populate_tile(&mut self, tile: &(usize, usize)) {
        let row = tile.0;
        let col = tile.1;

        let value = if rand::thread_rng().gen_bool(0.9) {
            2
        } else {
            4
        };
        self.tiles[row][col].value = value;
    }

    pub fn remove_tile(&mut self, tile_to_remove: &(usize, usize)) {
        self.empty_tiles.remove(tile_to_remove);
    }

    pub fn add_tile_to_empty(&mut self, tile: (usize, usize)) {
        self.empty_tiles.insert(tile);
    }

    pub fn print_board(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{:4}", tile.value); // Adjust the format as needed
            }
            println!();
        }
        println!();
    }

    pub fn move_tile(&mut self, direction: Direction) -> bool {
        let (row_range, col_range, i_change, j_change) = match direction {
            Direction::Up => (&ROW_RANGE_UP[..], &COL_RANGE_UP[..], -1, 0),
            Direction::Down => (&ROW_RANGE_DOWN[..], &COL_RANGE_DOWN[..], 1, 0),
            Direction::Left => (&ROW_RANGE_LEFT[..], &COL_RANGE_LEFT[..], 0, -1),
            Direction::Right => (&ROW_RANGE_RIGHT[..], &COL_RANGE_RIGHT[..], 0, 1),
        };
        let mut changed = false;

        for &i in row_range {
            for &j in col_range {
                //only process non 0 tiles
                if self.tiles[i][j].value != 0 {
                    let mut current_i = i;
                    let mut current_j = j;

                    // loop to keep moving the tile until it can't move anymore
                    loop {
                        let next_i = (current_i as isize + i_change) as usize;
                        let next_j = (current_j as isize + j_change) as usize;

                        //check the next tile is in range
                        if next_i < self.tiles.len() && next_j < self.tiles[next_i].len() {
                            let current_value = self.tiles[current_i][current_j].value;
                            let next_value = self.tiles[next_i][next_j].value;

                            if current_value != 0 {
                                // number is found
                                // check if value in direction is 0
                                // or if values are the same
                                if next_value == 0 || next_value == current_value {
                                    changed = true;
                                    self.tiles[next_i][next_j] = self.tiles[next_i][next_j]
                                        .merge(self.tiles[current_i][current_j]);
                                    self.tiles[current_i][current_j].value = 0;
                                    self.add_tile_to_empty((current_i, current_j));
                                    self.remove_tile(&(next_i, next_j));
                                    if next_value == current_value {
                                        break;
                                    }

                                    // Move to the next position
                                    current_i = next_i;
                                    current_j = next_j;
                                } else {
                                    // Stop moving if conditions are not met
                                    break;
                                }
                            }
                        } else {
                            // Stop moving if the next position is out of bounds
                            break;
                        }
                    }
                }
            }
        }
        changed
    }
}
