use crate::{rs2_intrinsics, BetterRawPixel};

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // color: BetterRawPixel, //This is good concept but idk how I am gna use it
    color: u8,
}

impl Point {
    //I do not like how I am using BetterRawPixel this might not even be worth considering how I want to get the color and somehow later display it
    // pub fn new(u: f32, v: f32, pixel: BetterRawPixel, intrinsics: rs2_intrinsics) -> Point {
    pub fn new(u: usize, v: usize, pixel_value: u8, intrinsics: rs2_intrinsics) -> Point {
        // Factory function to create a pointcloud from a depth image and a camera. Given depth value d at (u, v) image coordinate, the corresponding 3d point is:

        // z = d / depth_scale

        // x = (u - cx) * z / fx
        //cx = camera center = ppx in this context
        // y = (v - cy) * z / fy
        //cy = camera center = ppy in this context

        let z = pixel_value;

        //The use of the intrinsics here is a large source of error because both the focal length x/y and image center x/y are in float which means they
        //can be negative. Meaning I do not currently know which representation of the images it uses. So center might be 0,0 or width/2 or height/2.
        //Need to figure that out lol
        let x = (u as f32 - intrinsics.ppx) * z as f32 / intrinsics.fx;
        let y = (v as f32 - intrinsics.ppy) * z as f32 / intrinsics.fy;

        Point {
            x,
            y,
            z: z as f32,
            color: z,
        }
    }
}
