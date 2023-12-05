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

    //add values and return new tile with new value
    pub fn merge(self, other: Tile) -> Tile {
        Tile {
            value: self.value + other.value,
        }
    }
}
