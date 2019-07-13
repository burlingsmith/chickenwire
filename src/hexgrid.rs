//! Hexagonal grid
extern crate petgraph;

use petgraph::graph::{Graph, UnGraph, NodeIndex};

use std::rc::Rc;
use std::collections::HashMap;

use crate::coordinate;

// pathfind
// petgraph::graph::Graph::node_weight
// !! switch to constant indexing

// replicate examples from
//  - https://www.danneu.com/elm-hex-grid/
//  - https://www.redblobgames.com/grids/hexagons/

//////////////////////////////////////////////////////////////////////////////
//
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
enum CoordSys {
    Cube,
    Axial,
    Offset,
    Double,
}

#[derive(Debug)]
struct HexGrid<T> {
    pub tilt: Tilt,
    pub parity: Parity,
    pub coord_sys: CoordSys,
    graph: UnGraph<Rc<T>, ()>,
    map: HashMap<coordinate::Cube, NodeIndex>,
}

impl<T> HexGrid<T> {
    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    fn new() -> Self {
        Self {
            tilt: Tilt::Flat,
            parity: Parity::Even,
            coord_sys: CoordSys::Cube,
            graph: Graph::new_undirected(),
            map: HashMap::new(),
        }
    }

    pub fn new_radial(radius: u32, tilt: Tilt) -> Self {
        if radius == 0 {
            HexGrid::new()
        } else {
            let new_hexes = coordinate::Cube::ORIGIN.spiral(radius);

            for new_hex in &new_hexes {
                // two-passes (create then link) ?
                unimplemented!();
            }

            unimplemented!();
        }
    }

    pub fn new_boxy(rows: u32, cols: u32, tilt: Tilt, parity: Parity) -> Self {
        for row in 0..rows {
            for col in 0..cols {
                unimplemented!();
            }
        }

        unimplemented!();
    }
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
