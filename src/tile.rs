//tile struct and methods

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub value: u32,
}

impl Tile {
    //constuctor
    pub fn new() -> Self {
        Tile { value: 0 }
    }

    //add self to passed tiles value and set self value to 0
    pub fn merge(self, other: Tile) -> Tile {
        Tile {
            value: self.value + other.value,
        }
    }
}
