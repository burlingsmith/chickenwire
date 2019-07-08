//! Hexagonal grid
extern crate petgraph;

use petgraph::graph::{Graph, UnGraph, NodeIndex};

use std::rc::Rc;
use std::collections::HashMap;

use crate::coordinate;

// pathfind
// petgraph::graph::Graph::node_weight

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
    tilt: Tilt,
    parity: Parity,
    coord_sys: CoordSys,
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

    fn new_radial(radius: u32, tilt: Tilt) -> Self {
        if radius == 0 {
            HexGrid::new()
        } else {
            unimplemented!();
        }
    }

    fn new_boxy(rows: u32, cols: u32, tilt: Tilt, parity: Parity) -> Self {
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
