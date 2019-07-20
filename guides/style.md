# Project Style Guide

This project firmly adheres to the following guides:
- [Formatting Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
- [Style Guide](https://doc.rust-lang.org/1.0.0/style/)

Everything below is in addition to those.

## General Rules
- Maximum line length is 79 characters. This includes comments, and there are
no exceptions.

## Sections
There are two tiers of section headers. The first spans the full line:

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
