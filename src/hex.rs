//!

use coordinate;

// (?) make iterator for things like rings
// (?) add limited range versions of map and search functions
// (?) add trait for obstruction and movement cost/movement events

//////////////////////////////////////////////////////////////////////////////
// Hex Orientation
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Compass {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

#[derive(Debug)]
enum Tilt {
    Flat,
    Sharp,
}

//////////////////////////////////////////////////////////////////////////////
// Hexes
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Hex<T> {
    pos: coordinate::Cube,
    tilt: Tilt,
    data: T,
}

impl<T> Hex<T> {
    // Code here
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
