//! Hexagonal grid

use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::Undirected;

use std::collections::HashMap;

use crate::coordinate::*;

// pathfind
// petgraph::graph::Graph::node_weight
// !! switch to constant indexing

// replicate & improve upon examples from
//  - https://www.danneu.com/elm-hex-grid/
//  - https://www.redblobgames.com/grids/hexagons/
// add examples about creating maybe

//////////////////////////////////////////////////////////////////////////////
//
//////////////////////////////////////////////////////////////////////////////

type HexGraph<T> = StableGraph<T, (), Undirected>;

type HexMap = HashMap<Cube, NodeIndex>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Tilt {
    Flat,
    Sharp,
}

impl Default for Tilt {
    fn default() -> Self { Tilt::Flat }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Parity {
    Even,
    Odd,
}

impl Default for Parity {
    fn default() -> Self { Parity::Even }
}

#[derive(Debug)]
struct HexGrid<T> {
    pub tilt: Tilt,
    pub parity: Parity,
    pub sys: CoordSys,
    graph: HexGraph<T>,
    map: HexMap,
}

impl<T> HexGrid<T> {
    //////////////////////////////////
    // Utilities
    //////////////////////////////////

    fn cube_coord(&self, coord: MultiCoord) -> Cube {
        match CoordSys::from(coord) {
            Offset => {
                let offset = Offset::from(coord);

                match (self.tilt, self.parity) {
                    (Flat, Odd) => Offset::oflat_to_cube(offset),
                    (Flat, Even) => Offset::eflat_to_cube(offset),
                    (Sharp, Odd) => Offset::osharp_to_cube(offset),
                    (Sharp, Even) => Offset::esharp_to_cube(offset),
                }
            }
            Double => {
                let double = Double::from(coord);

                match self.tilt {
                    Flat => Double::flat_to_cube(double),
                    _ => Double::sharp_to_cube(double),
                }
            }
            _ => Cube::from(coord),
        }
    }

    fn graph_index(&self, coord: MultiCoord) -> Option<&NodeIndex> {
        self.map.get(&self.cube_coord(coord))
    }

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn new_radial(radius: u32, tilt: Tilt) -> Self {
        if radius == 0 {
            HexGrid::new()
        } else {
            let new_hexes = Cube::ORIGIN.spiral(radius);

            for new_hex in &new_hexes {
                // two-passes (create then link)
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

    //////////////////////////////////
    //
    //////////////////////////////////

    pub fn get(&self, coord: MultiCoord) -> Option<&T> {
        match self.graph_index(coord) {
            Some(&index) => self.graph.node_weight(index),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, coord: MultiCoord) -> Option<&mut T> {
        match self.graph_index(coord) {
            Some(&index) => self.graph.node_weight_mut(index),
            _ => None,
        }
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
