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
//! `Axial` and `Offset` coordinates can be instantiated normally:
//!
//! ```
//! use chickenwire::coordinate::{Axial, Offset};
//!
//! let axial = Axial { q: 0, r: 1 };
//! let offset = Offset { col: 2, row: 3 };
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
//! `Cube` and `Double` coordinates have enforced constraints. If these
//! constraints aren't met during instantiation, the program will panic. See
//! the `Cube` and `Double` documentation for more information.
//!
//! # Modifying Coordinates
//! Use the `set_coords` method to update a `Cube` or `Double`. Otherwise,
//! coordinate `struct`s should be manipulated normally via their public
//! fields.
//!
//! # Coordinate Conversion
//! The `From` and `Into` traits are implemented between `Axial` and `Cube`,
//! and for between all coordinates and `MultiCoord`s. For `Offset` and
//! `Double` coordinates, conversion to a different system requires additional
//! knowledge about the grid's state. Namely, whether the hexes have a "flat"
//! or "sharp" orientation and if the offset patterning is "even" or "odd."
//! Conversion methods for each circumstance are documented in the coordinate
//! system sub-modules. See either the `chickenwire::hexgrid` documentation or
//! [The Guide](https://www.redblobgames.com/grids/hexagons) for explantations
//! of flat/sharp orientation and even/odd offsetting.
//!
//! # Arithmetic
//! The `Add<Self>`, `Sub<Self>`, and `Mul<i32>` traits are implemented for
//! `Axial`, `Cube`, and `Double`. `Div<i32>` is implemented for `Axial` and
//! `Cube`. These operations treat the coordinates as vectors:
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
//!     Double::from_coords(4, -2) - Double::from_coords(-3, 1),
//!     Double::from_coords(7, -3)
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
// Convenience Aliases
//////////////////////////////////////////////////////////////////////////////

/// [ docs missing ]
pub type CoordResult<T> = Result<T, &'static str>;

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

/// The default `CoordSys` is `CoordSys::Axial`.
impl Default for CoordSys {
    fn default() -> Self {
        CoordSys::Axial
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

impl MultiCoord {
    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    /// Convenience function equivalent to
    /// `MultiCoord::from(Axial { q: q, r: r })`
    pub fn axial(q: i32, r: i32) -> Self {
        Self::from(Axial::from_coords(q, r))
    }

    /// [ docs missing ]
    pub fn cube(x: i32, y: i32, z: i32) -> CoordResult<Self> {
        match Cube::from_coords(x, y, z) {
            Ok(cube) => Ok(Self::from(cube)),
            Err(msg) => Err(msg),
        }
    }

    /// [ docs missing ]
    pub fn force_cube(x: i32, y: i32, z: i32) -> Self {
        Self::from(Cube::force_from_coords(x, y, z))
    }

    /// [ docs missing ]
    pub fn double(col: i32, row: i32) -> CoordResult<Self> {
        match Double::from_coords(col, row) {
            Ok (double) => Ok(Self::from(double)),
            Err(msg) => Err(msg),
        }
    }

    /// [ docs missing ]
    pub fn force_double(col: i32, row: i32) -> Self {
        Self::from(Double::force_from_coords(col, row))
    }

    /// [ docs missing ]
    pub fn offset(col: i32, row: i32) -> Self {
        Self::from(Offset::from_coords(col, row))
    }

    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    /// Attempts to create an `Axial` from a `MultiCoord`. If successful,
    /// returns the result wrapped in an `Ok`. Otherwise, returns an `Err`.
    pub fn to_axial(self) -> CoordResult<Axial> {
        match self.sys {
            CoordSys::Axial | CoordSys::Cube => Ok(Axial::from(self)),
            CoordSys::Double => {
                Err("cannot create Axial from Double MultiCoord")
            }
            CoordSys::Offset => {
                Err("cannot create Axial from Offset MultiCoord")
            }
        }
    }

    /// Attempts to create a `Cube` from a `MultiCoord`. If successful,
    /// returns the result wrapped in an `Ok`. Otherwise, returns an `Err`.
    pub fn to_cube(self) -> CoordResult<Cube> {
        match self.sys {
            CoordSys::Axial | CoordSys::Cube => Ok(Cube::from(self)),
            CoordSys::Double => {
                Err("cannot create Cube from Double MultiCoord")
            }
            CoordSys::Offset => {
                Err("cannot create Cube from Offset MultiCoord")
            }
        }
    }

    /// Attempts to create a `Double` from a `MultiCoord`. If successful,
    /// returns the result wrapped in an `Ok`. Otherwise, returns an `Err`.
    pub fn to_double(self) -> CoordResult<Double> {
        match self.sys {
            CoordSys::Double => Ok(Double::from(self)),
            CoordSys::Axial => {
                Err("cannot create Double from Axial MultiCoord")
            }
            CoordSys::Cube => {
                Err("cannot create Double from Cube MultiCoord")
            }
            CoordSys::Offset => {
                Err("cannot create Double from Offset MultiCoord")
            }
        }
    }

    /// Attempts to create an `Offset` from a `MultiCoord`. If successful,
    /// returns the result wrapped in an `Ok`. Otherwise, returns an `Err`.
    pub fn to_offset(self) -> CoordResult<Offset> {
        match self.sys {
            CoordSys::Offset => Ok(Offset::from(self)),
            CoordSys::Axial => {
                Err("cannot create Offset from Axial MultiCoord")
            }
            CoordSys::Cube => {
                Err("cannot create Offset from Cube MultiCoord")
            }
            CoordSys::Double => {
                Err("cannot create Offset from Double MultiCoord")
            }
        }
    }
}
