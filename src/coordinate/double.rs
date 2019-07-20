//! Double Coordinates

use std::cmp;

use super::*;

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
//////////////////////////////////////////////////////////////////////////////

/// Interlaced (Double) coordinates are similar to offset coordinates.
/// However, instead of alternating rows and columns, the horizontal or
/// vertical step size of the adjacent hexes are doubled. This can be useful
/// when calculating rendering offsets, and a variety of other operations.
///
/// For a `Sharp` grid, the column value increases by a factor of two with
/// each column. For a `Flat` grid, the row value increases by a factor of two
/// with each row. In both cases, the sum of a valid coordinate's row and
/// column values must be even, i.e. for interlaced coordinate (col, row),
/// (col + row) % 2 == 0.
///
/// `Double`s are opaque, to enforce constraints.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Double {
    col: i32,
    row: i32,
}

//////////////////////////////////////////////////////////////////////////////
// Traits: From & Into
//////////////////////////////////////////////////////////////////////////////

/// Creates a `Double` from an `(i32, i32)`.
///
/// # Panics
///
/// Panics when the sum of the tuple elements is odd.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::double::Double;
///
/// let double = Double::from((0, 2));
///
/// assert_eq!(double.col(), 0);
/// assert_eq!(double.row(), 2);
/// ```
impl From<(i32, i32)> for Double {
    fn from((col, row): (i32, i32)) -> Self {
        if (col + row) & 1 == 0 {
            Self {
                col: col,
                row: row,
            }
        } else {
            panic!("({}, {}) is an invalid double coordinate", col, row);
        }
    }
}

/// Creates a `Double` from a `MultiCoord`.
///
/// # Panics
///
/// Panics when the `MultiCoord` encodes an `Axial`, `Cube`, or `Offset`.
impl From<MultiCoord> for Double {
    fn from(coord: MultiCoord) -> Self {
        if coord.sys == CoordSys::Double {
            Self {
                col: coord.a,
                row: coord.b,
            }
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

    /// `Double` coordinate origin of (0, 0).
    pub const ORIGIN: Offset = Offset { col: 0, row: 0 };

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    /// Create a `Double` from two `i32` values.
    ///
    /// # Panics
    ///
    /// Panics when the sum of the `i32` values is odd.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::double::Double;
    ///
    /// let double = Double::from_coords(1, 3);
    ///
    /// assert_eq!(double.col(), 1);
    /// assert_eq!(double.row(), 3);
    /// ```
    pub fn from_coords(col: i32, row: i32) -> Self {
        Self::from((col, row))
    }

    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    /// Convert a `Double` to a `Cube`, assuming the grid has a `Tilt::Flat`.
    pub fn flat_to_cube(self) -> Cube {
        let x = self.col;
        let z = (self.row - self.col) / 2;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    /// Convert a `Double` to a `Cube`, assuming the grid has a `Tilt::Sharp`.
    pub fn sharp_to_cube(self) -> Cube {
        let x = (self.col - self.row) / 2;
        let z = self.row / 2;
        let y = 0 - x - z;

        Cube::from_coords(x, y, z)
    }

    //////////////////////////////////
    // Retrieval
    //////////////////////////////////

    /// Return the column value of a `Double`.
    pub fn col(&self) -> i32 {
        self.col
    }

    /// Return the row value of a `Double`.
    pub fn row(&self) -> i32 {
        self.row
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

    /// Calculates the `Double` coordinates of the hexes surrounding the
    /// calling instance, in the context of a `HexGrid` whose hexes have the
    /// `Tilt::Flat` orientation.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn flat_neighbors(self) -> Vec<Self> {
        let offsets = [[1, -1], [1, 1], [0, 2], [-1, 1], [-1, -1], [0, -2]];

        self.offset_map(offsets)
    }

    /// Calculates the `Double` coordinates of the hexes surrounding the
    /// calling instance, in the context of a `HexGrid` whose hexes have the
    /// `Tilt::Sharp` orientation.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn sharp_neighbors(self) -> Vec<Self> {
        let offsets = [[1, -1], [2, 0], [1, 1], [-1, 1], [-2, 0], [-1, -1]];

        self.offset_map(offsets)
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    /// Calculates the distance between two `Double` coordinates in the
    /// context of a `HexGrid` whose hexes have the `Tilt::Flat` orientation.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn flat_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        y_dist + cmp::max(0, (x_dist - y_dist) / 2)
    }

    /// Calculates the distance between two `Double` coordinates in the
    /// context of a `HexGrid` whose hexes have the `Tilt::Sharp` orientation.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn sharp_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        x_dist + cmp::max(0, (y_dist - x_dist) / 2)
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////
