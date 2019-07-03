//! An extenisible hexagonal grid structure and associated functions. Grids
//! are stored using the cube coordinate system, but conversion functions are
//! provided for offset, interlaced, and axial systems.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

//////////////////////////////////////////////////////////////////////////////
// Coordinates
//////////////////////////////////////////////////////////////////////////////

/// A sharp tilt considers the top of a hexagon to be the corner, whereas a
/// flat tilt assumes it to be an edge.
#[derive(Debug)]
enum Tilt {
    Sharp,
    Flat,
}

/// Parity determines whether the even or odd rows/columns are offset in a
/// grid using either offset or double coordinates.
#[derive(Debug)]
enum Parity {
    Even,
    Odd,
}

/// Grids using either offset or double coordinates need tilt and parity
/// information to be fully defined.
type ShapeInfo = Option<(Tilt, Parity)>;

#[derive(Debug)]
enum CoordSys {
    Offset,
    Double,
    Cube,
    Axial,
}

#[derive(Debug)]
struct Coord {
    sys: CoordSys,
    q: u32,         // Either a row value or an x value
    r: u32,         // Either a column value or a y value
}

/// Chickenwire supports the four coordinate systems addressed in Red Blob's
/// guide.
#[derive(Clone, Copy, Debug, Hash)]
enum Coord {
    Offset { row: u32, col: u32 },
    Double { row: u32, col: u32 },
    Cube { x: u32, y: u32, z: u32 },
    Axial { q: u32, r: u32 },
}

impl Coord {
    //////////////////////////////////
    // Boolean Analysis
    //////////////////////////////////

    /// State if a coordinate is in offset form.
    fn is_offset(&self) -> bool {
        match self.sys {
            Coord::Offset { .. } => true,
            _ => false,
        }
    }

    /// State if a coordinate is in double form.
    fn is_double(&self) -> bool {
        match self.sys {
            CoordSys::Double { .. } => true,
            _ => false,
        }
    }

    /// State if a coordinate is in cube form.
    fn is_cube(&self) -> bool {
        match self {
            Coord::Cube { .. } => true,
            _ => false,
        }
    }

    /// State if a coordinate is in axial form.
    fn is_axial(&self) -> bool {
        match self {
            Coord::Axial { .. } => true,
            _ => false,
        }
    }

    //////////////////////////////////
    // Conversion
    //////////////////////////////////

    /// Convert a coordinate to offset form.
    fn to_offset(&self, old_shape: ShapeInfo, new_shape: ShapeInfo) -> Self {
        unimplemented!();
    }

    /// Convert a coordinate to double form.
    fn to_double(&self, old_shape: ShapeInfo, new_shape: ShapeInfo) -> Self {
        unimplemented!();
    }

    /// Convert a coordinate to cube form.
    fn to_cube(&self, old_shape: ShapeInfo) -> Self {
        unimplemented!();
    }

    /// Convert a coordinate to axial form.
    fn to_axial(&self, old_shape: ShapeInfo) -> Self {
        unimplemented!();
    }

    /// Convert a coordinate's form to match another's.
    fn match_form(&self, template: Self) -> Self {
        match template {
            Coord::Offset { .. } => self.to_offset(),
            Coord::Double { .. } => self.to_double(),
            Coord::Cube { .. } => self.to_cube(),
            Coord::Axial { .. } => self.to_axial(),
        }
    }

    //////////////////////////////////
    // Neighbors
    //////////////////////////////////

    /// Determine the coordinates of all a hex's neighbors.
    ///
    /// Values increment clockwise. At a flat tilt, the top edge has the index
    /// zero. At a sharp tilt, it is assumed the hex rotated clockwise, so the
    /// NE edge has the zero index.
    fn neighbors(&self, shape: ShapeInfo) -> [Self; 6] {
        unimplemented!();
    }
}

//////////////////////////////////////////////////////////////////////////////
// Hexes
//////////////////////////////////////////////////////////////////////////////

type HexRef<T> = Rc<Hex<T>>;

#[derive(Debug)]
struct Hex<T> {
    pos: Coord,
    data: T,
    tilt: Tilt,
    neighbors: [Option<HexRef<T>>; 6],
}

impl<T> Hash for Hex<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl<T> Hex<T> {
    // code here //
}

//////////////////////////////////////////////////////////////////////////////
// Grids
//////////////////////////////////////////////////////////////////////////////

type HexMap<T> = HashMap<u64, HexRef<T>>;

/// A `HexGrid<T>` is essentially a graph which tracks only nodes. Each node
/// is a `Hex<T>`, which can be accessed via its neighbors or through the
/// `HexGrid<T>`'s HashMap. Hashes are generated based upon a `Hex<T>`'s
/// coordinate position.
#[derive(Debug)]
struct HexGrid<T> {
    hexes: HexMap<T>,
    offset: Parity,
    // (?) hashmap of border hexes
}

impl <T> HexGrid<T> {
    //////////////////////////////////
    // Creation
    //////////////////////////////////

    /// Initialize a grid
    fn new() -> Self {
        unimplemented!();
    }

