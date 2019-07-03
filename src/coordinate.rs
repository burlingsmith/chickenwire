//! Coordinate Systems
//!
//! # Arithmetic
//! Cube coordinates are treated like vectors in terms of addition/subtraction
//! and scalar multiplication/division.
//!
//! # On Neighbors
//! The exact rule for the ordering of neighbors is that the first position
//! which remains in the same cardinal wedge always receives the zero index,
//! and then indexing procedes clockwise for the remaining neighbors. In
//! practice, this means that the Northeastern neighbor receives the zero
//! index, save for the diagonal case, where the Southeastern neighbor is the
//! recipient.

use std::cmp;
use std::ops::{Add, Div, Mul, Sub};

// add a higher-level compass that abstracts this away
#[derive(Debug)]
enum Compass {
    North,        // 0 degrees clockwise
    NortheastA,   // 30 degrees clockwise
    NortheastB,   // 60 degrees clockwise
    East,         // 90 degrees clockwise
    SoutheastA,   // 120 degrees clockwise
    SoutheastB,   // 150 degrees clockwise
    South,        // 180 degrees clockwise
    SouthwestA,   // 210 degrees clockwise
    SouthwestB,   // 240 degrees clockwise
    West,         // 270 degrees clockwise
    NorthwestA,   // 300 degrees clockwise
    NorthwestB,   // 330 degrees clockwise
}

impl Compass {
    fn flat_index(self) -> usize {
        match self {
            // Edges/Neighbors
            NortheastB => 0,
            SoutheastA => 1,
            South => 2,
            SouthwestB => 3,
            NorthwestA => 4,
            North => 5,

            // Vertices/Diagonals
            SoutheastB => 0,
            SouthwestA => 1,
            West => 2,
            NorthwestB => 3,
            NortheastA => 4,
            East => 5,
        }
    }

