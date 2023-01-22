use crate::bindings::*;
use crate::types::format::Rs2Format;
use crate::utils::*;
use image::*;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::slice;

pub const BITS_IN_A_BYTE: i32 = 8;

pub struct FrameInfo {
    frame_number: u64,
    frame_timestamp: f64,
    frame_timestamp_domain: u32,
    frame_metadata_time_of_arrival: i64,
    format: Rs2Format,
    index: i32,
    unique_id: i32,
    frame_rate: i32,
    width: i32,
    height: i32,
    bits_per_pixel: i32,
    stride: i32,
    data_size: i32,
}

pub unsafe fn get_frame_info(frame: *mut rs2_frame) -> FrameInfo {
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

    let width = rs2_get_frame_width(frame, &mut error);
    check_error(error);

    let height = rs2_get_frame_height(frame, &mut error);
    check_error(error);

    let bits_per_pixel = rs2_get_frame_bits_per_pixel(frame, &mut error);
    check_error(error);

    let stride = rs2_get_frame_stride_in_bytes(frame, &mut error);
    check_error(error);

    let data_size = rs2_get_frame_data_size(frame, &mut error);
    check_error(error);

    debug_assert_eq!(data_size, width * height * bits_per_pixel / BITS_IN_A_BYTE);

    return FrameInfo {
        frame_number,
        frame_timestamp,
        frame_timestamp_domain,
        frame_metadata_time_of_arrival,
        format: Rs2Format::from_i32(format.assume_init() as i32).unwrap(),
        index: index.assume_init(),
        unique_id: unique_id.assume_init(),
        frame_rate: frame_rate.assume_init(),
        width,
        height,
        bits_per_pixel,
        stride,
        data_size,
    };
}

fn frame_to_image(frame: *mut rs2_frame) {
    unsafe {
        let mut error = std::ptr::null_mut::<rs2_error>();

        let frame_info = get_frame_info(frame);
        check_error(error);

        let frame_data = rs2_get_frame_data(frame, &mut error);
        check_error(error);

        let slice =
            slice::from_raw_parts(frame_data.cast::<u8>(), frame_info.bits_per_pixel as usize);

        //Change this to a dynamic image eventually
        //Actually do something with this match statement
        match frame_info.format {
            RGB8 => {}
            _ => println!("I have not worked on this case yet"),
        }

        //Can change this loop to populating variable channels 1,2,3,4 and then use those in the correct order based on the format
        let mut image = image::RgbImage::new(frame_info.width as u32, frame_info.height as u32);
        for row in 0..frame_info.height {
            for col in 0..frame_info.width {
                let r = slice.get_unchecked((row * frame_info.stride + col * 3) as usize);
                let g = slice.get_unchecked((row * frame_info.stride + col * 3 + 1) as usize);
                let b = slice.get_unchecked((row * frame_info.stride + col * 3 + 2) as usize);

                let temp_pixel = image::Rgb([*r, *g, *b]);
                image.put_pixel(col as u32, row as u32, temp_pixel);
            }
        }
    }
}

//     format: Rs2Format,
// data_size_in_bytes: usize,
// data: *const c_void,
// stride_in_bytes: usize,
// col: usize,
// row: usize,

// let slice = slice::from_raw_parts(data.cast::<u8>(), data_size_in_bytes);
// let offset = (row * stride_in_bytes) + (col * 3);

// PixelKind::Bgr8 {
//     b: slice.get_unchecked(offset),
//     g: slice.get_unchecked(offset + 1),
//     r: slice.get_unchecked(offset + 2),

//         let timestamp_domain =
//             sys::rs2_get_frame_timestamp_domain(frame_ptr.as_ptr(), &mut err);
//         check_rs2_error!(err, FrameConstructionError::CouldNotGetTimestampDomain)?;

//         let profile_ptr = sys::rs2_get_frame_stream_profile(frame_ptr.as_ptr(), &mut err);
//         check_rs2_error!(err, FrameConstructionError::CouldNotGetFrameStreamProfile)?;

//         let nonnull_profile_ptr =
//             NonNull::new(profile_ptr as *mut sys::rs2_stream_profile).unwrap();
//         let profile = StreamProfile::try_from(nonnull_profile_ptr)?;

//         let data_ptr = sys::rs2_get_frame_data(frame_ptr.as_ptr(), &mut err);
//         check_rs2_error!(err, FrameConstructionError::CouldNotGetData)?;

//         let nonnull_data_ptr = NonNull::new(data_ptr as *mut std::os::raw::c_void).unwrap();

//         Ok(ImageFrame {
//             frame_ptr,
//             width: width as usize,
//             height: height as usize,
//             stride: stride as usize,
//             bits_per_pixel: bits_per_pixel as usize,
//             timestamp,
//             timestamp_domain: Rs2TimestampDomain::from_i32(timestamp_domain as i32).unwrap(),
//             frame_number,
//             frame_stream_profile: profile,
//             data_size_in_bytes: size as usize,
//             data: nonnull_data_ptr,
//             should_drop: true,
//             _phantom: PhantomData::<K> {},
//         })
//     }
// }
// }
