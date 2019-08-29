//! Double Coordinates

use std::cmp;
use std::ops::{Add, Mul, Sub};

use super::*;

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
//////////////////////////////////////////////////////////////////////////////

/// `Double` coordinates are similar to offset coordinates. However, instead
/// of alternating rows and columns, the horizontal or vertical step size of
/// the adjacent hexes are doubled. This is useful for a variety of
/// operations.
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
// Traits: Arithmetic
//////////////////////////////////////////////////////////////////////////////

/// Adds two `Double` coordinates in the same manner as vectors.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::double::Double;
///
/// assert_eq!(
///     Double::force_from_coords(-2, 2) + Double::force_from_coords(3, 5),
///     Double::force_from_coords(1, 7)
/// );
/// ```
impl Add for Double {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

/// Subtracts two `Double` coordinates in the same manner as vectors.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::double::Double;
///
/// assert_eq!(
///     Double::force_from_coords(3, 1) - Double::force_from_coords(6, 2),
///     Double::force_from_coords(-3, -1)
/// );
/// ```
impl Sub for Double {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            col: self.col - other.col,
            row: self.row - other.row,
        }
    }
}

/// Multiplies a `Double` coordinate by an `i32` scalar, like a vector.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::double::Double;
///
/// assert_eq!(
///     Double::force_from_coords(1, 3) * 5,
///     Double::force_from_coords(5, 15)
/// );
/// assert_eq!(
///     0 * Double::force_from_coords(100, 2),
///     Double::ORIGIN
/// );
/// assert_eq!(
///     Double::force_from_coords(1, 3) * (-1),
///     Double::force_from_coords(-1, -3)
/// );
/// ```
impl Mul<i32> for Double {
    type Output = Self;

    fn mul(self, n: i32) -> Self {
        Self {
            col: self.col * n,
            row: self.row * n,
        }
    }
}

impl Mul<Double> for i32 {
    type Output = Double;

    fn mul(self, coord: Double) -> Double {
        coord * self
    }
}

//////////////////////////////////////////////////////////////////////////////
// Traits: From & Into
//////////////////////////////////////////////////////////////////////////////

