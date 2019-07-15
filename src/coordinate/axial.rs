//! Axial Coordinates

use std::ops::{Add, Div, Mul, Sub};

use super::*;

//////////////////////////////////////////////////////////////////////////////
// Primary Structure
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

//////////////////////////////////////////////////////////////////////////////
// Traits
//////////////////////////////////////////////////////////////////////////////

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
        let (x, _, z) = coord.to_tuple();

        Self { q: x, r: z }
    }
}

impl From<MultiCoord> for Axial {  // panics on bad c unwrap
    fn from(coord: MultiCoord) -> Self {
        match coord.sys {
            CoordSys::Axial => Axial { q: coord.a, r: coord.b },
            CoordSys::Cube => Axial { q: coord.a, r: coord.c.unwrap() },
            _ => panic!("{:?} is not an Axial or Cube coordinate", coord),
        }
    }
}

impl Add for Axial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl Sub for Axial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

impl Mul<i32> for Axial {
    type Output = Self;

    fn mul(self, n: i32) -> Self {
        Self {
            q: self.q * n,
            r: self.r * n,
        }
    }
}

impl Mul<Axial> for i32 {
    type Output = Axial;

    fn mul(self, coord: Axial) -> Axial {
        coord * self
    }
}

impl Div<i32> for Axial {
    type Output = Self;

    fn div(self, n: i32) -> Self {
        Self {
            q: self.q / n,
            r: self.r / n,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Methods
//////////////////////////////////////////////////////////////////////////////

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
// Unit Tests
//////////////////////////////////////////////////////////////////////////////
