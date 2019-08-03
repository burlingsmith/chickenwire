# Project Style Guide

This project firmly adheres to the following guides:
- [Formatting Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
- [Style Guide](https://doc.rust-lang.org/1.0.0/style/)

Everything below is in addition to those.

## General Rules
- Maximum line length is 79 characters. This includes comments, and there are
no exceptions.

## `mod` and `use` Order
- All `mod` declarations go before all `use` declarations within the scope.
- Blocks are then ordered by scope (`pub use` is above `pub(crate) use` is
above `use`, etc.).
- The following blocks need to be separated by a single line, in this order:
    - Standard library crates
    - Each individual community crate
    - Each project crate module
- Items within blocks are sorted alphabetically.
- Community crate blocks are sorted alphabetically.
- Prefer not to glob on `pub use`
- Never `pub use` or `pub mod` standard are community crates.
- Sample:

```Rust
//! [ module docs ]

pub mod axial;
pub mod offset;

mod cube;
mod double;

pub use axial::Axial;

use std::collections::HashSet;
use std::iter::FromIterator;

use petgraph::graph::{node_index, NodeIndex};
use petgraph::stable_graph::StableGraph;
use petgraph::visit::Dfs;

use rand::prelude::*;

use axial::*;
use cube::*;
use double::*;
use offset::*;
```

## Headers
Section headers span the full line:

```Rust
//////////////////////////////////////////////////////////////////////////////
// Section Header
//////////////////////////////////////////////////////////////////////////////
```

This header is to be used in regions where there is no indentation, most
commonly before structure definitions, traits, method implementations, and
unit tests.

The second header is the sub-header, which goes up to the 39th character:

```Rust
    //////////////////////////////////
    // Sub-Header
    //////////////////////////////////
```

This header should be at one level of indentation, and is used to separate
method and unit testing categories.

## Documentation
Please document all of your code.
