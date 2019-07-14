//! Contains hexagon geometry and pixel operations

use crate::hexgrid::*;

pub type Position = (f64, f64);

#[derive(Debug)]
pub enum Compass {
    North,       // direction   0 degrees clockwise to center, before rotation
    NortheastA,  // direction  30 degrees clockwise to center, before rotation
    NortheastB,  // direction  60 degrees clockwise to center, before rotation
    East,        // direction  90 degrees clockwise to center, before rotation
    SoutheastA,  // direction 120 degrees clockwise to center, before rotation
    SoutheastB,  // direction 150 degrees clockwise to center, before rotation
    South,       // direction 180 degrees clockwise to center, before rotation
    SouthwestA,  // direction 210 degrees clockwise to center, before rotation
    SouthwestB,  // direction 240 degrees clockwise to center, before rotation
    West,        // direction 270 degrees clockwise to center, before rotation
    NorthwestA,  // direction 300 degrees clockwise to center, before rotation
    NortheastB,  // direction 330 degrees clockwise to center, before rotation
}

#[derive(Debug)]
pub struct Hexagon {
    pub center: Position,
    rotation: f64,      // clockwise rotation in radians
    side_len: f64,
}

impl Hexagon {
    pub fn width(&self) -> f64 {
        sqrt(3) * self.side_len
    }

    pub fn height(&self) -> f64 {
        2 * self.side_len
    }

    pub fn hspacing(&self) -> f64 {
        self.width()
    }

    pub fn vspacing(&self) -> f64 {
        1.5 * self.side_len
    }

    pub fn corners(&self) -> Vec<Position> {
        unimplemented!();
    }
}
