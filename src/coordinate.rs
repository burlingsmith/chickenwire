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
// MultiCoordinates
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MultiCoordinate {
    Cube(i32, i32, i32),
    Axial(i32, i32),
    Offset(i32, i32),
    Double(i32, i32),
}

impl From<Cube> for MultiCoordinate {
    fn from(coord: Cube { x: x, y: y, z: z }) -> Self {
        MultiCoordinate::Cube(x, y, z)
    }
}

impl From<MultiCoordinate> for Cube {
    fn from(coord: MultiCoordinate) -> Self {
        match coord {
            Cube(x, y, z) => Cube { x: x, y: y, z: z },
            _ => panic!("{:?} is not a Cube coordinate", coord);,
        }
    }
}

impl From<Axial> for MultiCoordinate {
    fn from(coord: Axial { q: q, r: r }) -> Self {
        MultiCoordinate::Axial(q, r)
    }
}

impl From<MultiCoordinate> for Axial {
    fn from(coord: MultiCoordinate) -> Self {
        match coord {
            Axial(q, r) => Axial { q: q, r: r },
            _ => panic!("{:?} is not an Axial coordinate", coord);,
        }
    }
}

impl From<Offset> for MultiCoordinate {
    fn from(coord: Offset { col: c, row: r }) -> Self {
        MultiCoordinate::Offset(c, r)
    }
}

impl From<MultiCoordinate> for Offset {
    fn from(coord: MultiCoordinate) -> Self {
        match coord {
            Offset(c, r) => Offset { col: c, row: r },
            _ => panic!("{:?} is not an Offset coordinate", coord);,
        }
    }
}

impl From<Double> for MultiCoordinate {
    fn from(coord: Double { col: c, row: r }) -> Self {
        MultiCoordinate::Double(c, r)
    }
}

