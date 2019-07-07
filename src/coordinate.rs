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

//////////////////////////////////////////////////////////////////////////////
// Compass Position Notation
//////////////////////////////////////////////////////////////////////////////

// add a higher-level compass that abstracts this away
#[derive(Debug)]
pub enum Compass {
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
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

/// For three-element tuple of unsigned 32-bit integers (a, b, c), the
/// corresponding cube coordinate (x, y, z) is calculated by solving for z
/// based upon the constraint x + y + z == 0, where x == a and y == b. This
/// method ensures the production of valid cube coordinates.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Cube;
///
/// let valid_cube_tuple = (1, 2, -3);
/// let invalid_cube_tuple = (1, 2, 10);
///
/// assert_eq!(Cube::from(valid_cube_tuple), Cube::from(invalid_cube_tuple));
/// ```
impl From<(i32, i32, i32)> for Cube {
    fn from((x, y, _): (i32, i32, i32)) -> Self {
        Self { x: x, y: y, z: 0 - x - y }
    }
}

/// For axial coordinate (q, r), cube coordinate (x, y, z) is calculated by
/// solving for y based upon the constraint x + y + z == 0, where x == q and
/// z == r.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::{Cube, Axial};
///
/// let axial = Axial::from_coords(1, 2);
///
/// assert_eq!(Cube::from(axial), Cube::from_coords(1, -3, 2));
/// assert_eq!(Cube::from(Axial::origin()), Cube::origin());
/// ```
impl From<Axial> for Cube {
    fn from(coord: Axial) -> Self {
        let x = coord.q;
        let z = coord.r;
        let y = 0 - x - z;

        Self { x: x, y: y, z: z }
    }
}

/// Cube coordinates are added together like vectors.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Cube;
///
/// let coord_1 = Cube::from_coords(1, 2, -3);
/// let coord_2 = Cube::from_coords(-5, -7, 12);
///
/// assert_eq!(coord_1 + coord_2, Cube::from_coords(-4, -5, 9));
/// assert_eq!(coord_2 + coord_1, Cube::from_coords(-4, -5, 9));
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
/// use chickenwire::coordinate::Cube;
///
/// let coord_1 = Cube::from_coords(1, 2, -3);
/// let coord_2 = Cube::from_coords(5, 7, -12);
///
/// assert_eq!(coord_1 - coord_2, Cube::from_coords(-4, -5, 9));
/// assert_eq!(coord_2 - coord_1, Cube::from_coords(4, 5, -9));
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
/// use chickenwire::coordinate::Cube;
///
/// let coord = Cube::from_coords(1, 2, -3);
///
/// assert_eq!(-1 * coord, Cube::from_coords(-1, -2, 3));
/// assert_eq!(0 * coord, Cube::origin());
/// assert_eq!(2 * coord, Cube::from_coords(2, 4, -6));
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

/// Cube coordinates can be multiplied by `i32` scalars, like vectors.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Cube;
///
/// let coord = Cube::from_coords(1, 2, -3);
///
/// assert_eq!(coord * (-1), Cube::from_coords(-1, -2, 4));
/// assert_eq!(coord * 0, Cube::origin());
/// assert_eq!(coord * 2, Cube::from_coords(2, 4, -6));
/// ```
impl Mul<Cube> for i32 {
    type Output = Cube;

