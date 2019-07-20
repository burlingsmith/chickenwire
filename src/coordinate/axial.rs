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
// Traits: Arithmetic
//////////////////////////////////////////////////////////////////////////////

/// Axial coordinates are added together like vectors.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Axial;
///
/// let coord_1 = Axial::from_coords(1, -3);
/// let coord_2 = Axial::from_coords(-5, 12);
///
/// assert_eq!(coord_1 + coord_2, Axial::from_coords(-4, 9));
/// assert_eq!(coord_2 + coord_1, Axial::from_coords(-4, 9));
/// ```
impl Add for Axial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

/// Axial coordinates are subtracted from each other like vectors.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Axial;
///
/// let coord_1 = Axial::from_coords(1, -3);
/// let coord_2 = Axial::from_coords(5, -12);
///
/// assert_eq!(coord_1 - coord_2, Axial::from_coords(-4, 9));
/// assert_eq!(coord_2 - coord_1, Axial::from_coords(4, -9));
/// ```
impl Sub for Axial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            q: self.q - other.q,
            r: self.r - other.r,
        }
    }
}

/// Axial coordinates can be multiplied by `i32` scalars, like vectors.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Axial;
///
/// let coord = Axial::from_coords(1, -3);
///
/// assert_eq!(-1 * coord, Axial::from_coords(-1, 3));
/// assert_eq!(0 * coord, Axial::ORIGIN);
/// assert_eq!(coord * 2, Axial::from_coords(2, -6));
/// ```
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

/// Axial coordinates can be divided by `i32` scalars, like vectors. Values are
/// rounded toward zero, truncating the fractional component.
///
/// # Panics
///
/// Panics when trying to divide by zero.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Axial;
///
/// let coord = Axial::from_coords(12, -36);
///
/// assert_eq!(coord / -1, Axial::from_coords(-12, 36));
/// assert_eq!(coord / 2, Axial::from_coords(6, -18));
/// assert_eq!(coord / 3, Axial::from_coords(4, -12));
/// ```
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
// Traits: From & Into
//////////////////////////////////////////////////////////////////////////////

/// Creates an `Axial` from an `(i32, i32)`.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::Axial;
///
/// assert_eq!(
///     Axial::from((1, 2)),
///     Axial { q: 1, r: 2 }
/// );
/// ```
impl From<(i32, i32)> for Axial {
    fn from((q, r): (i32, i32)) -> Self {
        Self {
            q: q,
            r: r,
        }
    }
}

/// Creates an `Axial` from a `Cube`.
///
/// # Examples
///
/// ```
/// use chickenwire::coordinate::{Cube, Axial};
///
/// let cube = Cube::from_coords(1, 2, -3);
///
/// assert_eq!(
///     Axial::from(cube),
///     Axial { q: 1, r: -3 }
/// );
/// assert_eq!(
///     Axial::from(Cube::ORIGIN),
///     Axial::ORIGIN
/// );
/// ```
impl From<Cube> for Axial {
    fn from(coord: Cube) -> Self {
        let (x, _, z) = coord.to_tuple();

        Self { q: x, r: z }
    }
}

/// Creates an `Axial` from a `MultiCoord`.
///
/// Conversion from both axial and cube `MultiCoord`s is supported.
///
/// # Panics
///
/// Panics when parsing a double or offset `MultiCoord`.
impl From<MultiCoord> for Axial {  // panics on bad c unwrap
    fn from(coord: MultiCoord) -> Self {
        match coord.sys {
            CoordSys::Axial => Axial { q: coord.a, r: coord.b },
            CoordSys::Cube => Axial { q: coord.a, r: coord.c.unwrap() },
            _ => panic!("{:?} is not an Axial or Cube coordinate", coord),
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

    /// `Axial` coordinate origin of (0, 0).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(1 + 1, 2);
    }
}
