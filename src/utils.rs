use crate::bindings::*;
use std::ffi::CStr;

//Maybe want this to return an option like everything else should be
pub unsafe fn check_error(error: *mut rs2_error) -> bool {
    {
        if let Some(_) = error.as_ref() {
            println!(
                "Exception type: {} with message {:?}",
                rs2_get_librealsense_exception_type(error),
                CStr::from_ptr(rs2_get_error_message(error))
            );
            return true;
        }
        println!("No error");
        return false;
    }
}

pub unsafe fn print_device_info(device: *mut rs2_device) {
    let mut error = std::ptr::null_mut::<rs2_error>();

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
    rs2_free_error(error);
}
