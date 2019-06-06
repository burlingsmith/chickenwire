//! # Hexes
//!

use std::option


/*****************************************************************************
 Coordinates
 ***********/

/// chickenwire supports the four coordinate encodings included in
/// [_Red Blob Games_' hexagonal grid
/// guide](https://www.redblobgames.com/grids/hexagons/#basics).
///
/// ## Offset
/// Offset coordinates encode position as though the hexagonal grid were
/// rectangular, but with alternating rows or columns being offset
/// horizontally or vertically, respectively.
///
/// ## Double
/// Double coordinates encode position like offset coordinates, but by
/// doubling the step size between rows/columns, more or less interlacing two
/// grids which are aligned with themselves but offset with each other.
/// Double coordinates also have the additional constraint that the sum of
/// each hex's column and row coordinates is always even.
///
/// ## Cube
/// Cube coordinates encode position by treating hexagonal grids as flattend
/// images of a cube sliced along it's diagonal. This coordinate system bears
/// the constaint that x + y + z = 0.
///
/// ## Axial
/// Axial coordinates encode position by using 2 of the 3 components of a cube
/// coordinate, leveraging the latter's x + y + z = 0 constraint to eliminate
/// the redundant information.
#[derive(Debug)]
enum Coordinate {
    Offset { col: u64, row: u64 },
    Double { col: u64, row: u64 },
    Cube { x: u64, y: u64, z: u64 },
    Axial { q: u64, r: u64},
}


/*****************************************************************************
 Hexes
 *****/

/// chickenwire supports two hexagonal orientations in its grids: `Flat` and
/// `Sharp`, corresponding to the shape of an individual hex's top face.
#[derive(Debug)]
pub enum Rotation {
    Flat,
    Sharp,
}

/// For rectangle-like grids, rows or columns must have an alternating
/// indentation level. This is encoded as either `Even` or `Odd`, meaning that
/// every even or odd row is more indented, respectively
#[derive(Debug)]
pub enum Sidedness {
    Even,
    Odd,
}

/// Hex tiles in a grid are represented as `Hex` structures
#[derive(Debug)]
pub struct Hex {
    pos: Coordinate,        // Position within the hexagonal grid
    rot: Rotation,          // Edge (Flat) or corner (Sharp) at the hex's top
    lop: Option<Sidedness>  // Even/odd indentation for appropriate models
}

/// Calculate the distance between two hexes
pub fn dist(hex_1: Hex, hex_2: Hex) -> u64 {
    match hex_1.pos {
        Offset { } => {
            let tmp_hex = hex_2.to_()
            unimplemented!();
        }
        Double { } => {
            unimplemented!();
        }
        Cube { } => {
            unimplemented!();
        }
        Axial { } => {
            unimplemented!();
        }
    }
}


/*****************************************************************************
 Unit Tests
 **********/
