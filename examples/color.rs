pub use realsense_wrapper::*;
use std::ffi::CStr;

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
    unsafe {
        let mut error = std::ptr::null_mut::<realsense_wrapper::rs2_error>();

        let context = rs2_create_context(RS2_API_VERSION as i32, &mut error);
        check_error(error);

        let device_list = rs2_query_devices(context, &mut error);

        let device_count = rs2_get_device_count(device_list, &mut error);
        check_error(error);

        if device_count == 0 {
            println!("No devices connected");
            return None;
        } else {
            println!("Device count is {}", device_count);
        }

        let device = rs2_create_device(device_list, 0, &mut error);
        check_error(error);
        print_device_info(device);

        let pipeline = rs2_create_pipeline(context, &mut error);
        check_error(error);

        let config = rs2_create_config(&mut error);
        check_error(error);

        rs2_config_enable_stream(
            config,
            rs2_stream_RS2_STREAM_COLOR,
            STREAM_INDEX as i32,
            WIDTH as i32,
            HEIGHT as i32,
            rs2_format_RS2_FORMAT_RGB8,
            FPS as i32,
            &mut error,
        );

        let pipeline_profile = rs2_pipeline_start_with_config(pipeline, config, &mut error);
        check_error(error);

        if let Some(_) = error.as_ref() {
            println!("Error with color streaming");
            return None;
        }

        let frames = rs2_pipeline_wait_for_frames(pipeline, RS2_DEFAULT_TIMEOUT, &mut error);
        check_error(error);

        let num_of_frames = rs2_embedded_frames_count(frames, &mut error);
        check_error(error);

        for i in 0..num_of_frames {
            let frame = rs2_extract_frame(frames, i, &mut error);
            check_error(error);

            println!("RGB frame arrived");
            realsense_wrapper::frame_to_image(frame);

            rs2_release_frame(frame);
        }

        rs2_release_frame(frames);

        rs2_pipeline_stop(pipeline, &mut error);
        check_error(error);

        rs2_delete_pipeline_profile(pipeline_profile);
        rs2_delete_config(config);
        rs2_delete_pipeline(pipeline);
        rs2_delete_device(device);
        rs2_delete_device_list(device_list);
        rs2_delete_context(context);

        return Some(true);
    }
}
