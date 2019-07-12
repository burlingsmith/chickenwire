//!

use crate::coordinate;

// (?) make iterator for things like rings
// (?) add limited range versions of map and search functions
// (?) add trait for obstruction and movement cost/movement events

//////////////////////////////////////////////////////////////////////////////
// Hex Orientation
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
// Hexes
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Hex<T> {
    pos: coordinate::MultiCoordinate,
    tilt: Tilt,
    data: T,
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
