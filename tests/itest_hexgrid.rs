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

fn radial_sweep_validation(radius: u32) -> HexGrid<i32> {
    let grid = HexGrid::new_radial(radius, 0);

    let included = Cube::ORIGIN.spiral(radius);
    let excluded = Cube::ORIGIN.ring(radius + 1);

    for coord in included {
        assert!(
            grid.contains_coord(MultiCoord::from(coord)),
            "expected coordinate presence in radial grid validation"
        );
        assert_eq!(
            grid.get(MultiCoord::from(coord)),
            Some(&0),
            "expected 'Some(0)' in radial grid validation"
        );
    }

    for coord in excluded {
        assert!(
            !grid.contains_coord(MultiCoord::from(coord)),
            "expected coordinate absence in radial grid validation"
        );
        assert_eq!(
            grid.get(MultiCoord::from(coord)),
            None,
            "expected 'None' in radial grid validation"
        );
    }

    grid
}

#[test]
fn test_radial_1() {
    let grid = radial_sweep_validation(1);

    // Search for data

    // Modiy values
    unimplemented!();
}

#[test]
fn test_radial_5() {
    unimplemented!();
}

#[test]
fn test_radial_10() {
    unimplemented!();
}

//////////////////////////////////////////////////////////////////////////////
// Rectangular Grids
//////////////////////////////////////////////////////////////////////////////

fn boxy_contains_coord_validation(grid: HexGrid<i32>, cols: i32, rows: i32) -> bool {
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
        boxy_contains_coord_validation(grid, 1, 1),
        "boxy-1x1 coordinate validation"
    );

    unimplemented!();

    // Access coordinate data

    // Search for coordinate data

    // Modify values (and search again)
}

#[test]
fn test_boxy_5x5() {
    let grid = HexGrid::new_boxy(5, 5, 0);

    // Validate coordinate Range
    assert!(
        boxy_contains_coord_validation(grid, 5, 5),
        "boxy-5x5 coordinate validation"
    );

    unimplemented!();

    // Access coordinate data

    // Search for coordinate data

    // Modify values (and search again)
}

#[test]
fn test_boxy_10x5() {
    let grid = HexGrid::new_boxy(10, 5, 0);

    // Validate coordinate Range
    assert!(
        boxy_contains_coord_validation(grid, 10, 5),
        "boxy-10x5 coordinate validation"
    );

    unimplemented!();

    // Access coordinate data

    // Search for coordinate data

    // Modify values (and search again)
}

#[test]
fn test_boxy_5x10() {
    let grid = HexGrid::new_boxy(5, 10, 0);

    // Validate coordinate Range
    assert!(
        boxy_contains_coord_validation(grid, 5, 10),
        "boxy-5x10 coordinate validation"
    );

    unimplemented!();

    // Access coordinate data

    // Search for coordinate data

    // Modify values (and search again)
}
