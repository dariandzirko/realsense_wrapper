use crate::{bindings::*, ImageData};
use std::{ffi::CStr, fmt};

#[derive(Debug)]
pub struct RealsenseError {
    ty: u32,
    details: String,
}

impl RealsenseError {
    unsafe fn new(error: *mut rs2_error) -> Self {
        RealsenseError {
            ty: rs2_get_librealsense_exception_type(error),
            details: CStr::from_ptr(rs2_get_error_message(error))
                .to_str()
                .unwrap()
                .to_string(),
        }
    }
}

impl fmt::Display for RealsenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "After checking the error pointer recieved type: {} with details {}",
            self.ty, self.details
        )
    }
}

impl std::error::Error for RealsenseError {}

//Maybe want this to return an option like everything else should be
pub unsafe fn check_error(error: *mut rs2_error) -> Result<(), RealsenseError> {
    {
        if let Some(_) = error.as_ref() {
            return Err(RealsenseError::new(error));
        }
        return Ok(());
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
