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
    //unimplemented!();
}

#[test]
fn test_radial_5() {
    let grid = radial_sweep_validation(5);

    // Search for data

    // Modiy values
    //unimplemented!();
}

#[test]
fn test_radial_10() {
    let grid = radial_sweep_validation(10);

    // Search for data

    // Modiy values
    //unimplemented!();
}

//////////////////////////////////////////////////////////////////////////////
// Rectangular Grids
//////////////////////////////////////////////////////////////////////////////

fn boxy_sweep_validation(cols: i32, rows: i32) -> HexGrid<i32> {
    unimplemented!();
}

#[test]
fn test_boxy_1x1() {
    unimplemented!();
}

#[test]
fn test_boxy_5x5() {
    unimplemented!();
}

#[test]
fn test_boxy_10x5() {
    unimplemented!();
}

#[test]
fn test_boxy_5x10() {
    unimplemented!();
}
