//! Hexagonal grid
//!
//! # Modifying HexGrid Layouts
//! Just change the `tilt`, `parity`, and/or `sys` field(s) in the `HexGrid`
//! instance. These parameters aren't central to the actual representation of
//! the grid, so changing them is cheap and painless.

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

#[derive(Debug)]
pub enum Compass {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl Compass {
    // add code here
}

type HexGraph<T> = StableGraph<T, Compass>;

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

impl<T> Default for HexGrid<T> {
    fn default() -> Self {
        Self {
            tilt: Tilt::Flat,
            parity: Parity::Odd,
            sys: CoordSys::Cube,
            graph: StableGraph::new(),
            map: HashMap::new(),
        }
    }
}

impl<T> HexGrid<T> {
    //////////////////////////////////
    // Utilities
    //////////////////////////////////

    /// Convert a `MultiCoord` to its `Cube` equivalent for the given
    /// `HexGrid`.
    fn cube_from(&self, coord: MultiCoord) -> Cube {
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

    /// Return the `NodeIndex` of the node at the given coordinate if it
    /// exists.
    fn graph_index(&self, coord: MultiCoord) -> Option<&NodeIndex> {
        self.map.get(&self.cube_from(coord))
    }

    ///
    fn single_insert(&mut self, coord: MultiCoord, data: T) {
        match self.graph_index(coord) {
            Some(index) => {},
            _ => {},
        }
    }

    fn single_link_burst(&mut self, coord: MultiCoord) {
        match self.graph_index(coord) {
            Some(index) => {
                let ncoords = self.cube_from(coord).neighbors();

                for neighbor in &ncoords {

                }
            }
            _ => (),
        }
    }

    //////////////////////////////////
    // Initialization
    //////////////////////////////////

    /// Creates an empty `HexGrid` with the given parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn new(tilt: Tilt, parity: Parity, sys: CoordSys) -> Self {
        Self {
            tilt: tilt,
            parity: parity,
            sys: sys,
            ..Default::default()
        }
    }

    /// Creates a new, radial `HexGrid` with a given number of hexes from
    /// center to edge. `radius` includes the center hex, so a `radius` of 0
    /// would result in an empty `HexGrid`.
    ///
    /// `sys` defaults to `CoordSys::Cube` in the instantiated `HexGrid`.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn new_radial(radius: u32) -> Self {
        if radius == 0 {
            Default::default()
        } else {
            let new_hexes = Cube::ORIGIN.spiral(radius);

            for new_hex in &new_hexes {
                // two-passes (create then link)
                unimplemented!();
            }

            unimplemented!();
        }
    }

    /// Creates a new, rectangular `HexGrid` with a given number of rows and
    /// columns.
    ///
    /// Initial values of `tilt`, `parity`, and `sys`` for `HexGrid`s
    /// instantiated with this method are `Tilt::Flat`, `Parity::Odd`, and
    /// `CoordSys::Offset`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn new_boxy(rows: u32, cols: u32) -> Self {
        for row in 0..rows {
            for col in 0..cols {
                unimplemented!();
            }
        }

        unimplemented!();
    }

    //////////////////////////////////
    // Boolean Analysis
    //////////////////////////////////

    /// Returns `true` if the calling instance has a value associated with the
    /// given coordinate, else `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn contains_coord(&self, coord: MultiCoord) -> bool {
        self.graph_index(coord).is_some()
    }

    /// Returns `true` if the calling instance contains one or more hex values
    /// equal to the given target, else `false`. The target type must have the
    /// `Eq` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn contains_value(&self, target: T) -> bool {
        unimplemented!();
    }

    //////////////////////////////////
    // Retrieval
    //////////////////////////////////

    /// Return an immutable reference to the data contained at the given
    /// coordinate within the grid, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn get(&self, coord: MultiCoord) -> Option<&T> {
        match self.graph_index(coord) {
            Some(&index) => self.graph.node_weight(index),
            _ => None,
        }
    }

    /// Return a mutable reference to the data contained at the given
    /// coordinate within the grid, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn get_mut(&mut self, coord: MultiCoord) -> Option<&mut T> {
        match self.graph_index(coord) {
            Some(&index) => self.graph.node_weight_mut(index),
            _ => None,
        }
    }

    //////////////////////////////////
    // Extension & Modification
    //////////////////////////////////

    /// Strictly add a hex value to the grid at a given coordinate. Returns a
    /// `Result::Err(String)` if there is already a value at the given
    /// coordinate. Otherwise, returns `Result::Ok(())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::hexgrid::HexGrid;
    /// use chickenwire::coordinate::{Cube, MultiCoord};
    ///
    /// let &mut grid = HexGrid::new();
    ///
    /// let coord_1 = MultiCoord::from(Cube::from_coords(1, 2, -3));
    /// let coord_2 = MultiCoord::from(Cube::from_coords(3, -4, 1));
    ///
    /// unimplemented!();
    /// ```
    pub fn add(&mut self, coord: MultiCoord) -> Result<(), String> {
        if self.contains_coord(coord) {
            let str = format!("Grid already contains a value at {:?}", coord);

            Result::Err(str)
        } else {
            unimplemented!();

            Result::Ok(())
        }
    }

    /// Strictly update a pre-existing hex value at a given coordinate.
    /// Returns a `Result::Err(String)` if the position is vacant. Otherwise,
    /// returns `Result::Ok(())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::hexgrid::HexGrid;
    /// use chickenwire::coordinate::{Cube, MultiCoord};
    ///
    /// let &mut grid = HexGrid::new();
    ///
    /// let coord_1 = MultiCoord::from(Cube::from_coords(1, 2, -3));
    /// let coord_2 = MultiCoord::from(Cube::from_coords(3, -4, 1));
    ///
    /// unimplemented!();
    /// ```
    pub fn update(&mut self, coord: MultiCoord) -> Result<(), String> {
        if self.contains_coord(coord) {
            unimplemented!();

            Result::Ok(())
        } else {
            let str = format!("Grid contains no value at {:?}", coord);

            Result::Err(str)
        }
    }

    /// Either creates & adds or updates a hex value at the given coordinate.
    ///
    /// # Examples
    ///
    /// # Examples
    ///
    /// ```
    /// use chickenwire::hexgrid::HexGrid;
    /// use chickenwire::coordinate::{Cube, MultiCoord};
    ///
    /// let &mut grid = HexGrid::new();
    ///
    /// let coord_1 = MultiCoord::from(Cube::from_coords(1, 2, -3));
    /// let coord_2 = MultiCoord::from(Cube::from_coords(3, -4, 1));
    ///
    /// unimplemented!();
    /// ```
    pub fn set() {
        unimplemented!();
    }

    /// Cleanly removes a hex and its associated data from the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// unimplemented!();
    /// ```
    pub fn remove(&mut self, coord: MultiCoord) {
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
