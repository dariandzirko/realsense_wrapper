use std::simd::intrinsics;

pub use realsense_wrapper::*;

fn main() {
    if let Some(_) = black_white_example() {
        print!("black and white example passed")
    } else {
        println!("black and white example failed");
    }
}

fn black_white_example() -> Option<bool> {
    let mut realsense = RealsenseInstance::new();

    let stream_index = 0;
    let width = 640;
    let height = 480;
    let fps = 30;
    let stream = stream::Rs2StreamKind::Depth;
    let format = format::Rs2Format::Z16;

    realsense.stream_frames(stream_index, width, height, fps, stream, format);

    unsafe { rs2_get_video_stream_intrinsics(stream, intrinsics, error) }

    let mut buffer = FrameBuffer::new();
    buffer.populate_queue(&mut realsense);

    let image = buffer.get_curr_frame();
    if let Some(image_data) = image {
        let point_cloud = PointCloud::new(&image_data, intrinstics);

        if let Some(saved_pic) = image_data.to_image() {
            saved_pic.save("depth_example.png");
        } else {
            return None;
        }
    } else {
        return None;
    }

    return Some(true);
}