    fn mul(self, coord: Cube) -> Cube {
        coord * self
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
/// use chickenwire::coordinate::Cube;
///
/// let coord = Cube::from_coords(12, 24, -36);
///
/// assert_eq!(coord / -1, Cube::from_coords(-12, -24, 36));
/// assert_eq!(coord / 2, Cube::from_coords(6, 12, -18));
/// assert_eq!(coord / 3, Cube::from_coords(4, 8, -12));
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
    // Constants
    //////////////////////////////////

    const NEIGHBOR_OFFSETS: [(i32, i32, i32); 6] = [
        (1, 0, -1),     // NE
        (1, -1, 0),
        (0, -1, 1),
        (-1, 0, 1),     // SW
        (-1, 1, 0),
        (0, 1, -1),
    ];

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    pub fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    /// For three unsigned 32-bit integers a, b, and c, the corresponding cube
    /// coordinate (x, y, z) is calculated by solving for z based upon the
    /// constraint x + y + z == 0, where x == a and y == b. This method
    /// ensures the production of valid cube coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Cube;
    ///
    /// assert_eq!(Cube::from_coords(1, 2, -3), Cube::from((1, 2, -3)));
    ///
    /// assert_eq!(Cube::from_coords(1, 2, -3), Cube::from_coords(1, 2, 99));
    /// assert_ne!(Cube::from_coords(1, 2, -3), Cube::from_coords(0, 2, -3));
    /// ```
    pub fn from_coords(x: i32, y: i32, z: i32) -> Self {
        Self::from((x, y, z))
    }

    //////////////////////////////////
    // Setters
    //////////////////////////////////

    pub fn set_coords(&mut self, x: i32, y: i32, z: i32) {
        self.x = x;
        self.y = y;
        self.z = 0 - x - y;
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    fn offset_map(self, offsets: Vec<(i32, i32, i32)>) -> Vec<Self> {
        offsets
        .into_iter()
        .map(|coord_tuple| self + Self::from(coord_tuple))
        .collect()
    }

    pub fn neighbors(self) -> Vec<Self> {
        self.offset_map(Self::NEIGHBOR_OFFSETS.to_vec())
    }

    pub fn diagonals(self) -> Vec<Self> {
        let diagonal_offsets = vec![
            (1, 1, -2),     // SE
            (-1, 2, -1),
            (-2, 1, 1),
            (-1, -1, 2),    // NW
            (1, -2, 1),
            (2, -1, -1),
        ];

        self.offset_map(diagonal_offsets)
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    /// Determines the distance between two cube coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Cube;
    ///
    /// let origin = Cube::origin();
    /// let coord_1 = Cube::from_coords(1, 2, -3);
    /// let coord_2 = Cube::from_coords(-8, 6, 2);
    ///
    /// assert_eq!(origin.dist(coord_1), 3);
    /// assert_eq!(coord_1.dist(origin), 3);
    /// assert_eq!(coord_1.dist(coord_1), 0);
    /// assert_eq!(coord_2.dist(coord_1), 9);
    /// ```
    pub fn dist(self, other: Self) -> i32 {
        let x_dist = (self.x - other.x).abs();
        let y_dist = (self.y - other.y).abs();
        let z_dist = (self.z - other.z).abs();

        cmp::max(cmp::max(x_dist, y_dist), z_dist)
    }

    //////////////////////////////////
    // Rotation
    //////////////////////////////////

    pub fn rotate_cw(self, point: Self, num_turns: u32) -> Self {
        let mut vector = point - self;

        for turns in 0..(num_turns % 6) {
            let new_x = 0 - vector.z;
            let new_y = 0 - vector.x;
            let new_z = 0 - vector.y;

            vector.set_coords(new_x, new_y, new_z);
        }

        vector + self
    }

    pub fn rotate_cc(self, point: Self, num_turns: u32) -> Self {
        let mut vector = point - self;

        for turns in 0..(num_turns % 6) {
            let new_x = 0 - vector.y;
            let new_y = 0 - vector.z;
            let new_z = 0 - vector.x;

            vector = Self::from_coords(new_x, new_y, new_z);
        }

        vector + self
    }

    //////////////////////////////////
    // Rings
    //////////////////////////////////

    pub fn ring(self, radius: u32) -> Vec<Self> {
        let mut coords = Vec::new();

        if radius == 0 {
            coords = vec![self];
        } else {
            let dir_vector = Cube::from(Cube::NEIGHBOR_OFFSETS[0]);
            let mut next_cube = self + (radius as i32) * dir_vector;

            for side in 0..6 {
                for coord in 0..radius {
                    coords.push(next_cube);
                    next_cube = next_cube.neighbors()[side];
                }
            }
        }

        coords
    }

    pub fn spiral(self, radius: u32) -> Vec<Self> {
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
pub struct Axial {
    q: i32,
    r: i32,
}

impl From<(i32, i32)> for Axial {
    fn from((q, r): (i32, i32)) -> Self {
        Self { q: q, r: r }
    }
}

impl From<Cube> for Axial {
    fn from(coord: Cube) -> Self {
        Self { q: coord.x, r: coord.z }
    }
}

impl Axial {
    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    pub fn origin() -> Self {
        Self { q: 0, r: 0 }
    }

    /// For two unsigned 32-bit integers x and y, the corresponding axial
    /// coordinate is (x, y).
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Axial;
    ///
    /// assert_eq!(Axial::from((1, 2)), Axial::from_coords(1, 2));
    /// ```
    pub fn from_coords(q: i32, r: i32) -> Self {
        Self::from((q, r))
    }

    //////////////////////////////////
    // Distances
    //////////////////////////////////

    pub fn dist(self, other: Self) -> i32 {
        Cube::from(self).dist(Cube::from(other))
    }
}

//////////////////////////////////////////////////////////////////////////////
// Offset Coordinate System
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Offset {
    col: i32,
    row: i32,
}

impl Offset {
    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    pub fn oflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col - (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    pub fn eflat_to_cube(self) -> Cube {
        let x = self.col;
        let z = self.row - (self.col + (self.col & 1)) / 2;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    pub fn osharp_to_cube(self) -> Cube {
        let x = self.col - (self.row - (self.row & 1)) / 2;
        let z = self.row;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    pub fn esharp_to_cube(self) -> Cube {
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
// Interlaced (Double) Coordinate System
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Double {
    col: i32,
    row: i32,
}

impl Double {
    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    pub fn flat_to_cube(self) -> Cube {
        let x = self.col;
        let z = (self.row - self.col) / 2;
        let y = 0 - x - z;

        Cube { x: x, y: y, z: z }
    }

    pub fn sharp_to_cube(self) -> Cube {
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

#[cfg(test)]
mod tests {
    use super::*;

    //////////////////////////////////
    // General
    //////////////////////////////////

    #[test]
    fn it_works_1() {
        assert_eq!(2 + 2, 4);
    }

    //////////////////////////////////
    // Cube Coordinates
    //////////////////////////////////

    #[test]
    fn test_cube_initialization() {
        unimplemented!();
    }

    #[test]
    fn test_cube_from_tuple() {
        assert_eq!(Cube::origin(), Cube::from(0, 0, 0));
        assert_eq!(Cube { x: 1, y: 2, z: -3 }, Cube::from(1, 2, -3));
        assert_eq!(Cube { x: -3, y: -4, z: 7 }, Cube::from(-3, -4, 7));
        assert_eq!(Cube { x: -5, y: 6, z: -1 }, Cube::from(-5, 6, -1));
        assert_eq!(Cube { x: 7, y: -8, z: 1 }, Cube::from(7, -8, 1));
        assert_eq!(Cube { x: 1, y: 2, z: -3 }, Cube::from(1, 2, 0));
    }

    #[test]
    fn test_cube_from_axial() {
        unimplemented!();
    }

    #[test]
    fn test_cube_arithmetic() {
        unimplemented!();
    }

    //////////////////////////////////
    // Axial Coordinates
    //////////////////////////////////

    #[test]
    fn it_works_3() {
        assert_eq!(2 + 2, 4);
    }

    //////////////////////////////////
    // Offset Coordinates
    //////////////////////////////////

    #[test]
    fn it_works_4() {
        assert_eq!(2 + 2, 4);
    }

    //////////////////////////////////
    // Interlaced Coordinates
    //////////////////////////////////

    #[test]
    fn it_works_5() {
        assert_eq!(2 + 2, 4);
    }
}
