//! Integration tests for `chickenwire::hexgrid`.

use chickenwire::coordinate::*;
use chickenwire::coordinate::axial::*;
use chickenwire::coordinate::cube::*;
use chickenwire::coordinate::double::*;
use chickenwire::coordinate::offset::*;
use chickenwire::hexgrid::*;

//////////////////////////////////////////////////////////////////////////////
//
//////////////////////////////////////////////////////////////////////////////

#[test]
fn test_empty() {
    unimplemented!();
}

#[test]
fn test_radial_1() {
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

#[test]
fn test_boxy_1x1() {
    //! Create a boxy with 1 column and 1 row.
    //! Test boundaries.
    //! Manipulate data.

    // 1 column, 1 row Offset hexagonal HexGrid with (Flat, Odd) layout
    let grid = HexGrid::new_boxy(1, 1, 0);

    // Expected coordinates
    assert!(
        grid.contains_coord(MultiCoord::from(Offset::ORIGIN)),
        "doesn't contain expected coordinate"
    );

    // Unexpected (surrounding) coordinates
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: 0, row: 1 })),
        "contains unexpected coordinate"
    );
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: 1, row: 1 })),
        "contains unexpected coordinate"
    );
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: 1, row: 0 })),
        "contains unexpected coordinate"
    );
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: 1, row: -1 })),
        "contains unexpected coordinate"
    );
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: 0, row: -1 })),
        "contains unexpected coordinate"
    );
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: -1, row: -1 })),
        "contains unexpected coordinate"
    );
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: -1, row: 0 })),
        "contains unexpected coordinate"
    );
    assert!(
        !grid.contains_coord(MultiCoord::from(Offset { col: -1, row: 1 })),
        "contains unexpected coordinate"
    );

    // Access coordinate data
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
