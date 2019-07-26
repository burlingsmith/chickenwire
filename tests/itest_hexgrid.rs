//! Integration tests for `chickenwire::hexgrid`.

use chickenwire::coordinate::*;
use chickenwire::coordinate::axial::*;
use chickenwire::coordinate::cube::*;
use chickenwire::coordinate::double::*;
use chickenwire::coordinate::offset::*;
use chickenwire::hexgrid::*;

//////////////////////////////////////////////////////////////////////////////
// Unshaped Grids
//////////////////////////////////////////////////////////////////////////////

#[test]
fn test_empty() {
    unimplemented!();
}

//////////////////////////////////////////////////////////////////////////////
// Radial Grids
//////////////////////////////////////////////////////////////////////////////

fn radial_coord_validation(grid: &HexGrid<i32>, radius: u32) -> bool {
    let mut result = true;

    let included = Cube::ORIGIN.spiral(radius);
    let excluded = Cube::ORIGIN.ring(radius + 1);

    for coord in included {
        if !grid.contains_coord(MultiCoord::from(coord)) {
            result = false;
            break;
        }
    }

    if result {
        for coord in excluded {
            if grid.contains_coord(MultiCoord::from(coord)) {
                result = false;
                break;
            }
        }
    }

    result
}

#[test]
fn test_radial_1() {
    let grid = HexGrid::new_radial(1, 0);

    // Validate coordinate range
    assert!(
        radial_coord_validation(&grid, 1),
        "radial-1 coordinate validation"
    );

    unimplemented!();
}

#[test]
fn test_radial_5() {
    let grid = HexGrid::new_radial(5, 0);

    // Validate coordinate range
    assert!(
        radial_coord_validation(&grid, 1),
        "radial-5 coordinate validation"
    );

    unimplemented!();
}

#[test]
fn test_radial_10() {
    let grid = HexGrid::new_radial(10, 0);

    // Validate coordinate range
    assert!(
        radial_coord_validation(&grid, 1),
        "radial-10 coordinate validation"
    );

    unimplemented!();
}

//////////////////////////////////////////////////////////////////////////////
// Rectangular Grids
//////////////////////////////////////////////////////////////////////////////

fn boxy_coord_validation(grid: HexGrid<i32>, cols: i32, rows: i32) -> bool {
    let mut result = true;

    // Included coordinates
    for c in 0..cols {
        for r in 0..rows {
            if !grid.contains_coord(MultiCoord::offset(c, r)) {
                result = false;
                break;
            }
        }
    }

    // Excluded coordinates (top & bottom)
    if result {
        for col in 0..cols {
            let top = grid.contains_coord(MultiCoord::offset(col, -1));
            let bot = grid.contains_coord(MultiCoord::offset(col, rows));

            if top || bot {
                result = false;
                break;
            }
        }
    }

    // Excluded coordinates (sides)
    if result {
        for row in 0..rows {
            let left = grid.contains_coord(MultiCoord::offset(-1, row));
            let right = grid.contains_coord(MultiCoord::offset(cols, row));

            if left || right {
                result = false;
                break;
            }
        }
    }

    // Excluded coordinates (corners)
    if result {
        let ne = !grid.contains_coord(MultiCoord::offset(-1, -1));
        let se = !grid.contains_coord(MultiCoord::offset(-1, rows));
        let sw = !grid.contains_coord(MultiCoord::offset(cols, -1));
        let nw = !grid.contains_coord(MultiCoord::offset(cols, rows));

        result = ne && se && sw && nw;
    }

    result
}

#[test]
fn test_boxy_1x1() {
    let grid = HexGrid::new_boxy(1, 1, 0);

    // Validate coordinate Range
    assert!(
        boxy_coord_validation(grid, 1, 1),
        "boxy-1x1 coordinate validation"
    );

    unimplemented!();

    // Access coordinate data

    // Search for coordinate data

    // Modify values (and search again)
}

#[test]
fn test_boxy_5x5() {
    //! Create a boxy with 5 columns and 5 rows.
    //! Test boundaries.
    //! Manipulate data.

    unimplemented!();
}

#[test]
fn test_boxy_10x5() {
    //! Create a boxy with 10 columns and 5 rows.
    //! Test boundaries.
    //! Manipulate data.

    unimplemented!();
}

#[test]
fn test_boxy_5x10() {
    //! Create a boxy with 5 columns and 10 rows.
    //! Test boundaries.
    //! Manipulate data.

    unimplemented!();
}
