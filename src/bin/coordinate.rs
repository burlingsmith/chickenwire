//! # Coordinate Systems
//!
//! This module encodes the four coordinate systems included in
//! [_Red Blob Games_' hexagonal grid guide](https://www.redblobgames.com/grids/hexagons/#basics)

use std::convert::{From};
use hex


/*****************************************************************************
 Common
 ******/

/// For rectangle-like grids, rows or columns must have an alternating
/// indentation level. This is encoded as either `Even` or `Odd`, meaning that
/// every even or odd row is more indented, respectively
#[derive(Debug)]
pub enum Sidedness {
    Even,
    Odd,
}


/*****************************************************************************
 Offset Coordinates
 ******************/

/// Offset coordinates encode position as though the hexagonal grid were
/// rectangular, but with alternating rows or columns being offset
/// horizontally or vertically, respectively.
#[derive(Debug)]
pub struct Offset {
    col: u64,
    row: u64,
    sys: HexType,
    lop: Sidedness,
}

impl Offset {
    fn is_even() {
        unimplemented!();
    }

    fn is_odd() {
        unimplemented!();
    }

    fn is_flat() {
        unimplemented!();
    }

    fn is_sharp() {
        unimplemented!();
    }

    fn to_even() {
        unimplemented!();
    }

    fn to_odd() {
        unimplemented!();
    }

    fn to_flat() {
        unimplemented!();
    }

    fn to_sharp() {
        unimplemented!();
    }
}

impl From<Interlaced> for Offset {
    unimplemented!();
}

impl From<Cube> for Offset {
    unimplemented!();
}

impl From<Axial> for Offset {
    unimplemented!();
}


/*****************************************************************************
 Interlaced Coordinates
 **********************/

/// Interlaced coordinates encode position like offset coordinates, but by
/// doubling the step size between rows/columns, more or less interlacing two
/// grids which are aligned with themselves but offset with each other.
/// Interlaced coordinates also have the additional constraint that the sum of
/// each hex's column and row coordinates is always even.
#[derive(Debug)]
pub struct Interlaced {
    col: u64,
    row: u64,
    sys: HexType,
}

impl From<Offset> for Interlaced {
    fn from(coords: Offset) -> Self {
        unimplemented!();
    }
}

impl From<Cube> for Interlaced {
    fn from(coords: Cube) -> Self {
        unimplemented!();
    }
}

impl From<Axial> for Interlaced {
    fn from(coords: Axial) -> Self {
        unimplemented!();
    }
}


/*****************************************************************************
 Cube Coordinates
 ****************/

/// Cube coordinates encode position by treating hexagonal grids as flattend
/// images of a cube sliced along it's diagonal. This coordinate system bears
/// the constaint that x + y + z = 0.
#[derive(Debug)]
pub struct Cube {
    x: u64,
    y: u64,
    z: u64,
}

impl From<Offset> for Cube {
    fn from(coords: Offset) -> Self {
        let col = coords.col;
        let row = coords.row;

        match (coords.sys, coords.lop) {
            (hex::Flat, Even) => {
                let z = row - (col + (col & 1)) / 2;

                Cube { x: col, y: 0 - x - z, z: z }
            }
            (hex::Flat, Odd) => {
                let z = row - (col - (col & 1)) / 2;

                Cube { x: col, y: 0 - x - z, z: z }
            }
            (hex::Sharp, Even) => {
                let x = col - (row + (row & 1)) / 2;

                Cube { x: x, y: 0 - x - row, z: row }
            }
            (hex::Sharp, Odd) => {
                let x = col - (row - (row & 1)) / 2;

                Cube { x: x, y: 0 - x - row, z: row }
            }
        }
    }
}

impl From<Interlaced> for Cube {
    fn from(coords: Interlaced) -> Self {
        let col = coords.col;
        let row = coords.row;

        match coords.sys {
            hex::Flat => {
                let z = (row - col) / 2;

                Cube { x: col, y: 0 - col - z, z: z }
            }
            hex::Sharp => {
                let x = (col - row) / 2;

                Cube { x: x, y: 0 - x - row, z: row }
            }
        }
    }
}

impl From<Axial> for Cube {
    fn from(coords: Axial) -> Self {
        let x = coords.q;
        let y = coords.r;
        let z = 0 - x - y;

        Cube { x: x, y: y, z: z }
    }
}


/*****************************************************************************
 Axial Coordinates
 *****************/

/// Axial coordinates encode position by using 2 of the 3 components of a cube
/// coordinate, leveraging the latter's x + y + z = 0 constraint to eliminate
/// the redundant information.
#[derive(Debug)]
pub struct Axial {
    q: u64,
    r: u64,
}

impl From<Offset> for Axial {
    fn from(coords: Offset) -> Self {
        unimplemented!();
    }
}

impl From<Interlaced> for Axial {
    fn from(coords: Interlaced) -> Self {
        unimplemented!();
     }
}

impl From<Cube> for Axial {
    fn from(coords: Cube) -> Self {
        Axial { q: coords.x, r: coords.y }
    }
}


/*****************************************************************************
 Unit Testing
 ************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_conversion() {
        unimplemented!();
    }

    #[test]
    fn test_interlaced_conversion() {
        unimplemented!();
    }

    #[test]
    fn test_cube_conversion() {
        unimplemented!();
    }

    #[test]
    fn test_axial_conversion() {
        unimplemented!();
    }
}
