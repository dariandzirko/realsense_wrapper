use crate::bindings::*;
use crate::types::format::Rs2Format;
use crate::utils::*;
use image::*;
use ndarray::Array2;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::slice;

pub const BITS_IN_A_BYTE: i32 = 8;

#[derive(Debug)]
pub struct FrameInfo {
    frame_number: u64,
    frame_timestamp: f64,
    frame_timestamp_domain: u32,
    frame_metadata_time_of_arrival: i64,
    pub format: Rs2Format,
    index: i32,
    unique_id: i32,
    frame_rate: i32,
    pub width: i32,
    pub height: i32,
    pub bits_per_pixel: i32,
    pub stride: i32,
    data_size: i32,
}

impl FrameInfo {
    //move unsafe
    pub unsafe fn new(frame: *mut rs2_frame) -> Result<FrameInfo, RealsenseError> {
        let mut error = std::ptr::null_mut::<rs2_error>();

        let frame_number = rs2_get_frame_number(frame, &mut error);
        check_error(error)?;

        let frame_timestamp = rs2_get_frame_timestamp(frame, &mut error);
        check_error(error)?;

        let frame_timestamp_domain = rs2_get_frame_timestamp_domain(frame, &mut error);
        check_error(error)?;

        let frame_timestamp_domain_str =
            CStr::from_ptr(rs2_timestamp_domain_to_string(frame_timestamp_domain));

        let frame_metadata_time_of_arrival = rs2_get_frame_metadata(
            frame,
            rs2_frame_metadata_value_RS2_FRAME_METADATA_TIME_OF_ARRIVAL,
            &mut error,
        );
        check_error(error)?;

        let profile = rs2_get_frame_stream_profile(frame, &mut error);

        //This has high potential to be a source of error
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
        check_error(error)?;

        let width = rs2_get_frame_width(frame, &mut error);
        check_error(error)?;

        let height = rs2_get_frame_height(frame, &mut error);
        check_error(error)?;

        let bits_per_pixel = rs2_get_frame_bits_per_pixel(frame, &mut error);
        check_error(error)?;

        let stride = rs2_get_frame_stride_in_bytes(frame, &mut error);
        check_error(error)?;

        let data_size = rs2_get_frame_data_size(frame, &mut error);
        check_error(error)?;

        debug_assert_eq!(data_size, width * height * bits_per_pixel / BITS_IN_A_BYTE);

        rs2_free_error(error);

        return Ok(FrameInfo {
            frame_number,
            frame_timestamp,
            frame_timestamp_domain,
            frame_metadata_time_of_arrival,
            format: Rs2Format::from_i32(format.assume_init() as i32).unwrap(),
            //Leading to this having high potential for being a source of error
            index: index.assume_init(),
            unique_id: unique_id.assume_init(),
            frame_rate: frame_rate.assume_init(),
            width,
            height,
            bits_per_pixel,
            stride,
            data_size,
        });
    }
}

impl Default for FrameInfo {
    fn default() -> Self {
        FrameInfo {
            width: 640,
            height: 480,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct FrameData {
    pub raw_data: Array2<u8>, //this size should be height * stride, where stride is width*bytes per pixel
    pub height: usize,
    pub stride: usize,
}

impl FrameData {
    pub unsafe fn new(
        frame: *mut rs2_frame,
        height: usize,
        stride: usize,
        bits_per_pixel: usize,
    ) -> Result<FrameData, RealsenseError> {
        let mut error = std::ptr::null_mut::<rs2_error>();

        let frame_data = rs2_get_frame_data(frame, &mut error);

        check_error(error)?;

        let slice = slice::from_raw_parts(frame_data.cast::<u8>(), bits_per_pixel);
        let mut raw_data = Array2::<u8>::zeros((height, stride));

        for row in 0..height {
            for col in 0..stride {
                raw_data[[row, col]] = *slice.get_unchecked(row * stride + col);
            }
        }
        rs2_free_error(error);
        drop(frame_data); //Do I even need this? Rust should just drop the pointer
        return Ok(FrameData {
            raw_data: raw_data,
            height: height,
            stride: stride,
        });
    }
}

impl Default for FrameData {
    fn default() -> Self {
        FrameData {
            raw_data: Array2::<u8>::zeros((480, 640)),
            height: 480,
            stride: 640,
        }
    }
}