    // a function to add/link hexes on all exposed edges of a border hex
    // maybe a function to automatically do this to all border hexes
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_is_coordsys() {
        let offset_1 = Coord::Offset { row: 1, col: 1 };
        assert!(offset_1.is_offset(), "offset_1.is_offset()");
        assert!(!offset_1.is_double(), "offset_1.is_double()");
        assert!(!offset_1.is_cube(), "offset_1.is_cube()");
        assert!(!offset_1.is_axial(), "offset_1.is_axial()");

        let offset_2 = Coord::Offset { row: 12345, col: 6789 };
        assert!(offset_2.is_offset(), "offset_2.is_offset()");
        assert!(!offset_2.is_double(), "offset_2.is_double()");
        assert!(!offset_2.is_cube(), "offset_2.is_cube()");
        assert!(!offset_2.is_axial(), "offset_2.is_axial()");

        let double_1 = Coord::Double { row: 2, col: 4 };
        assert!(!double_1.is_offset(), "double_1.is_offset()");
        assert!(double_1.is_double(), "double_1.is_double()");
        assert!(!double_1.is_cube(), "double_1.is_cube()");
        assert!(!double_1.is_axial(), "double_1.is_axial()");

        let double_2 = Coord::Double { row: 2453, col: 142 };
        assert!(!double_2.is_offset(), "double_2.is_offset()");
        assert!(double_2.is_double(), "double_2.is_double()");
        assert!(!double_2.is_cube(), "double_2.is_cube()");
        assert!(!double_2.is_axial(), "double_2.is_axial()");

        let cube_1 = Coord::Cube { x: 1, y: 2, z: 3 };
        assert!(!cube_1.is_offset(), "cube_1.is_offset()");
        assert!(!cube_1.is_double(), "cube_1.is_double()");
        assert!(cube_1.is_cube(), "cube_1.is_cube()");
        assert!(!cube_1.is_axial(), "cube_1.is_axial()");

        let cube_2 = Coord::Cube { x: 123, y: 456, z: 789 };
        assert!(!cube_2.is_offset(), "cube_2.is_offset()");
        assert!(!cube_2.is_double(), "cube_2.is_double()");
        assert!(cube_2.is_cube(), "cube_2.is_cube()");
        assert!(!cube_2.is_axial(), "cube_2.is_axial()");

        let axial_1 = Coord::Axial { q: 100, r: 200 };
        assert!(!axial_1.is_offset(), "axial_1.is_offset()");
        assert!(!axial_1.is_double(), "axial_1.is_double()");
        assert!(!axial_1.is_cube(), "axial_1.is_cube()");
        assert!(axial_1.is_axial(), "axial_1.is_axial()");

        let axial_2 = Coord::Axial { q: 42, r: 24 };
        assert!(!axial_2.is_offset(), "axial_2.is_offset()");
        assert!(!axial_2.is_double(), "axial_2.is_doubled()");
        assert!(!axial_2.is_cube(), "axial_2.is_cube()");
        assert!(axial_2.is_axial(), "axial_2.is_axial()");
    }

    #[test]
    fn test_coord_convert() {
        unimplemented!();
    }

    #[test]
    fn test_coord_neighbors() {
        unimplemented!();
        /*
        let coord_1 = Coord::Axial { q: 0, r: 0 };
        let expected_1 = [
            Coord::Axial { q: 0, r: -1 },
            Coord::Axial { q: 1, r: -1 },
            Coord::Axial { q: 1, r: 0 },
            Coord::Axial { q: 0, r: 1 },
            Coord::Axial { q: -1, r: 1 },
            Coord::Axial { q: -1, r: 0 },
        ];

        let coord_2 = Coord::Axial { q: 42, r: 144 };
        let expected_2 = [
            Coord::Axial { q: 42, r: 143 },
            Coord::Axial { q: 43, r: 143 },
            Coord::Axial { q: 43, r: 144 },
            Coord::Axial { q: 42, r: 145 },
            Coord::Axial { q: 41, r: 145 },
            Coord::Axial { q: 41, r: 144 },
        ];

        let coord_3 = Coord::Offset { row: 6, col: 4 };
        let expected_3 = [
            Coord::Offset { row: 0, col: 0 },
            Coord::Offset { row: 0, col: 0 },
            Coord::Offset { row: 0, col: 0 },
            Coord::Offset { row: 0, col: 0 },
            Coord::Offset { row: 0, col: 0 },
            Coord::Offset { row: 0, col: 0 },
        ];  // note: neighbors need parity and tilt data, so this will be a
            // grid level call

        let test_operation = |given: Coord, expected: [Coord; 6]| {
            for n in 0..6 {
                assert_eq!(
                    given.nth_neighbor(n),
                    Some(expected[n as usize]),
                    "in-bounds index"
                );
            }
            assert_eq!(given.nth_neighbor(6), None, "index of 6");
            assert_eq!(given.nth_neighbor(7), None, "index of 7");
            assert_eq!(given.nth_neighbor(100), None, "index of 100");
        };

        test_operation(coord_1, expected_1);
        test_operation(coord_2, expected_2);
        test_operation(coord_3, expected_3);
        */
    }
}
