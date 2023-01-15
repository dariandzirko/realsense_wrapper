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

        loop {
            let frames = rs2_pipeline_wait_for_frames(pipeline, RS2_DEFAULT_TIMEOUT, &mut error);
            check_error(error);

            let num_of_frames = rs2_embedded_frames_count(frames, &mut error);
            check_error(error);

            for i in 0..num_of_frames {
                let frame = rs2_extract_frame(frames, i, &mut error);
                check_error(error);

                let frame_data = rs2_get_frame_data(frame, &mut error);
                check_error(error);

                let frame_number = rs2_get_frame_number(frame, &mut error);
                check_error(error);

                let frame_timestamp = rs2_get_frame_timestamp(frame, &mut error);
                check_error(error);

                let frame_timestamp_domain = rs2_get_frame_timestamp_domain(frame, &mut error);
                check_error(error);

                let frame_timestamp_domain_str =
                    CStr::from_ptr(rs2_timestamp_domain_to_string(frame_timestamp_domain));

                let frame_metadata_time_of_arrival = rs2_get_frame_metadata(
                    frame,
                    rs2_frame_metadata_value_RS2_FRAME_METADATA_TIME_OF_ARRIVAL,
                    &mut error,
                );
                check_error(error);

                println!("RGB frame arrived");
                println!("The first 10 bytes: ");
                for i in 0..10 {
                    println!("{}", frame_data[i]);
                }
                println!("Frame number: {}", frame_number);
                println!("Timestamp: {}", frame_timestamp);
                println!("Timestamp domain: {:?}", frame_timestamp_domain_str);
                println!("Time of arrival: {}", frame_metadata_time_of_arrival);
                rs2_release_frame(frame);
            }

            rs2_release_frame(frames);
        }

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

unsafe fn check_error(error: *mut rs2_error) {
    {
        if let Some(_) = error.as_ref() {
            println!(
                "Exception type: {} with message {:?}",
                rs2_get_librealsense_exception_type(error),
                CStr::from_ptr(rs2_get_error_message(error))
            );
        }
    }
}

unsafe fn print_device_info(device: *mut rs2_device) {
    let mut error = std::ptr::null_mut::<realsense_wrapper::rs2_error>();

    println!(
        "Using device 0: {:?} Serial number: {:?}, Firmware version: {:?}",
        CStr::from_ptr(rs2_get_device_info(
            device,
            rs2_camera_info_RS2_CAMERA_INFO_NAME,
            &mut error
        )),
        CStr::from_ptr(rs2_get_device_info(
            device,
            rs2_camera_info_RS2_CAMERA_INFO_SERIAL_NUMBER,
            &mut error
        )),
        CStr::from_ptr(rs2_get_device_info(
            device,
            rs2_camera_info_RS2_CAMERA_INFO_FIRMWARE_VERSION,
            &mut error
        ))
    );
}

// void print_device_info(rs2_device* dev)
// {
//     rs2_error* e = 0;
//     printf("\nUsing device 0, an %s\n", rs2_get_device_info(dev, RS2_CAMERA_INFO_NAME, &e));
//     check_error(e);
//     printf("    Serial number: %s\n", rs2_get_device_info(dev, RS2_CAMERA_INFO_SERIAL_NUMBER, &e));
//     check_error(e);
//     printf("    Firmware version: %s\n\n", rs2_get_device_info(dev, RS2_CAMERA_INFO_FIRMWARE_VERSION, &e));
//     check_error(e);
// }
