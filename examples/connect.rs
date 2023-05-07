pub use realsense_wrapper::*;

fn main() {
    if let Some(_) = device_info_example() {
        print!("color example passed")
    } else {
        println!("color example failed");
    }
}

fn device_info_example() -> Option<bool> {
    //Very simple example this will just create a realsense object and then connect to it
    let realsense = RealsenseInstance::new();

    //If this code crashes then you do not have a realsense plugged in or I royally borked something
    return Some(true);
}
