use std::vec;

use crate::{rs2_intrinsics, ImageData, Point};

pub struct PointCloud {
    intrinsics: rs2_intrinsics,
    points: Vec<Point>,
}

impl PointCloud {
    //Right now should just take in a depth image and try to make a single staticpoint cloud from that
    //Then I need a function that will convert it to some file format
    //that other point cloud visuallizers can actually use
    //Image data is a fine start for this function. For now should evolve to be able to differentiate between different types of image data?
    //Also this should be a "living" struct that will constantly be updated as images are streamed in
    pub fn new(depth_image: ImageData, intrinsics: rs2_intrinsics) -> PointCloud {
        let mut temp_vector = vec![];

        //This is abusing the fact that I know the exact input to this, so not great
        depth_image
            .frame_data
            .raw_data
            .exact_chunks([2, 1])
            .into_iter()
            .enumerate()
            .for_each(|(index, chunk)| {
                let high_byte = (chunk.get(0) as u16) << 8;
                let low_byte = chunk.get(1) as u16;
                //What am i doing here, isn't this just a shift right 8 times?
                // let temp_data = (high_byte | low_byte) / u16::MAX * u8::MAX as u16;
                let temp_data = (high_byte | low_byte) >> 8;
                temp_vector.push(Point::new(
                    index % width,
                    index % height,
                    temp_data,
                    intrinsics,
                ))
            });

        PointCloud {
            intrinsics,
            points: temp_vector,
        }
    }
}
