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
// Traits: From & Into
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

    /// `Offset` coordinate origin of (0, 0).
    pub const ORIGIN: Offset = Offset { col: 0, row: 0 };

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    /// Create an `Offset` from two `i32` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::offset::Offset;
    ///
    /// assert_eq!(
    ///     Offset::from_coords(1, 2),
    ///     Offset { col: 1, row: 2 }
    /// );
    /// ```
    pub fn from_coords(col: i32, row: i32) -> Self {
        Self::from((col, row))
    }

    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    /// Converts an `Offset` to a `Cube`, assuming the `HexGrid` has
    /// `Parity::Odd` and `Tilt::Flat` parameters.
    pub fn oflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col - (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube::force_from_coords(x, y, z)
    }

    /// Converts an `Offset` to a `Cube`, assuming the `HexGrid` has
    /// `Parity::Even` and `Tilt::Flat` parameters.
    pub fn eflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col + (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube::force_from_coords(x, y, z)
    }

    /// Converts an `Offset` to a `Cube`, assuming the `HexGrid` has
    /// `Parity::Odd` and `Tilt::Sharp` parameters.
    pub fn osharp_to_cube(self) -> Cube {
        let x = self.col - (self.row - (self.row & 1)) / 2;
        let z = self.row;
        let y = 0 - x - z;

        Cube::force_from_coords(x, y, z)
    }

    /// Converts an `Offset` to a `Cube`, assuming the `HexGrid` has
    /// `Parity::Even` and `Tilt::Sharp` parameters.
    pub fn esharp_to_cube(self) -> Cube {
        let x = self.col - (self.row + (self.row & 1)) / 2;
        let z = self.row;
        let y = 0 - x - z;

        Cube::force_from_coords(x, y, z)
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

    /// Calculates the `Offset` coordinates of the hexes surrounding the
    /// calling instance, in the context of a `HexGrid` with `Parity::Odd` and
    /// `Tilt::Flat` parameters.
    pub fn oflat_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, -1], [1, 0], [0, 1], [-1, 0], [-1, -1], [0, -1]],
            [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [0, -1]],
        ];

        self.offset_map(offsets, self.col)
    }

    /// Calculates the `Offset` coordinates of the hexes surrounding the
    /// calling instance, in the context of a `HexGrid` with `Parity::Even`
    /// and `Tilt::Flat` parameters.
    pub fn eflat_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [0, -1]],
            [[1, -1], [1, 0], [0, 1], [-1, 0], [-1, -1], [0, -1]],
        ];

        self.offset_map(offsets, self.col)
    }

    /// Calculates the `Offset` coordinates of the hexes surrounding the
    /// calling instance, in the context of a `HexGrid` with `Parity::Odd` and
    /// `Tilt::Sharp` parameters.
    pub fn osharp_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[0, -1], [1, 0], [0, 1], [-1, 1], [-1, 0], [-1, -1]],
            [[1, -1], [1, 0], [1, 1], [0, 1], [-1, 0], [0, -1]],
        ];

        self.offset_map(offsets, self.row)
    }

    /// Calculates the `Offset` coordinates of the hexes surrounding the
    /// calling instance, in the context of a `HexGrid` with `Parity::Even`
    /// and `Tilt::Sharp` parameters.
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

    /// Calculates the distance between two `Offset` coordinates.
    pub fn dist(self, other: Self) -> i32 {
        self.eflat_to_cube().dist(other.eflat_to_cube())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////
