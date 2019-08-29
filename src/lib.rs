//! A hexagonal grid representation based upon Red Blob Games' guide.

#![crate_name = "chickenwire"]
#![crate_type = "lib"]

pub mod coordinate;
pub mod hexgrid;
pub mod prelude;

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
