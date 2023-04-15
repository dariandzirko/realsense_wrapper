pub use realsense_wrapper::*;

//rs2_stream_RS2_STREAM_COLOR
//rs2_format_RS2_FORMAT_RGB8
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FPS: u32 = 30;
const STREAM_INDEX: u32 = 0;

fn main() {
    if let Some(_) = color_example() {
        print!("color example passed")
    } else {
        println!("color example failed");
    }
}

fn color_example() -> Option<bool> {
    let mut realsense = RealsenseInstance::new();
    let mut frame_buffer = FrameBuffer::new();
    let mut error = std::ptr::null_mut::<rs2_error>();

    let stream_index = 0;
    let width = 640;
    let height = 480;
    let fps = 30;
    let stream = rs2_stream_RS2_STREAM_COLOR;
    let format = rs2_format_RS2_FORMAT_RGB8;

    unsafe {
        rs2_config_enable_stream(
            realsense.config,
            stream,
            stream_index,
            width,
            height,
            format,
            fps,
            &mut error,
        );

        let pipeline_profile =
            rs2_pipeline_start_with_config(realsense.pipeline, realsense.config, &mut error);
        check_error(error);
    }

    frame_buffer.pull_frame(&mut realsense);
    frame_buffer
        .get_curr_frame()
        .to_image()
        .save("image_data_rgb.png");
    return Some(true);
}