    fn sharp_index(self) -> usize {
        match self {
            // Edges/Neighbors
            NortheastA => 0,
            East => 1,
            SouthwestA => 2,
            SoutheastB => 3,
            West => 4,
            NorthwestB => 5,

            // Vertices/Diagonals
            SoutheastA => 0,
            South => 1,
            SouthwestB => 2,
            NorthwestA => 3,
            North => 4,
            NortheastB => 5,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Cube Coordinate System
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

/// For axial coordinate (q, r), cube coordinate (x, y, z) is calculated by
/// solving for y based upon the constraint x + y + z == 0, where x == q and
/// z == r.
///
/// # Examples
///
/// ```
/// let axial = Axial { q: 1, r: 2 };
///
/// assert_eq!(Cube::from(axial), Cube { x: 1, y: -3, z: 2});
/// assert_eq!(Cube::from(Axial::origin()), Cube::origin());
/// ```
impl From<Axial> for Cube {
    fn from(coord: Axial) -> Self {
        let x = coord.q;
        let z = coord.r;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }
}

/// Cube coordinates are added together like vectors.
///
/// # Examples
///
/// ```
/// let coord_1 = Cube { x: 1, y: 2, z: -3 };
/// let coord_2 = Cube { x: -5, y: -7, z: 12 };
///
/// assert_eq!(coord_1 + coord_2, Cube { x: -4, y: -5, z: 9 });
/// assert_eq!(coord_2 + coord_1, Cube { x: -4, y: -5, z: 9 });
/// ```
impl Add for Cube {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Cube coordinates are subtracted from each other like vectors.
///
/// # Examples
///
/// ```
/// let coord_1 = Cube { x: 1, y: 2, z: 3 };
/// let coord_2 = Cube { x: 5, y: 7, z: 11 };
///
/// assert_eq!(coord_1 - coord_2, Cube { x: -4, y: -5, z: -8 });
/// assert_eq!(coord_2 - coord_1, Cube { x: 4, y: 5, z: 8 });
/// ```
impl Sub for Cube {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Cube coordinates can be multiplied by `i32` scalars, like vectors.
///
/// # Examples
///
/// ```
/// let coord = Cube { x: 1, y: 2, z: -3 };
///
/// assert_eq!(-1 * coord, Cube { x: -1, y: -4, z: 6});
/// assert_eq!(0 * coord, Cube::origin());
/// assert_eq!(2 * coord, Cube { x: 2, y: 4, z: -6 });
/// ```
impl Mul<i32> for Cube {
    type Output = Self;

    fn mul(self, n: i32) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}

/// Cube coordinates can be divided by `i32` scalars, like vectors. Values are
/// rounded toward zero, truncating the fractional component.
///
/// # Panics
///
/// Panics when trying to divide by zero.
///
/// # Examples
///
/// ```
/// let coord = Cube { x: 3, y: 4, z: -6 };
///
/// assert_eq!(coord / -1, Cube { x: -3, y: -4, z: 6});
/// assert_eq!(coord / 2, Cube::origin());
/// assert_eq!(2 * coord, Cube { x: 2, y: 4, z: -6 });
/// ```
impl Div<i32> for Cube {
    type Output = Self;

    fn div(self, n: i32) -> Self {
        Self {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl Cube {
    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    fn origin() -> Self {
        Cube { x: 0, y: 0, z: 0 }
    }

    //////////////////////////////////
    // Boolean Analysis
    //////////////////////////////////

    fn valid(self) -> bool {
        self.x + self.y + self.z == 0
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    fn offset_map(self, offsets: Vec<(i32, i32, i32)>) -> Vec<Self> {
        offsets
        .into_iter()
        .map(|(x, y, z)| Cube { x: self.x + x, y: self.y + y, z: self.z + z })
        .collect()
    }

    fn neighbors(self) -> Vec<Self> {
        let mut offsets = vec![
            (1, 0, -1),     // NE
            (1, -1, 0),
            (0, -1, 1),
            (-1, 0, 1),     // SW
            (-1, 1, 0),
            (0, 1, -1),
        ];

        self.offset_map(offsets)
    }

    fn diagonals(self) -> Vec<Self> {
        let mut offsets = vec![
            (1, 1, -2),     // SE
            (-1, 2, -1),
            (-2, 1, 1),
            (-1, -1, 2),    // NW
            (1, -2, 1),
            (2, -1, -1),
        ];

        self.offset_map(offsets)
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    /// Determines the distance between two cube coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let origin = Cube::origin();
    /// let coord_1 = Cube { x: 1, y: 2, z: -3 };
    /// let coord_2 = Cube { x: -8, y: 6, z: 2 };
    ///
    /// assert_eq!(origin.dist(coord_1), 3);
    /// assert_eq!(coord_1.dist(origin), 3);
    /// assert_eq!(coord_1.dist(coord_1), 0);
    /// assert_eq!(coord_2.dist(coord_1), 9);
    /// ```
    fn dist(self, other: Self) -> i32 {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        let z_dist = (self.z - other.z).abs();

        cmp::max(cmp::max(x_dist, y_dist), z_dist)
    }

    //////////////////////////////////
    // Rotation
    //////////////////////////////////

    fn rotate_cw(self, point: Self, num_turns: u32) -> Self {
        let mut vector = point - self;

        for turns in 0..(num_turns % 6) {
            let new_x = 0 - vector.z;
            let new_y = 0 - vector.x;
            let new_z = 0 - vector.y;

            vector = Cube { x: new_x, y: new_y, z: new_z };
        }

        vector + self
    }

    fn rotate_cc(self, point: Self, num_turns: u32) -> Self {
        let mut vector = point - self;

        for turns in 0..(num_turns % 6) {
            let new_x = 0 - vector.y;
            let new_y = 0 - vector.z;
            let new_z = 0 - vector.x;

            vector = Cube { x: new_x, y: new_y, z: new_z };
        }

        vector + self
    }

    //////////////////////////////////
    // Rings
    //////////////////////////////////

    fn ring(self, radius: u32) -> Vec<Self> {
        if radius == 0 {
            vec![self]
        } else {
            unimplemented!();
        }
    }

    fn spiral(self, radius: u32) -> Vec<Self> {
        let mut coords = Vec::new();

        for r in 0..=radius {
            coords.append(&mut self.ring(r));
        }

        coords
    }
}

//////////////////////////////////////////////////////////////////////////////
// Axial Coordinate System
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Axial {
    q: i32,
    r: i32,
}

impl From<Cube> for Axial {
    fn from(coord: Cube) -> Self {
        Axial { q: coord.x, r: coord.z }
    }
}

impl Axial {
    //////////////////////////////////
    // Distances
    //////////////////////////////////

    fn dist(self, other: Self) -> i32 {
        Cube::from(self).dist(Cube::from(other))
    }
}

//////////////////////////////////////////////////////////////////////////////
// Offset Coordinate System
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Offset {
    col: i32,
    row: i32,
}

impl Offset {
    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    fn oflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col - (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    fn eflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col + (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    fn osharp_to_cube(self) -> Cube {
        let x = self.col - (self.row - (self.row & 1)) / 2;
        let z = self.row;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    fn esharp_to_cube(self) -> Cube {
        let x = self.col - (self.row + (self.row & 1)) / 2;
        let z = self.row;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
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

    fn oflat_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, -1], [1, 0], [0, 1], [-1, 0], [-1, -1], [0, -1]],
            [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [0, -1]],
        ];

        self.offset_map(offsets, self.col)
    }

    fn eflat_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [0, -1]],
            [[1, -1], [1, 0], [0, 1], [-1, 0], [-1, -1], [0, -1]],
        ];

        self.offset_map(offsets, self.col)
    }

    fn osharp_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[0, -1], [1, 0], [0, 1], [-1, 1], [-1, 0], [-1, -1]],
            [[1, -1], [1, 0], [1, 1], [0, 1], [-1, 0], [0, -1]],
        ];

        self.offset_map(offsets, self.row)
    }

    fn esharp_neighbors(self) -> Vec<Self> {
        let offsets = [
            [[1, -1], [1, 0], [1, 1], [0, 1], [-1, 0], [0, -1]],
            [[0, -1], [1, 0], [0, 1], [-1, 1], [-1, 0], [-1, -1]],
        ];

        self.offset_map(offsets, self.row)
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    fn dist(self, other: Self) -> i32 {
        self.eflat_to_cube().dist(other.eflat_to_cube())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Interlaced (Double) Coordinate System
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Double {
    col: i32,
    row: i32,
}

impl Double {
    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    fn flat_to_cube(self) -> Cube {
        let x = self.col;
        let z = (self.row - self.col) / 2;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    fn sharp_to_cube(self) -> Cube {
        let x = (self.col - self.row) / 2;
        let z = self.row / 2;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    fn offset_map(self, offsets: [[i32; 2]; 6]) -> Vec<Self> {
        let mut neighbors = Vec::new();

        for side in 0..6 {
            let offset_pair = offsets[side];

            let col = offset_pair[0] + self.col;
            let row = offset_pair[1] + self.row;

            neighbors.push(Double { col: col, row: row });
        }

        neighbors
    }

    fn flat_neighbors(self) -> Vec<Self> {
        let offsets = [[1, -1], [1, 1], [0, 2], [-1, 1], [-1, -1], [0, -2]];

        self.offset_map(offsets)
    }

    fn sharp_neighbors(self) -> Vec<Self> {
        let offsets = [[1, -1], [2, 0], [1, 1], [-1, 1], [-2, 0], [-1, -1]];

        self.offset_map(offsets)
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    fn flat_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        y_dist + cmp::max(0, (x_dist - y_dist) / 2)
    }

    fn sharp_dist(self, other: Self) -> i32 {
        let x_dist = (self.col - other.col).abs();
        let y_dist = (self.row - other.row).abs();

        x_dist + cmp::max(0, (y_dist - x_dist) / 2)
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_cube_arithmetic() {
        assert_eq!(2 + 2, 4);
    }
}
