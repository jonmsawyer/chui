//! Chui: Coordinate Trainer

use coordinate_trainer::CoordinateTrainer;

fn main() {
    let mut ct = CoordinateTrainer::default();
    ct.run().expect("Could not run Coordinate Trainer")
}
