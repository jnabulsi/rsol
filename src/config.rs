//config.rs

pub mod constants {
    pub const GRID_COL: usize = 4;
    pub const GRID_ROW: usize = 4;

    pub const ROW_RANGE_UP: [usize; 4] = [0, 1, 2, 3];
    pub const COL_RANGE_UP: [usize; 4] = [0, 1, 2, 3];

    pub const ROW_RANGE_DOWN: [usize; 4] = [3, 2, 1, 0];
    pub const COL_RANGE_DOWN: [usize; 4] = [0, 1, 2, 3];

    pub const ROW_RANGE_LEFT: [usize; 4] = [0, 1, 2, 3];
    pub const COL_RANGE_LEFT: [usize; 4] = [0, 1, 2, 3];

    pub const ROW_RANGE_RIGHT: [usize; 4] = [0, 1, 2, 3];
    pub const COL_RANGE_RIGHT: [usize; 4] = [3, 2, 1, 0];
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