/// Creates a `Double` from an `(i32, i32)`.
///
/// # Panics
///
/// Panics when the sum of the tuple elements is odd, since this violates the
/// constraints of the `Double` coordinate system.
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
            Self { col: col, row: row }
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
    pub const ORIGIN: Double = Double { col: 0, row: 0 };

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    /// Attempts to create a `Double` from two `i32` values. If these values
    /// obey the `Double` constraint, (row + col) % 2 == 0, then returns that
    /// `Double` in an `Ok`. Otherwise, returns an `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::double::Double;
    ///
    /// let double_1 = Double::force_from_coords(1, 3);
    /// let double_2 = Double::force_from_coords(0, 1);
    ///
    /// assert!(double_1.is_ok());
    ///
    /// assert_eq!(double_1.unwrap().col(), 1);
    /// assert_eq!(double_1.unwrap().row(), 3);
    ///
    /// assert!(double_2.is_err());
    /// ```
    pub fn from_coords(col: i32, row: i32) -> CoordResult<Double> {
        if (col + row) & 1 == 0 {
            Ok(Self { col: col, row: row })
        } else {
            Err("invalid Double coordinate")
        }
    }

    /// Creates a `Double` from two `i32` values.
    ///
    /// # Panics
    ///
    /// Panics if the given `i32` values fail to obey the constraint
    /// (row + col) % 2 == 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::double::Double;
    ///
    /// let double_1 = Double::force_from_coords(1, 3);
    ///
    /// assert_eq!(double_1.col(), 1);
    /// assert_eq!(double_1.row(), 3);
    /// ```
    pub fn force_from_coords(col: i32, row: i32) -> Double {
        Double::from_coords(col, row).unwrap()
    }

    /// Attempts to create a `Double` from a tuple of two `i32` values. If
    /// these values obey the `Double` constraint, (row + col) % 2 == 0, then
    /// returns that `Double` in an `Ok`. Otherwise, returns an `Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::double::Double;
    ///
    /// let double_1 = Double::from_tuple((1, 3));
    /// let double_2 = Double::from_tuple((0, 1));
    ///
    /// assert!(double_1.is_ok());
    ///
    /// assert_eq!(double_1.unwrap().col(), 1);
    /// assert_eq!(double_1.unwrap().row(), 3);
    ///
    /// assert!(double_2.is_err());
    /// ```
    pub fn from_tuple((col, row): (i32, i32)) -> CoordResult<Double> {
        Double::from_coords(col, row)
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
    // Conversion
    //////////////////////////////////

    /// Convert a `Double` to a `Cube`, assuming the grid has a `Tilt::Flat`.
    pub fn flat_to_cube(self) -> Cube {
        let x = self.col;
        let z = (self.row - self.col) / 2;
        let y = 0 - x - z;

        Cube::force_from_coords(x, y, z)
    }

    /// Convert a `Double` to a `Cube`, assuming the grid has a `Tilt::Sharp`.
    pub fn sharp_to_cube(self) -> Cube {
        let x = (self.col - self.row) / 2;
        let z = self.row / 2;
        let y = 0 - x - z;

        Cube::force_from_coords(x, y, z)
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
    pub fn flat_neighbors(self) -> Vec<Self> {
        let offsets = [[1, -1], [1, 1], [0, 2], [-1, 1], [-1, -1], [0, -2]];

        self.offset_map(offsets)
    }

    /// Calculates the `Double` coordinates of the hexes surrounding the
    /// calling instance, in the context of a `HexGrid` whose hexes have the
    /// `Tilt::Sharp` orientation.
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
    /// use chickenwire::coordinate::double::Double;
    ///
    /// let coord_1 = Double::force_from_coords(1, 1);
    /// let coord_2 = Double::force_from_coords(2, 2);
    /// let coord_3 = Double::force_from_coords(4, 2);
    ///
    /// // Determines the distance between any two hexes
    /// assert_eq!(Double::ORIGIN.flat_dist(coord_1), 1);
    /// assert_eq!(coord_1.flat_dist(coord_2), 1);
    /// assert_eq!(coord_1.flat_dist(coord_3), 3);
    ///
    /// // The distance between two identical coordinates is always zero
    /// assert_eq!(coord_1.dist(coord_1), 0);
    ///
    /// // Ordering doesn't matter
    /// assert_eq!(
    ///     coord_1.flat_dist(coord_2),
    ///     coord_2.flat_dist(coord_1)
    /// );
    ///
    /// // Distance may or may not depend upon hex orientation
    /// assert_eq!(
    ///     coord_1.flat_dist(coord_2),
    ///     coord_1.sharp_dist(coord_2)
    /// );
    /// assert_ne!(
    ///     coord_1.flat_dist(coord_3),
    ///     coord_1.sharp_dist(coord_3)
    /// );
    /// ```
    pub fn flat_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        x_dist + cmp::max(0, (y_dist - x_dist) / 2)
    }

    /// Calculates the distance between two `Double` coordinates in the
    /// context of a `HexGrid` whose hexes have the `Tilt::Sharp` orientation.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::double::Double;
    ///
    /// let coord_1 = Double::force_from_coords(1, 1);
    /// let coord_2 = Double::force_from_coords(2, 2);
    /// let coord_3 = Double::force_from_coords(4, 2);
    ///
    /// // Determines the distance between any two hexes
    /// assert_eq!(Double::ORIGIN.sharp_dist(coord_1), 1);
    /// assert_eq!(coord_1.sharp_dist(coord_2), 1);
    /// assert_eq!(coord_1.sharp_dist(coord_3), 2);
    ///
    /// // The distance between two identical coordinates is always zero
    /// assert_eq!(coord_1.dist(coord_1), 0);
    ///
    /// // Ordering doesn't matter
    /// assert_eq!(
    ///     coord_1.sharp_dist(coord_2),
    ///     coord_2.sharp_dist(coord_1)
    /// );
    ///
    /// // Distance may or may not depend upon hex orientation
    /// assert_eq!(
    ///     coord_1.sharp_dist(coord_2),
    ///     coord_1.flat_dist(coord_2)
    /// );
    /// assert_ne!(
    ///     coord_1.sharp_dist(coord_3),
    ///     coord_1.flat_dist(coord_3)
    /// );
    /// ```
    pub fn sharp_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        y_dist + cmp::max(0, (x_dist - y_dist) / 2)
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    //////////////////////////////////
    // Traits: Arithmetic
    //////////////////////////////////

    #[test]
    fn test_add_trait() {
        let d1 = Double::force_from_coords(1, 3);
        let d2 = Double::force_from_coords(0, 2);
        let d3 = Double::force_from_coords(-4, 6);

        assert_eq!(Double::force_from_coords(2, 6), d1 + d1);
        assert_eq!(Double::force_from_coords(1, 5), d1 + d2);
        assert_eq!(Double::force_from_coords(-3, 9), d1 + d3);

        assert_eq!(Double::force_from_coords(0, 4), d2 + d2);
        assert_eq!(Double::force_from_coords(-4, 8), d2 + d3);

        assert_eq!(Double::force_from_coords(-8, 12), d3 + d3);
    }

    #[test]
    fn test_sub_trait() {
        let d1 = Double::force_from_coords(2, 4);
        let d2 = Double::force_from_coords(0, -2);
        let d3 = Double::force_from_coords(3, 7);

        assert_eq!(Double::ORIGIN, d1 - d1);
        assert_eq!(Double::force_from_coords(2, 6), d1 - d2);
        assert_eq!(Double::force_from_coords(-1, -3), d1 - d3);

        assert_eq!(Double::force_from_coords(-2, -6), d2 - d1);
        assert_eq!(Double::ORIGIN, d2 - d2);
        assert_eq!(Double::force_from_coords(-3, -9), d2 - d3);

        assert_eq!(Double::force_from_coords(1, 3), d3 - d1);
        assert_eq!(Double::force_from_coords(3, 9), d3 - d2);
        assert_eq!(Double::ORIGIN, d3 - d3);
    }

    #[test]
    fn test_mul_trait() {
        let d1 = Double::force_from_coords(1, 3);
        let d2 = Double::force_from_coords(-2, 8);
        let d3 = Double::force_from_coords(-3, -1);

        assert_eq!(2 * d1, d1 + d1);
        assert_eq!(0 * d1, Double::ORIGIN);
        assert_eq!(-1 * d1, Double::ORIGIN - d1);

        assert_eq!(5 * d2, d2 + d2 + d2 + d2 + d2);
        assert_eq!(0 * d2, Double::ORIGIN);
        assert_eq!(-3 * d2, Double::ORIGIN - d2 - d2 - d2);

        assert_eq!(1 * d3, d3);
        assert_eq!(14 * d3, Double::force_from_coords(-42, -14));
    }

    //////////////////////////////////
    // Traits: From & Into
    //////////////////////////////////

    #[test]
    fn test_double_from_tuple_trait() {
        assert_eq!(Double::from((0, 0)), Double::ORIGIN);
        assert_eq!(Double::from((3, 1)), Double { col: 3, row: 1 });
        assert_eq!(Double::from((2, 8)), Double { col: 2, row: 8 });
    }

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    #[test]
    fn test_from_coords_ok() {
        let ok_double_1 = Ok(Double { col: 1, row: 3 });
        let ok_double_2 = Ok(Double { col: 2, row: 4 });
        let ok_double_3 = Ok(Double { col: 3, row: 5 });
        let ok_double_4 = Ok(Double { col: 8, row: -4 });
        let ok_double_5 = Ok(Double { col: -2, row: 6 });

        assert_eq!(ok_double_1, Double::from_coords(1, 3));
        assert_eq!(ok_double_2, Double::from_coords(2, 4));
        assert_eq!(ok_double_3, Double::from_coords(3, 5));
        assert_eq!(ok_double_4, Double::from_coords(8, -4));
        assert_eq!(ok_double_5, Double::from_coords(-2, 6));
    }

    #[test]
    fn test_from_coords_err() {
        let err_double_1 = Double::from_coords(0, 1);
        let err_double_2 = Double::from_coords(1, 0);
        let err_double_3 = Double::from_coords(2, 1);
        let err_double_4 = Double::from_coords(-4, 3);
        let err_double_5 = Double::from_coords(5, -6);

        assert!(err_double_1.is_err());
        assert!(err_double_2.is_err());
        assert!(err_double_3.is_err());
        assert!(err_double_4.is_err());
        assert!(err_double_5.is_err());
    }

    #[test]
    fn test_force_from_coords_valid() {
        let double_1 = Double { col: 1, row: 3 };
        let double_2 = Double { col: 2, row: 4 };
        let double_3 = Double { col: 3, row: 5 };
        let double_4 = Double { col: 8, row: -4 };
        let double_5 = Double { col: -2, row: 6 };

        assert_eq!(double_1, Double::force_from_coords(1, 3));
        assert_eq!(double_2, Double::force_from_coords(2, 4));
        assert_eq!(double_3, Double::force_from_coords(3, 5));
        assert_eq!(double_4, Double::force_from_coords(8, -4));
        assert_eq!(double_5, Double::force_from_coords(-2, 6));
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_force_from_coords_invalid_1() {
        let invalid_double = Double::force_from_coords(0, 1);
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_force_from_coords_invalid_2() {
        let invalid_double = Double::force_from_coords(1, 0);
    }

    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_force_from_coords_invalid_3() {
        let invalid_double = Double::force_from_coords(5, 6);
    }

    #[test]
    fn test_from_tuple_ok() {
        let ok_double_1 = Ok(Double { col: 1, row: 3 });
        let ok_double_2 = Ok(Double { col: 2, row: 4 });
        let ok_double_3 = Ok(Double { col: 3, row: 5 });
        let ok_double_4 = Ok(Double { col: 8, row: -4 });
        let ok_double_5 = Ok(Double { col: -2, row: 6 });

        assert_eq!(ok_double_1, Double::from_tuple((1, 3)));
        assert_eq!(ok_double_2, Double::from_tuple((2, 4)));
        assert_eq!(ok_double_3, Double::from_tuple((3, 5)));
        assert_eq!(ok_double_4, Double::from_tuple((8, -4)));
        assert_eq!(ok_double_5, Double::from_tuple((-2, 6)));
    }

    #[test]
    fn test_from_tuple_err() {
        let err_double_1 = Double::from_tuple((0, 1));
        let err_double_2 = Double::from_tuple((1, 0));
        let err_double_3 = Double::from_tuple((2, 1));
        let err_double_4 = Double::from_tuple((-4, 3));
        let err_double_5 = Double::from_tuple((5, -6));

        assert!(err_double_1.is_err());
        assert!(err_double_2.is_err());
        assert!(err_double_3.is_err());
        assert!(err_double_4.is_err());
        assert!(err_double_5.is_err());
    }

    //////////////////////////////////
    // Retrieval
    //////////////////////////////////

    #[test]
    fn test_col() {
        let col_1 = 1;
        let col_2 = 2;

        let double_1 = Double { col: col_1, row: 3 };
        let double_2 = Double { col: col_2, row: 4 };

        assert_eq!(double_1.col(), col_1);
        assert_eq!(double_2.col(), col_2);
    }

    #[test]
    fn test_row() {
        let row_1 = 3;
        let row_2 = 4;

        let double_1 = Double { col: 1, row: row_1 };
        let double_2 = Double { col: 2, row: row_2 };

        assert_eq!(double_1.row(), row_1);
        assert_eq!(double_2.row(), row_2);
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    #[test]
    fn test_flat_neighbors() {
        let exp_origin_neighbors = vec![
            Double { col: 1, row: -1 },
            Double { col: 1, row: 1 },
            Double { col: 0, row: 2 },
            Double { col: -1, row: 1 },
            Double { col: -1, row: -1 },
            Double { col: 0, row: -2 },
        ];

        assert_eq!(
            exp_origin_neighbors,
            Double::ORIGIN.flat_neighbors(),
            "origin neighbors"
        );

        let offset_coord = Double { col: 1, row: 3 };
        let exp_offset_neighbors = vec![
            Double { col: 2, row: 2 },
            Double { col: 2, row: 4 },
            Double { col: 1, row: 5 },
            Double { col: 0, row: 4 },
            Double { col: 0, row: 2 },
            Double { col: 1, row: 1 },
        ];

        assert_eq!(
            exp_offset_neighbors,
            offset_coord.flat_neighbors(),
            "offset neighbors"
        );
    }

    #[test]
    fn test_sharp_neighbors() {
        let exp_origin_neighbors = vec![
            Double { col: 1, row: -1 },
            Double { col: 2, row: 0 },
            Double { col: 1, row: 1 },
            Double { col: -1, row: 1 },
            Double { col: -2, row: 0 },
            Double { col: -1, row: -1 },
        ];

        assert_eq!(
            exp_origin_neighbors,
            Double::ORIGIN.sharp_neighbors(),
            "origin neighbors"
        );

        let offset_coord = Double { col: 3, row: 5 };
        let exp_offset_neighbors = vec![
            Double { col: 4, row: 4 },
            Double { col: 5, row: 5 },
            Double { col: 4, row: 6 },
            Double { col: 2, row: 6 },
            Double { col: 1, row: 5 },
            Double { col: 2, row: 4 },
        ];

        assert_eq!(
            exp_offset_neighbors,
            offset_coord.sharp_neighbors(),
            "offset neighbors"
        );
    }

    //////////////////////////////////
    // Distance
    //////////////////////////////////

    #[test]
    fn test_flat_dist() {
        let coord_1 = Double { col: 1, row: 3 };
        let coord_2 = Double { col: 2, row: 8 };
        let coord_3 = Double { col: 3, row: 5 };
        let coord_4 = Double { col: 4, row: 4 };
        let coord_5 = Double { col: 7, row: 11 };

        assert_eq!(Double::ORIGIN.flat_dist(coord_1), 2);
        assert_eq!(Double::ORIGIN.flat_dist(coord_2), 5);
        assert_eq!(Double::ORIGIN.flat_dist(coord_3), 4);
        assert_eq!(Double::ORIGIN.flat_dist(coord_4), 4);
        assert_eq!(Double::ORIGIN.flat_dist(coord_5), 9);

        assert_eq!(coord_1.flat_dist(coord_1), 0);
        assert_eq!(coord_1.flat_dist(coord_2), 3);
        assert_eq!(coord_1.flat_dist(coord_3), 2);
        assert_eq!(coord_1.flat_dist(coord_4), 3);
        assert_eq!(coord_1.flat_dist(coord_5), 7);

        assert_eq!(coord_5.flat_dist(coord_5), 0);
        assert_eq!(coord_5.flat_dist(coord_4), 5);
        assert_eq!(coord_5.flat_dist(coord_3), 5);
        assert_eq!(coord_5.flat_dist(coord_2), 5);
        assert_eq!(coord_5.flat_dist(coord_1), 7);
    }

    #[test]
    fn test_sharp_dist() {
        let coord_1 = Double { col: 1, row: 3 };
        let coord_2 = Double { col: 4, row: 2 };
        let coord_3 = Double { col: 7, row: 3 };
        let coord_4 = Double { col: 8, row: 0 };
        let coord_5 = Double { col: 12, row: 6 };

        assert_eq!(Double::ORIGIN.sharp_dist(coord_1), 3);
        assert_eq!(Double::ORIGIN.sharp_dist(coord_2), 3);
        assert_eq!(Double::ORIGIN.sharp_dist(coord_3), 5);
        assert_eq!(Double::ORIGIN.sharp_dist(coord_4), 4);
        assert_eq!(Double::ORIGIN.sharp_dist(coord_5), 9);

        assert_eq!(coord_1.sharp_dist(coord_1), 0);
        assert_eq!(coord_1.sharp_dist(coord_2), 2);
        assert_eq!(coord_1.sharp_dist(coord_3), 3);
        assert_eq!(coord_1.sharp_dist(coord_4), 5);
        assert_eq!(coord_1.sharp_dist(coord_5), 7);

        assert_eq!(coord_5.sharp_dist(coord_5), 0);
        assert_eq!(coord_5.sharp_dist(coord_4), 6);
        assert_eq!(coord_5.sharp_dist(coord_3), 4);
        assert_eq!(coord_5.sharp_dist(coord_2), 6);
        assert_eq!(coord_5.sharp_dist(coord_1), 7);
    }
}
