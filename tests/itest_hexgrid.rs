//! Integration tests for `chickenwire::hexgrid`.

use chickenwire::prelude::*;

#[test]
fn test_radial() {
    let grid = HexGrid::new_radial(5, 0);

    assert_eq!(Some(&0), grid.get(MultiCoord::axial(0, 0)));
    assert_eq!(Some(&0), grid.get(MultiCoord::axial(1, 0)));
}
