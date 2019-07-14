//! Contains hexagon geometry and pixel operations

use crate::hexgrid::*;

//////////////////////////////////////////////////////////////////////////////
// Single Hexes
//////////////////////////////////////////////////////////////////////////////

pub type Position = (f64, f64);

#[derive(Debug)]
pub struct Hexagon {
    pub center: Position,
    rotation: f64,      // clockwise rotation in radians
    side_len: f64,
}

impl Hexagon {
    pub fn reg_width(&self) -> f64 {
        sqrt(3) * self.side_len
    }

    pub fn reg_height(&self) -> f64 {
        2 * self.side_len
    }

    pub fn reg_horizontal_spacing(&self) -> f64 {
        self.width()
    }

    pub fn reg_vertical_spacing(&self) -> f64 {
        1.5 * self.side_len
    }

    pub fn reg_corners(&self) -> Vec<Position> {
        unimplemented!();
    }
}

//////////////////////////////////////////////////////////////////////////////
// Hexagonal Grids
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct HexNet {}
