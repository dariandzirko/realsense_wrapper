use crate::{rs2_intrinsics, BetterRawPixel};

struct Point {
    x: usize,
    y: usize,
    z: usize,
    color: BetterRawPixel, //This is good concept but idk how I am gna use it
}

impl Point {
    pub fn new(u: usize, v: usize, pixel: BetterRawPixel, intrinsics: rs2_intrinsics) {}
}