impl From<MultiCoordinate> for Double {
    fn from(coord: MultiCoordinate) -> Self {
        match coord {
            Double(c, r) => Double { col: c, row: r },
            _ => panic!("{:?} is not a Double coordinate", coord);,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Cube Coordinate System
//////////////////////////////////////////////////////////////////////////////

/// Cube coordinates operate on three planes, treating hexes as cross-sections
/// of a cube sliced along the diagonal.
///
/// Cube coordinates possess the constraint that, for cube coorinate
/// (x, y, z), x + y + z == 0. This is automatically enforced in methods for
/// `Cube` and associated functions. As part of this enforcement, `Cube` is
/// opaque, requiring method or function calls for instantiation and
/// modification.
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
/// assert_eq!(Cube::from(Axial::ORIGIN), Cube::ORIGIN);
/// assert_eq!(Cube::from(axial), Cube::from_coords(1, -3, 2));
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
/// assert_eq!(0 * coord, Cube::ORIGIN);
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
/// assert_eq!(coord * 0, Cube::ORIGIN);
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

    /// Cube coordinate origin of (0, 0, 0).
    pub const ORIGIN: Cube = Cube { x: 0, y: 0, z: 0 };

    /// Offset values for cube coordinate neighbors, beginning with the
    /// Northeastern side and proceeding clockwise. Can be tthought of as unit
    /// vectors for the directions of the six sides.
    const NEIGHBOR_OFFSETS: [(i32, i32, i32); 6] = [
        (1, 0, -1),     // NE
        (1, -1, 0),
        (0, -1, 1),
        (-1, 0, 1),     // SW
        (-1, 1, 0),
        (0, 1, -1),
    ];

    /// Offset values for cube coordinate diagonals, beginning with the
    /// Southeastern side and proceeding clockwise.
    const DIAGONAL_OFFSETS: [(i32, i32, i32); 6] = [
        (1, -2, 1),     // SE
        (-1, -1, 2),
        (-2, 1, 1),
        (-1, 2, -1),    // NW
        (1, 1, -2),
        (2, -1, -1),
    ];

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

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

    /// Change a coordinate's contents. Works like `from_coords` but without
    /// creating a new instance. Coordinate validity is enforced
    /// automatically.
    pub fn set_coords(&mut self, x: i32, y: i32, _z: i32) {
        self.x = x;
        self.y = y;
        self.z = 0 - x - y;
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    /// Calculates the requested neighbor. An index of 0 returns the
    /// Northeastern neighbor, and increases clockwise. Indices wrap around.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Cube;
    ///
    /// assert_eq!(Cube::from_coords(1, 0, -1), Cube::ORIGIN.neighbor(0));
    /// assert_eq!(Cube::from_coords(1, -1, 0), Cube::ORIGIN.neighbor(1));
    /// assert_eq!(Cube::from_coords(0, 1, -1), Cube::ORIGIN.neighbor(5));
    ///
    /// assert_eq!(Cube::ORIGIN.neighbor(0), Cube::ORIGIN.neighbor(6));
    /// assert_eq!(Cube::ORIGIN.neighbor(1), Cube::ORIGIN.neighbor(7));
    /// ```
    pub fn neighbor(self, index: usize) -> Self {
        self + Self::from(Self::NEIGHBOR_OFFSETS[index % 6])
    }

    /// Produces a `Vec<Cube>` ordered beginning with the Northeastern
    /// neighbor of the calling instance and proceeding clockwise.
    ///
    /// The Northeastern coordinate was chosen as the anchor because it is the
    /// first diagonal (when proceeding clockwise) which visually remains in
    /// the same compass direction from the calling instance. This logic is
    /// consistent throughout the module.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Cube;
    ///
    /// let origin_neighbors = vec![
    ///     Cube::from_coords(1, 0, -1),
    ///     Cube::from_coords(1, -1, 0),
    ///     Cube::from_coords(0, -1, 1),
    ///     Cube::from_coords(-1, 0, 1),
    ///     Cube::from_coords(-1, 1, 0),
    ///     Cube::from_coords(0, 1, -1),
    /// ];
    ///
    /// assert_eq!(origin_neighbors, Cube::ORIGIN.neighbors());
    ///
    /// let offset_neighbors = vec![
    ///     Cube::from_coords(2, 2, -4),
    ///     Cube::from_coords(2, 1, -3),
    ///     Cube::from_coords(1, 1, -2),
    ///     Cube::from_coords(0, 2, -2),
    ///     Cube::from_coords(0, 3, -3),
    ///     Cube::from_coords(1, 3, -4),
    /// ];
    ///
    /// assert_eq!(offset_neighbors, Cube::from_coords(1, 2, -3).neighbors());
    /// ```
    pub fn neighbors(self) -> Vec<Self> {
        let mut coords = Vec::new();

        for index in  0..6 {
            coords.push(self.neighbor(index));
        }

        coords
    }

    /// Calculates the requested diagonal. An index of 0 returns the
    /// Southeastern diagonal, and increases clockwise. Indices wrap around.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Cube;
    ///
    /// assert_eq!(Cube::from_coords(1, -2, 1), Cube::ORIGIN.diagonal(0));
    /// assert_eq!(Cube::from_coords(-1, -1, 2), Cube::ORIGIN.diagonal(1));
    /// assert_eq!(Cube::from_coords(2, -1, -1), Cube::ORIGIN.diagonal(5));
    ///
    /// assert_eq!(Cube::ORIGIN.diagonal(0), Cube::ORIGIN.diagonal(6));
    /// assert_eq!(Cube::ORIGIN.diagonal(1), Cube::ORIGIN.diagonal(7));
    /// ```
    pub fn diagonal(self, index: usize) -> Self {
        self + Self::from(Self::DIAGONAL_OFFSETS[index % 6])
    }

    /// Produces a `Vec<Cube>` ordered beginning with the Southeastern
    /// diagonal of the calling instance and proceeding clockwise.
    ///
    /// The Southeastern coordinate was chosen as the anchor because it is the
    /// first diagonal (when proceeding clockwise) which visually remains in
    /// the same compass direction from the calling instance. This logic is
    /// consistent throughout the module.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Cube;
    ///
    /// let origin_diagonals = vec![
    ///     Cube::from_coords(1, -2, 1),
    ///     Cube::from_coords(-1, -1, 2),
    ///     Cube::from_coords(-2, 1, 1),
    ///     Cube::from_coords(-1, 2, -1),
    ///     Cube::from_coords(1, 1, -2),
    ///     Cube::from_coords(2, -1, -1),
    /// ];
    ///
    /// assert_eq!(origin_diagonals, Cube::ORIGIN.diagonals());
    ///
    /// let offset_diagonals = vec![
    ///     Cube::from_coords(2, 0, -2),
    ///     Cube::from_coords(0, 1, -1),
    ///     Cube::from_coords(-1, 3, -2),
    ///     Cube::from_coords(0, 4, -4),
    ///     Cube::from_coords(2, 3, -5),
    ///     Cube::from_coords(3, 1, -4),
    /// ];
    ///
    /// assert_eq!(offset_diagonals, Cube::from_coords(1, 2, -3).diagonals());
    /// ```
    pub fn diagonals(self) -> Vec<Self> {
        let mut coords = Vec::new();

        for index in  0..6 {
            coords.push(self.diagonal(index));
        }

        coords
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
    /// let origin = Cube::ORIGIN;
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

        for _turns in 0..(num_turns % 6) {
            let new_x = 0 - vector.z;
            let new_y = 0 - vector.x;
            let new_z = 0 - vector.y;

            vector.set_coords(new_x, new_y, new_z);
        }

        vector + self
    }

    pub fn rotate_cc(self, point: Self, num_turns: u32) -> Self {
        let mut vector = point - self;

        for _turns in 0..(num_turns % 6) {
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
            let init_index = 2;

            for side in 0..6 {
                let mut next_coord: Cube =
                    (radius as i32) * Cube::from(Cube::NEIGHBOR_OFFSETS[side])
                    + self;
                let side_dir = (side + init_index) % 6;

                for _coord in 0..radius {
                    coords.push(next_coord);
                    next_coord = next_coord.neighbors()[side_dir];
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

/// Axial coordinates use the same system as cube coordinates, but only store
/// two of the coordinate values. This is possible since, for cube coordinate
/// (x, y, z), the third value can always be calculated when the other two are
/// known due to the constraint x + y + z == 0.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Axial {
    pub q: i32,
    pub r: i32,
}

/// For two-element tuple of unsigned 32-bit integers (a, b), the
/// corresponding axial coordinate is (q, r), where q == a and r == b.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Axial;
///
/// assert_eq!(Axial::from((1, 2)), Axial { q: 1, r: 2 });
/// ```
impl From<(i32, i32)> for Axial {
    fn from((q, r): (i32, i32)) -> Self {
        Self { q: q, r: r }
    }
}

/// For cube coordinate (x, y, z), the corresponding axial coordinate is
/// (q, r), where q == x and r == z.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::{Cube, Axial};
///
/// let cube = Cube::from_coords(1, 2, -3);
///
/// assert_eq!(Axial::from(Cube::ORIGIN), Axial::ORIGIN);
/// assert_eq!(Axial::from(cube), Axial::from_coords(1, -3));
/// ```
impl From<Cube> for Axial {
    fn from(coord: Cube) -> Self {
        Self { q: coord.x, r: coord.z }
    }
}

impl Axial {
    //////////////////////////////////
    // Constants
    //////////////////////////////////

    /// Axial coordinate origin of (0, 0).
    pub const ORIGIN: Axial = Axial { q: 0, r: 0 };

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

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

    /// Determines the distance between two axial coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::coordinate::Axial;
    ///
    /// let origin = Axial::ORIGIN;
    /// let coord_1 = Axial { q: 1, r: -3 };
    /// let coord_2 = Axial { q: -8, r: 2 };
    ///
    /// assert_eq!(origin.dist(coord_1), 3);
    /// assert_eq!(coord_1.dist(origin), 3);
    /// assert_eq!(coord_1.dist(coord_1), 0);
    /// assert_eq!(coord_2.dist(coord_1), 9);
    /// ```
    pub fn dist(self, other: Self) -> i32 {
        Cube::from(self).dist(Cube::from(other))
    }
}

//////////////////////////////////////////////////////////////////////////////
// Offset Coordinate System
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Offset {
    pub col: i32,
    pub row: i32,
}

impl Offset {
    //////////////////////////////////
    // Constants
    //////////////////////////////////

    /// Offset coordinate origin of (0, 0).
    pub const ORIGIN: Offset = Offset { col: 0, row: 0 };

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
    fn test_cube_from_tuples() {
        assert_eq!(Cube::ORIGIN, Cube::from((0, 0, 0)));
        assert_eq!(Cube { x: 1, y: 2, z: -3 }, Cube::from((1, 2, -3)));
        assert_eq!(Cube { x: -3, y: -4, z: 7 }, Cube::from((-3, -4, 7)));
        assert_eq!(Cube { x: -5, y: 6, z: -1 }, Cube::from((-5, 6, -1)));
        assert_eq!(Cube { x: 7, y: -8, z: 1 }, Cube::from((7, -8, 1)));
        assert_eq!(Cube { x: 1, y: 2, z: -3 }, Cube::from((1, 2, 0)));
    }

    #[test]
    fn test_cube_from_integers() {
        assert_eq!(
            Cube::ORIGIN,
            Cube::from_coords(0, 0, 0)
        );
        assert_eq!(
            Cube { x: -1, y: 4, z: -3 },
            Cube::from_coords(-1, 4, -3)
        );
        assert_eq!(
            Cube { x: 7, y: -12, z: 5 },
            Cube::from_coords(7, -12, 5)
        );
        assert_eq!(
            Cube { x: -17, y: 10, z: 7 },
            Cube::from_coords(-17, 10, 7)
        );
        assert_eq!(
            Cube { x: 7, y: -10, z: 3 },
            Cube::from_coords(7, -10, 3)
        );
        assert_eq!(
            Cube { x: -4, y: -8, z: 12 },
            Cube::from_coords(-4, -8, 12)
        );
    }

    #[test]
    fn test_cube_from_axial() {
        assert_eq!(
            Cube::ORIGIN,
            Cube::from(Axial::ORIGIN),
            "Cube::ORIGIN != Axial::ORIGIN"
        );
        assert_eq!(
            Cube::from_coords(1, 2, -3),
            Cube::from(Axial::from_coords(1, -3)),
            "Cube(1, 2, -3) != Axial(1, -3)"
        );
        assert_eq!(
            Cube::from_coords(7, -11, 4),
            Cube::from(Axial::from_coords(7, 4)),
            "Cube(7, -11, 4) != Axial(7, 4)"
        );
        assert_eq!(
            Cube::from_coords(-11, 23, -12),
            Cube::from(Axial::from_coords(-11, -12)),
            "Cube(-11, 23, -12) != Axial(-11, -12)"
        );
        assert_eq!(
            Cube::from_coords(-10, 4, 6),
            Cube::from(Axial::from_coords(-10, 6)),
            "Cube(-10, 4, 6) != Axial(-10, 6)"
        );
    }

    #[test]
    fn test_cube_arithmetic() {
        let unit_cube_x = Cube { x: 1, y: 0, z: 0 };
        let unit_cube_y = Cube { x: 0, y: 1, z: 0 };
        let unit_cube_z = Cube { x: 0, y: 0, z: 1 };

        assert_eq!(3 * unit_cube_x, Cube { x: 3, y: 0, z: 0 });
        assert_eq!(unit_cube_y * 5, Cube { x: 0, y: 5, z: 0 });
        assert_eq!(7 * unit_cube_z, Cube { x: 0, y: 0, z: 7 });

        assert_eq!(
            unit_cube_x + unit_cube_y + unit_cube_z,
            Cube { x: 1, y: 1, z: 1 }
        );
        assert_eq!(
            7 * unit_cube_x + 5 * unit_cube_y + 3 * unit_cube_z,
            Cube { x: 7, y: 5, z: 3 }
        );
        assert_eq!(
            3 * (unit_cube_x + unit_cube_y) - 5 * unit_cube_z - unit_cube_y,
            Cube { x: 3, y: 2, z: -5 }
        );
        assert_eq!(
            (4 * unit_cube_x - 2 * unit_cube_y + unit_cube_z) / 2,
            Cube { x: 2, y: -1, z: 0 }
        );
    }

    #[test]
    fn test_cube_neighbors() {
        let exp_origin_neighbors = vec![
            Cube { x: 1, y: 0, z: -1 },
            Cube { x: 1, y: -1, z: 0 },
            Cube { x: 0, y: -1, z: 1 },
            Cube { x: -1, y: 0, z: 1 },
            Cube { x: -1, y: 1, z: 0 },
            Cube { x: 0, y: 1, z: -1},
        ];

        assert_eq!(
            exp_origin_neighbors[0],
            Cube::ORIGIN.neighbor(0),
            "origin neighbor (index of 0)"
        );
        assert_eq!(
            exp_origin_neighbors[1],
            Cube::ORIGIN.neighbor(1),
            "origin neighbor (index of 1)"
        );
        assert_eq!(
            exp_origin_neighbors[2],
            Cube::ORIGIN.neighbor(2),
            "origin neighbor (index of 2)"
        );
        assert_eq!(
            exp_origin_neighbors[3],
            Cube::ORIGIN.neighbor(3),
            "origin neighbor (index of 3)"
        );
        assert_eq!(
            exp_origin_neighbors[4],
            Cube::ORIGIN.neighbor(4),
            "origin neighbor (index of 4)"
        );
        assert_eq!(
            exp_origin_neighbors[5],
            Cube::ORIGIN.neighbor(5),
            "origin neighbor (index of 5)"
        );
        assert_eq!(
            exp_origin_neighbors,
            Cube::ORIGIN.neighbors(),
            "origin neighbors"
        );

        let offset_coord = Cube { x: -4, y: 13, z: -9 };
        let exp_offset_neighbors = vec![
            Cube { x: -3, y: 13, z: -10 },
            Cube { x: -3, y: 12, z: -9 },
            Cube { x: -4, y: 12, z: -8 },
            Cube { x: -5, y: 13, z: -8 },
            Cube { x: -5, y: 14, z: -9 },
            Cube { x: -4, y: 14, z: -10 },
        ];

        assert_eq!(
            exp_offset_neighbors[0],
            offset_coord.neighbor(0),
            "offset neighbor (index of 0)"
        );
        assert_eq!(
            exp_offset_neighbors[1],
            offset_coord.neighbor(1),
            "offset neighbor (index of 1)"
        );
        assert_eq!(
            exp_offset_neighbors[2],
            offset_coord.neighbor(2),
            "offset neighbor (index of 2)"
        );
        assert_eq!(
            exp_offset_neighbors[3],
            offset_coord.neighbor(3),
            "offset neighbor (index of 3)"
        );
        assert_eq!(
            exp_offset_neighbors[4],
            offset_coord.neighbor(4),
            "offset neighbor (index of 4)"
        );
        assert_eq!(
            exp_offset_neighbors[5],
            offset_coord.neighbor(5),
            "offset neighbor (index of 5)"
        );
        assert_eq!(
            exp_offset_neighbors,
            offset_coord.neighbors(),
            "offset neighbors"
        );
    }

    #[test]
    fn test_cube_diagonals() {
        let exp_origin_diagonals = vec![
            Cube { x: 1, y: -2, z: 1 },
            Cube { x: -1, y: -1, z: 2 },
            Cube { x: -2, y: 1, z: 1 },
            Cube { x: -1, y: 2, z: -1 },
            Cube { x: 1, y: 1, z: -2 },
            Cube { x: 2, y: -1, z: -1},
        ];

        assert_eq!(
            exp_origin_diagonals[0],
            Cube::ORIGIN.diagonal(0),
            "origin diagonal (index of 0)"
        );
        assert_eq!(
            exp_origin_diagonals[1],
            Cube::ORIGIN.diagonal(1),
            "origin diagonal (index of 1)"
        );
        assert_eq!(
            exp_origin_diagonals[2],
            Cube::ORIGIN.diagonal(2),
            "origin diagonal (index of 2)"
        );
        assert_eq!(
            exp_origin_diagonals[3],
            Cube::ORIGIN.diagonal(3),
            "origin diagonal (index of 3)"
        );
        assert_eq!(
            exp_origin_diagonals[4],
            Cube::ORIGIN.diagonal(4),
            "origin diagonal (index of 4)"
        );
        assert_eq!(
            exp_origin_diagonals[5],
            Cube::ORIGIN.diagonal(5),
            "origin diagonal (index of 5)"
        );
        assert_eq!(
            exp_origin_diagonals,
            Cube::ORIGIN.diagonals(),
            "origin diagonals"
        );

        let offset_coord = Cube { x: 7, y: 3, z: -10 };
        let exp_offset_diagonals = vec![
            Cube { x: 8, y: 1, z: -9 },
            Cube { x: 6, y: 2, z: -8 },
            Cube { x: 5, y: 4, z: -9 },
            Cube { x: 6, y: 5, z: -11 },
            Cube { x: 8, y: 4, z: -12 },
            Cube { x: 9, y: 2, z: -11},
        ];

        assert_eq!(
            exp_offset_diagonals[0],
            offset_coord.diagonal(0),
            "offset diagonal (index of 0)"
        );
        assert_eq!(
            exp_offset_diagonals[1],
            offset_coord.diagonal(1),
            "offset diagonal (index of 1)"
        );
        assert_eq!(
            exp_offset_diagonals[2],
            offset_coord.diagonal(2),
            "offset diagonal (index of 2)"
        );
        assert_eq!(
            exp_offset_diagonals[3],
            offset_coord.diagonal(3),
            "offset diagonal (index of 3)"
        );
        assert_eq!(
            exp_offset_diagonals[4],
            offset_coord.diagonal(4),
            "offset diagonal (index of 4)"
        );
        assert_eq!(
            exp_offset_diagonals[5],
            offset_coord.diagonal(5),
            "offset diagonal (index of 5)"
        );
        assert_eq!(
            exp_offset_diagonals,
            offset_coord.diagonals(),
            "offset diagonals"
        );
    }

    #[test]
    fn test_cube_rings() {
        let origin_ring_0 = vec![Cube::ORIGIN];
        let origin_ring_1 = vec![
            Cube::from_coords(1, 0, -1),
            Cube::from_coords(1, -1, 0),
            Cube::from_coords(0, -1, 1),
            Cube::from_coords(-1, 0, 1),
            Cube::from_coords(-1, 1, 0),
            Cube::from_coords(0, 1, -1),
        ];
        let origin_ring_2 = vec![
            Cube::from_coords(2, 0, -2),
            Cube::from_coords(2, -1, -1),
            Cube::from_coords(2, -2, 0),
            Cube::from_coords(1, -2, 1),
            Cube::from_coords(0, -2, 2),
            Cube::from_coords(-1, -1, 2),
            Cube::from_coords(-2, 0, 2),
            Cube::from_coords(-2, 1, 1),
            Cube::from_coords(-2, 2, 0),
            Cube::from_coords(-1, 2, -1),
            Cube::from_coords(0, 2, -2),
            Cube::from_coords(1, 1, -2),
        ];
        let origin_ring_5 = vec![
            Cube::from_coords(5, 0, -5),
            Cube::from_coords(5, -1, -4),
            Cube::from_coords(5, -2, -3),
            Cube::from_coords(5, -3, -2),
            Cube::from_coords(5, -4, -1),
            Cube::from_coords(5, -5, 0),
            Cube::from_coords(4, -5, 1),
            Cube::from_coords(3, -5, 2),
            Cube::from_coords(2, -5, 3),
            Cube::from_coords(1, -5, 4),
            Cube::from_coords(0, -5, 5),
            Cube::from_coords(-1, -4, 5),
            Cube::from_coords(-2, -3, 5),
            Cube::from_coords(-3, -2, 5),
            Cube::from_coords(-4, -1, 5),
            Cube::from_coords(-5, 0, 5),
            Cube::from_coords(-5, 1, 4),
            Cube::from_coords(-5, 2, 3),
            Cube::from_coords(-5, 3, 2),
            Cube::from_coords(-5, 4, 1),
            Cube::from_coords(-5, 5, 0),
            Cube::from_coords(-4, 5, -1),
            Cube::from_coords(-3, 5, -2),
            Cube::from_coords(-2, 5, -3),
            Cube::from_coords(-1, 5, -4),
            Cube::from_coords(0, 5, -5),
            Cube::from_coords(1, 4, -5),
            Cube::from_coords(2, 3, -5),
            Cube::from_coords(3, 2, -5),
            Cube::from_coords(4, 1, -5),
        ];

        assert_eq!(origin_ring_0, Cube::ORIGIN.ring(0), "origin ring 0");
        assert_eq!(origin_ring_1, Cube::ORIGIN.ring(1), "origin ring 1");
        assert_eq!(origin_ring_2, Cube::ORIGIN.ring(2), "origin ring 2");
        assert_eq!(origin_ring_5, Cube::ORIGIN.ring(5), "origin ring 5");

        let offset_coord = Cube::from_coords(2, 3, -5);
        let offset_ring_0 = vec![Cube::from_coords(2, 3, -5)];
        let offset_ring_1 = vec![
            Cube::from_coords(3, 3, -6),
            Cube::from_coords(3, 2, -5),
            Cube::from_coords(2, 2, -4),
            Cube::from_coords(1, 3, -4),
            Cube::from_coords(1, 4, -5),
            Cube::from_coords(2, 4, -6),
        ];
        let offset_ring_2 = vec![
            Cube::from_coords(4, 3, -7),
            Cube::from_coords(4, 2, -6),
            Cube::from_coords(4, 1, -5),
            Cube::from_coords(3, 1, -4),
            Cube::from_coords(2, 1, -3),
            Cube::from_coords(1, 2, -3),
            Cube::from_coords(0, 3, -3),
            Cube::from_coords(0, 4, -4),
            Cube::from_coords(0, 5, -5),
            Cube::from_coords(1, 5, -6),
            Cube::from_coords(2, 5, -7),
            Cube::from_coords(3, 4, -7),
        ];

        assert_eq!(offset_ring_0, offset_coord.ring(0), "offset ring 0");
        assert_eq!(offset_ring_1, offset_coord.ring(1), "offset ring 1");
        assert_eq!(offset_ring_2, offset_coord.ring(2), "offset ring 2");
    }

    #[test]
    fn test_cube_spirals() {
        let origin_spiral_0 = vec![Cube::ORIGIN];
        let origin_spiral_1 = vec![
            Cube::ORIGIN,
            Cube::from_coords(1, 0, -1),
            Cube::from_coords(1, -1, 0),
            Cube::from_coords(0, -1, 1),
            Cube::from_coords(-1, 0, 1),
            Cube::from_coords(-1, 1, 0),
            Cube::from_coords(0, 1, -1),
        ];
        let origin_spiral_2 = vec![
            Cube::ORIGIN,
            Cube::from_coords(1, 0, -1),
            Cube::from_coords(1, -1, 0),
            Cube::from_coords(0, -1, 1),
            Cube::from_coords(-1, 0, 1),
            Cube::from_coords(-1, 1, 0),
            Cube::from_coords(0, 1, -1),
            Cube::from_coords(2, 0, -2),
            Cube::from_coords(2, -1, -1),
            Cube::from_coords(2, -2, 0),
            Cube::from_coords(1, -2, 1),
            Cube::from_coords(0, -2, 2),
            Cube::from_coords(-1, -1, 2),
            Cube::from_coords(-2, 0, 2),
            Cube::from_coords(-2, 1, 1),
            Cube::from_coords(-2, 2, 0),
            Cube::from_coords(-1, 2, -1),
            Cube::from_coords(0, 2, -2),
            Cube::from_coords(1, 1, -2),
        ];

        assert_eq!(
            origin_spiral_0,
            Cube::ORIGIN.spiral(0),
            "origin spiral 0"
        );
        assert_eq!(
            origin_spiral_1,
            Cube::ORIGIN.spiral(1),
            "origin spiral 1"
        );
        assert_eq!(
            origin_spiral_2,
            Cube::ORIGIN.spiral(2),
            "origin spiral 2"
        );

        let offset_spiral_0 = vec![Cube::from_coords(5, -3, -2)];
        let offset_spiral_1 = vec![
            Cube::from_coords(5, -3, -2),
            Cube::from_coords(6, -3, -3),
            Cube::from_coords(6, -4, -2),
            Cube::from_coords(5, -4, -1),
            Cube::from_coords(4, -3, -1),
            Cube::from_coords(4, -2, -2),
            Cube::from_coords(5, -2, -3),
        ];
        let offset_spiral_2 = vec![
            Cube::from_coords(5, -3, -4),
            Cube::from_coords(6, -3, -3),
            Cube::from_coords(6, -4, -2),
            Cube::from_coords(5, -4, -1),
            Cube::from_coords(4, -3, -1),
            Cube::from_coords(4, -2, -2),
            Cube::from_coords(5, -2, -3),
            Cube::from_coords(7, -3, -4),
            Cube::from_coords(7, -4, -3),
            Cube::from_coords(7, -5, -2),
            Cube::from_coords(6, -5, -1),
            Cube::from_coords(5, -5, 0),
            Cube::from_coords(4, -4, 0),
            Cube::from_coords(3, -3, 0),
            Cube::from_coords(3, -2, -1),
            Cube::from_coords(3, -1, -2),
            Cube::from_coords(4, -1, -3),
            Cube::from_coords(5, -1, -4),
            Cube::from_coords(6, -2, -4),
        ];

        assert_eq!(
            offset_spiral_0,
            Cube::from_coords(5, -3, -2).spiral(0),
            "offset spiral 0"
        );
        assert_eq!(
            offset_spiral_1,
            Cube::from_coords(5, -3, -2).spiral(1),
            "offset spiral 1"
        );
        assert_eq!(
            offset_spiral_2,
            Cube::from_coords(5, -3, -2).spiral(2),
            "offset spiral 2"
        );
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
