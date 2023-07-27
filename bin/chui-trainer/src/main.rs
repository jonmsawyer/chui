//! Chui: Coordinate Trainer

use chui_core as _;
use chui_error as _;
use chui_macros as _;
use chui_trainer::CoordinateTrainer;
use rand as _;

fn main() {
    let mut ct = CoordinateTrainer::default();
    ct.run().expect("Could not run Coordinate Trainer");
}
