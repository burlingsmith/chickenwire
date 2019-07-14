//! Hexagonal grid
//!
//! # Modifying HexGrid Layouts
//! Just change the `tilt`, `parity`, and/or `sys` field(s) in the `HexGrid`
//! instance. These parameters aren't central to the actual representation of
//! the grid, so changing them is cheap and painless.

use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

use crate::coordinate::*;

// pathfind
// petgraph::graph::Graph::node_weight
// !! switch to constant indexing

// replicate & improve upon examples from
//  - https://www.danneu.com/elm-hex-grid/
//  - https://www.redblobgames.com/grids/hexagons/
// add examples about creating maybe

//////////////////////////////////////////////////////////////////////////////
// Convenience Aliases
//////////////////////////////////////////////////////////////////////////////

type HexGraph<T> = StableGraph<T, Compass>;

type HexMap = HashMap<Cube, NodeIndex>;

//////////////////////////////////////////////////////////////////////////////
//
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    pub fn rotate_cw(self, rotations: u32) -> Self {
        let cur_dir = self;

        for _ in 0..(rotations % 8) {
            let cur_dir = {
                match cur_dir {
                    Compass::North => Compass::Northeast,
                    Compass::Northeast => Compass::East,
                    Compass::East => Compass::Southeast,
                    Compass::Southeast => Compass::South,
                    Compass::South => Compass::Southwest,
                    Compass::Southwest => Compass::West,
                    Compass::West => Compass::Northwest,
                    Compass::Northwest => Compass::North,
                }
            };
        }

        cur_dir
    }

    pub fn inverse(self) -> Self {
        match self {
            Compass::North => Compass::South,
            Compass::Northeast => Compass::Southwest,
            Compass::East => Compass::West,
            Compass::Southeast => Compass::Northwest,
            Compass::South => Compass::North,
            Compass::Southwest => Compass::Northeast,
            Compass::West => Compass::East,
            Compass::Northwest => Compass::Southeast,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Tilt {
    Flat,
    Sharp,
}

impl Default for Tilt {
    fn default() -> Self { Tilt::Flat }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Parity {
    Even,
    Odd,
}

impl Default for Parity {
    fn default() -> Self { Parity::Even }
}

pub struct HexIter<T> {
    search_algo: Box<Fn(T) -> bool>,
    last_match: NodeIndex,
    field: Rc<HexGrid<T>>
}

impl<T> fmt::Debug for HexIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HexIter at {:?}", self.last_match)
    }
}

#[derive(Debug)]
pub struct HexGrid<T> {
    pub tilt: Tilt,
    pub parity: Parity,
    pub sys: CoordSys,
    graph: HexGraph<T>,
    map: HexMap,
}

impl<T> Default for HexGrid<T> {
    fn default() -> Self {
        Self {
            tilt: Tilt::default(),
            parity: Parity::default(),
            sys: CoordSys::default(),
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
            CoordSys::Offset => {
                let offset = Offset::from(coord);

                match (self.tilt, self.parity) {
                    (Tilt::Flat, Parity::Odd) => {
                        Offset::oflat_to_cube(offset)
                    }
                    (Tilt::Flat, Parity::Even) => {
                        Offset::eflat_to_cube(offset)
                    }
                    (Tilt::Sharp, Parity::Odd) => {
                        Offset::osharp_to_cube(offset)
                    }
                    (Tilt::Sharp, Parity::Even) => {
                        Offset::esharp_to_cube(offset)
                    }
                }
            }
            CoordSys::Double => {
                let double = Double::from(coord);

                match self.tilt {
                    Tilt::Flat => Double::flat_to_cube(double),
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

    fn nlink(&mut self, coord: MultiCoord) {
        match self.graph_index(coord) {
            None => (),
            Some(&own_index) => {
                let mut dir = Compass::Northeast;
                let ncoords = self.cube_from(coord).neighbors();

                for neighbor in &ncoords {
                    match self.graph_index(coord) {
                        None => (),
                        Some(&other_index) => {
                            self.graph.add_edge(
                                own_index,
                                other_index,
                                dir
                            );
                            self.graph.add_edge(
                                other_index,
                                own_index,
                                dir.inverse(),
                            );
                        }
                    }
                    dir = dir.rotate_cw(1);
                }
            }
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
    pub fn new_radial(radius: u32, blank_val: T) -> Self
    where
        T: Copy,
    {
        let mut grid = Self::default();

        if radius != 0 {
            let new_hexes = Cube::ORIGIN.spiral(radius);

            for new_hex in new_hexes {
                grid.set(MultiCoord::from(new_hex), blank_val);
            }
        }

        grid
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
    pub fn new_boxy(rows: u32, cols: u32, blank_val: T) -> Self
    where
        T: Copy,
    {
        let mut grid = Self {
            sys: CoordSys::Offset,
            ..Default::default()
        };

        for col in 0..(cols as i32) {
            for row in 0..(rows as i32) {
                let offset = Offset { col: col, row: row };
                grid.set(MultiCoord::from(offset), blank_val);
            }
        }

        grid
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

    // search iter that halts search when it finds a function match, then
    // resumes on next iter call

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
    pub fn add(&mut self, coord: MultiCoord, data: T) -> Result<(), String> {
        if self.contains_coord(coord) {
            Result::Err(
                format!("Grid already contains a value at {:?}", coord)
            )
        } else {
            self.set(coord, data);
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
    pub fn update(
        &mut self,
        coord: MultiCoord,
        data: T,
    ) -> Result<(), String> {
        if self.contains_coord(coord) {
            self.set(coord, data);
            Result::Ok(())
        } else {
            Result::Err(format!("Grid contains no value at {:?}", coord))
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
    pub fn set(&mut self, coord: MultiCoord, data: T) {
        match self.get_mut(coord) {
            Some(value) => unimplemented!(),
            _ => {
                let index = self.graph.add_node(data);
                self.map.insert(self.cube_from(coord), index);
            }
            self.nlink(coord);
        }
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

    //////////////////////////////////
    // Traversal
    //////////////////////////////////

    // map
    // iters
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
