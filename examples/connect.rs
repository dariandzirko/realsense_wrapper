pub use realsense_wrapper::*;

fn main() {
    if let Some(_) = device_info_example() {
        print!("color example passed")
    } else {
        println!("color example failed");
    }
}

fn device_info_example() -> Option<bool> {
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

        rs2_delete_device(device);
        rs2_delete_device_list(device_list);
        rs2_delete_context(context);

        return Some(true);
    }
}
