//! Hexagonal grid
/*
extern crate petgraph;

use petgraph::graph::UnGraph;

use std::rc::Rc;
use std::hash::{Hash, Hasher};

//////////////////////////////////////////////////////////////////////////////
// Hexes
//////////////////////////////////////////////////////////////////////////////

type HexRef<T> = Rc<Hex<T>>;

#[derive(Debug)]
struct Hex<T> {
    // contents //
}

//////////////////////////////////////////////////////////////////////////////
// Hexagonal Grids
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Tilt {
    Flat,
    Sharp,
}

#[derive(Debug)]
enum Parity {
    Even,
    Odd,
}

#[derive(Debug)]
struct HexGrid<T> {
    // tilt //
    // parity //
    // graph //
    // map //
}

impl<T> HexGrid<T> {
    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    fn new() -> Self {
        unimplemented!();
    }

    fn new_radial(radius: u32) -> Self {
        if radius == 0 {
            HexGrid::new()
        } else {
            unimplemented!();
        }
    }

    fn new_boxy(rows: u32, cols: u32, ) -> Self {
        unimplemented!();
    }

    //////////////////////////////////
    //
    //////////////////////////////////

    fn pathfind(&self, ) -> RetType {
        unimplemented!();
    }
}
*/
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
