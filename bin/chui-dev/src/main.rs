//! Main dev.

#![allow(unused_imports)]

use std::mem::{size_of, size_of_val};

use chui_core::prelude::*;
use chui_error as _;
use chui_macros as _;

fn main() {
    let coord = Coord::try_from(bitmask::G6).unwrap();
    let mask = u64::from(coord);

    println!("Coord: {}", coord);
    println!("u64 mask: {}", mask);
    println!("binary mask: {:#o}", mask);
}
