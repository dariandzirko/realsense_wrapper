use crate::bindings::*;
use crate::types::format::Rs2Format;
use crate::utils::*;
use image::*;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::mem::MaybeUninit;

pub struct FrameInfo {
    frame_number: u64,
    frame_timestamp: f64,
    frame_timestamp_domain: u32,
    frame_metadata_time_of_arrival: i64,
    format: Rs2Format,
    index: i32,
    unique_id: i32,
    frame_rate: i32,
}

pub fn get_frame_info(frame: *mut rs2_frame) -> FrameInfo {
    unsafe {
        let mut error = std::ptr::null_mut::<rs2_error>();

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

        let profile = rs2_get_frame_stream_profile(frame, &mut error);

        let mut stream = MaybeUninit::uninit();
        let mut format = MaybeUninit::uninit();
        let mut index = MaybeUninit::uninit();
        let mut unique_id = MaybeUninit::uninit();
        let mut frame_rate = MaybeUninit::uninit();

        rs2_get_stream_profile_data(
            profile,
            stream.as_mut_ptr(),
            format.as_mut_ptr(),
            index.as_mut_ptr(),
            unique_id.as_mut_ptr(),
            frame_rate.as_mut_ptr(),
            &mut error,
        );
        check_error(error);

        return FrameInfo {
            frame_number,
            frame_timestamp,
            frame_timestamp_domain,
            frame_metadata_time_of_arrival,
            format: Rs2Format::from_i32(format.assume_init() as i32).unwrap(),
            index: index.assume_init(),
            unique_id: unique_id.assume_init(),
            frame_rate: frame_rate.assume_init(),
        };
    }
}

fn frame_to_image(frame: *mut rs2_frame) {
    unsafe {
        let mut error = std::ptr::null_mut::<rs2_error>();

        let frame_data = rs2_get_frame_data(frame, &mut error);
        check_error(error);
    }
}
