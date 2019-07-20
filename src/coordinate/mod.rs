//! Hexagonal coordinate systems.
//!
//! # Coordinate Systems
//! Chickenwire supports the four coorinate representations presented in
//! [The Guide](https://www.redblobgames.com/grids/hexagons): Cube, Axial,
//! Offset, and Double. The `CoordSys` enum holds valueless labels for each,
//! and each has a `struct` with appropriately labeled values.
//!
//! # Multi-Coordinates
//! In addition to the four coordinates' individual `struct`s, Chickenwire
//! defines a `MultiCoord`. This effectively fulfills the role of a union
//! type, with the added benefit of tracking and maintaining its own typing.
//!
//! # Creating Coordinates
//! Axial, Offset, and Double coordinates can be instantiated normally:
//!
//! ```
//! use chickenwire::coordinate::{Axial, Offset, Double};
//!
//! let axial = Axial { q: 0, r: 1 };
//! let offset = Offset { col: 2, row: 3 };
//! let double = Double { col: 4, row: 5 };
//! ```
//!
//! Additionally, each coordinate `struct` has methods for instantiation from
//! tuple or `i32` values:
//!
//! ```
//! use chickenwire::coordinate::{Cube, Offset};
//!
//! // Use _::from() for tuples
//! let cube_from_tup = Cube::from((1, 2, -3));
//! let offset_from_tup = Offset::from((-1, 0));
//!
//! // Use _::from_coords() for i32s
//! let cube_from_int = Cube::from_coords(1, 2, -3);
//! let offset_from_int = Offset::from_coords(-1, 0);
//!
//! // The result of the two calls is equivalent
//! assert_eq!(cube_from_tup, cube_from_int);
//! assert_eq!(offset_from_tup, offset_from_int);
//! ```
//!
//! Note that, to enforce coordinate constraints, a `Cube` mightn't correspond
//! 1:1 with all three values provided in its instantiation. You can find more
//! details and examples below, in the `From` and `from_coords`
//! implementations of each coordinate's associated `struct`.
//!
//! # Modifying Coordinates
//! Use the `set_coords` method to update a `Cube`. Otherwise, coordinate
//! `struct`s should be manipulated normally via their public fields.
//!
//! # Converting Coordinates
//! The `From` and `Into` traits are implemented between `Axial` and `Cube`.
//! For `Offset` and `Double` coordinates, conversion to a different system
//! requires additional knowledge about the grid's state. Namely, whether the
//! hexes have a "flat" or "sharp" orientation and if the offset patterning is
//! "even" or "odd." Conversion methods for each combination are documented
//! below. See either the `chickenwire::hex` docs or
//! [The Guide](https://www.redblobgames.com/grids/hexagons) for explantations
//! of flat/sharp orientation and even/odd offsetting.
//!
//! # Arithmetic
//! The `Add<Self>`, `Sub<Self>`, `Mul<i32>`, and `Div<i32>` traits are
//! implemented for `Cube` and `Axial`. These operations treat the coordinates
//! as vectors:
//!
//! ```
//! use chickenwire::coordinate::{Cube, Axial};
//!
//! // Vector addition
//! assert_eq!(
//!     Cube::from_coords(2, -4, 2) + Cube::from_coords(5, 6, -11),
//!     Cube::from_coords(7, 2, -9)
//! );
//!
//! // Vector subtraction
//! assert_eq!(
//!     Axial { q: 4, r: -2 } - Axial { q: 1, r: -3 },
//!     Axial { q: 3, r: 1 }
//! );
//!
//! // Scalar multiplication
//! assert_eq!(
//!     Axial { q: 1, r: -3 } * 2i32,
//!     Axial { q: 2, r: -6 }
//! );
//!
//! // Scalar division
//! assert_eq!(
//!     Cube::from_coords(1, 2, -3) / -1i32,
//!     Cube::from_coords(-1, -2, 3)
//! );
//! ```
//!
//! # On Neighbors
//! The exact rule for the ordering of neighbors is that the first position
//! which remains in the same cardinal wedge always receives the zero index,
//! and then indexing procedes clockwise for the remaining neighbors. In
//! practice, this means that the Northeastern neighbor receives the zero
//! index, save for the diagonal case, where the Southeastern neighbor is the
//! recipient.

pub mod axial;
pub mod cube;
pub mod double;
pub mod offset;

use axial::*;
use cube::*;
use double::*;
use offset::*;

//////////////////////////////////////////////////////////////////////////////
// Coordinate System Labels
//////////////////////////////////////////////////////////////////////////////

/// A `CoordSys` is a valueless label for any of the four coordinate systems
/// supported in Chickenwire (`Axial`, `Cube`, `Double`, and `Offset`).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CoordSys {
    Axial,
    Cube,
    Double,
    Offset,
}

/// The default `CoordSys` is `CoordSys::Cube`.
impl Default for CoordSys {
    fn default() -> Self {
        CoordSys::Cube
    }
}

/// Creates a `CoordSys` from a `MultiCoord`.
impl From<MultiCoord> for CoordSys {
    fn from(coord: MultiCoord) -> Self {
        coord.sys
    }
}

/// Creates a `CoordSys::Axial` from an `Axial`.
impl From<Axial> for CoordSys {
    fn from(_: Axial) -> Self {
        CoordSys::Axial
    }
}

/// Creates a `CoordSys::Cube` from a `Cube`.
impl From<Cube> for CoordSys {
    fn from(_: Cube) -> Self {
        CoordSys::Cube
    }
}

/// Creates a `CoordSys::Double` from a `Double`.
impl From<Double> for CoordSys {
    fn from(_: Double) -> Self {
        CoordSys::Double
    }
}

/// Creates a `CoordSys::Offset` from an `Offset`.
impl From<Offset> for CoordSys {
    fn from(_: Offset) -> Self {
        CoordSys::Offset
    }
}

//////////////////////////////////////////////////////////////////////////////
// Multi-Coordinates
//////////////////////////////////////////////////////////////////////////////

/// A `MultiCoord` is capable of representing an `Axial`, `Cube`, `Double`, or
/// `Offset` coordinate, similar to a union type. `MultiCoord` values must be
/// created from one of these coordinates.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MultiCoord {
    sys: CoordSys,
    a: i32,
    b: i32,
    c: Option<i32>,
}

/// Creates a `MultiCoord` from an `Axial`.
impl From<Axial> for MultiCoord {
    fn from(coord: Axial) -> Self {
        Self {
            sys: CoordSys::Axial,
            a: coord.q,
            b: coord.r,
            c: None,
        }
    }
}

/// Creates a `MultiCoord` from a `Cube`.
impl From<Cube> for MultiCoord {
    fn from(coord: Cube) -> Self {
        let (x, y, z) = coord.to_tuple();

        Self {
            sys: CoordSys::Cube,
            a: x,
            b: y,
            c: Some(z),
        }
    }
}

/// Creates a `MultiCoord` from a `Double`.
impl From<Double> for MultiCoord {
    fn from(coord: Double) -> Self {
        Self {
            sys: CoordSys::Double,
            a: coord.col(),
            b: coord.row(),
            c: None,
        }
    }
}

/// Creates a `MultiCoord` from an `Offset`.
impl From<Offset> for MultiCoord {
    fn from(coord: Offset) -> Self {
        Self {
            sys: CoordSys::Offset,
            a: coord.col,
            b: coord.row,
            c: None,
        }
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
        assert_eq!(2 + 2, 4);
    }
}
