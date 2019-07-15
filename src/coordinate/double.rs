//! Double Coordinates

use std::cmp;

use super::*;

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Double {
    pub col: i32,
    pub row: i32,
}

//////////////////////////////////////////////////////////////////////////////
// Traits
//////////////////////////////////////////////////////////////////////////////

impl From<(i32, i32)> for Double {
    fn from((col, row): (i32, i32)) -> Self {
        Self {
            col: col,
            row: row,
        }
    }
}

impl From<MultiCoord> for Double {
    fn from(coord: MultiCoord) -> Self {
        if coord.sys == CoordSys::Double {
            Double { col: coord.a, row: coord.b }
        } else {
            panic!("{:?} is not a Double coordinate", coord)
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Methods
//////////////////////////////////////////////////////////////////////////////

impl Double {
    //////////////////////////////////
    // Constants
    //////////////////////////////////

    /// Interlaced coordinate origin of (0, 0).
    pub const ORIGIN: Offset = Offset { col: 0, row: 0 };

    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    pub fn flat_to_cube(self) -> Cube {
        let x = self.col;
        let z = (self.row - self.col) / 2;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    pub fn sharp_to_cube(self) -> Cube {
        let x = (self.col - self.row) / 2;
        let z = self.row / 2;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    fn offset_map(self, offsets: [[i32; 2]; 6]) -> Vec<Self> {
        let mut neighbors = Vec::new();

        for side in 0..6 {
            let col = offsets[side][0] + self.col;
            let row = offsets[side][1] + self.row;

            neighbors.push(Double { col: col, row: row });
        }

        neighbors
    }

    pub fn flat_neighbors(self) -> Vec<Self> {
        let offsets = [[1, -1], [1, 1], [0, 2], [-1, 1], [-1, -1], [0, -2]];

        self.offset_map(offsets)
    }

    pub fn sharp_neighbors(self) -> Vec<Self> {
        let offsets = [[1, -1], [2, 0], [1, 1], [-1, 1], [-2, 0], [-1, -1]];

        self.offset_map(offsets)
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    pub fn flat_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        y_dist + cmp::max(0, (x_dist - y_dist) / 2)
    }

    pub fn sharp_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        x_dist + cmp::max(0, (y_dist - x_dist) / 2)
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[test]
fn test_it_works() {
    unimplemented!();
}
