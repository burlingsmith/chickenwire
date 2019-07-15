//! Offset Coordinates

use super::*;

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Offset {
    pub col: i32,
    pub row: i32,
}

//////////////////////////////////////////////////////////////////////////////
// Traits
//////////////////////////////////////////////////////////////////////////////

impl From<(i32, i32)> for Offset {
    fn from((col, row): (i32, i32)) -> Self {
        Self {
            col: col,
            row: row,
        }
    }
}

impl From<MultiCoord> for Offset {
    fn from(coord: MultiCoord) -> Self {
        if coord.sys == CoordSys::Offset {
            Offset { col: coord.a, row: coord.b }
        } else {
            panic!("{:?} is not an Offset coordinate", coord)
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Methods
//////////////////////////////////////////////////////////////////////////////

impl Offset {
    //////////////////////////////////
    // Constants
    //////////////////////////////////

    /// Offset coordinate origin of (0, 0).
    pub const ORIGIN: Offset = Offset { col: 0, row: 0 };

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    pub fn from_coords(col: i32, row: i32) -> Self {
        Self::from((col, row))
    }

    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    pub fn oflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col - (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    pub fn eflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col + (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    pub fn osharp_to_cube(self) -> Cube {
        let x = self.col - (self.row - (self.row & 1)) / 2;
        let z = self.row;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    pub fn esharp_to_cube(self) -> Cube {
        let x = self.col - (self.row + (self.row & 1)) / 2;
        let z = self.row;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    fn offset_map(
        self,
        offsets: [[[i32; 2]; 6]; 2],
        parity_check: i32
    ) -> Vec<Self> {
        let mut neighbors = Vec::new();

        for side in 0..6 {
            let parity: usize = (parity_check & 1) as usize;
            let offset_pair = offsets[parity][side];

            let col = self.col + offset_pair[0];
            let row = self.row + offset_pair[1];

            neighbors.push(Offset { col: col, row: row });
        }

        neighbors
    }

    pub fn oflat_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, -1], [1, 0], [0, 1], [-1, 0], [-1, -1], [0, -1]],
            [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [0, -1]],
        ];

        self.offset_map(offsets, self.col)
    }

    pub fn eflat_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [0, -1]],
            [[1, -1], [1, 0], [0, 1], [-1, 0], [-1, -1], [0, -1]],
        ];

        self.offset_map(offsets, self.col)
    }

    pub fn osharp_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[0, -1], [1, 0], [0, 1], [-1, 1], [-1, 0], [-1, -1]],
            [[1, -1], [1, 0], [1, 1], [0, 1], [-1, 0], [0, -1]],
        ];

        self.offset_map(offsets, self.row)
    }

    pub fn esharp_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, -1], [1, 0], [1, 1], [0, 1], [-1, 0], [0, -1]],
            [[0, -1], [1, 0], [0, 1], [-1, 1], [-1, 0], [-1, -1]],
        ];

        self.offset_map(offsets, self.row)
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    pub fn dist(self, other: Self) -> i32 {
        self.eflat_to_cube().dist(other.eflat_to_cube())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////
