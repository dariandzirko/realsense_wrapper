use crate::{
    bindings::*, check_error, get_frame_info, print_device_info, ImageData, RealsenseError,
};

pub struct RealsenseInstance {
    pub context: *mut rs2_context,
    pub device: *mut rs2_device,
    pub pipeline: *mut rs2_pipeline,
    pub config: *mut rs2_config,
    pub pipeline_profile: *mut rs2_pipeline_profile,
}

unsafe impl Sync for RealsenseInstance {}

unsafe impl Send for RealsenseInstance {}

pub struct FrameBuffer {
    curr_frame: *mut rs2_frame,
    next_frame: *mut rs2_frame,
    // curr_data: ImageData,
    // next_data: ImageData,
}

unsafe impl Sync for FrameBuffer {}

unsafe impl Send for FrameBuffer {}

impl RealsenseInstance {
    //This might want to take in index, in the case of more than 1 device
    pub fn new() -> Self {
        unsafe {
            let mut error = std::ptr::null_mut::<rs2_error>();

            let context = rs2_create_context(RS2_API_VERSION as i32, &mut error);
            check_error(error);

            let device_list = rs2_query_devices(context, &mut error);

            let device_count = rs2_get_device_count(device_list, &mut error);
            check_error(error);

            if device_count == 0 {
                println!("No devices connected");
                //return error
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

            rs2_delete_device_list(device_list); //might cause errors if you delete the device_list but keep the device
            rs2_free_error(error);

            RealsenseInstance {
                context: context,
                device: device,
                pipeline: pipeline,
                config: config,
                pipeline_profile: std::ptr::null_mut::<rs2_pipeline_profile>(),
            }
        }
    }

    pub fn stream_frames(
        &mut self,
        stream_index: i32,
        width: i32,
        height: i32,
        fps: i32,
        stream: rs2_stream, //rs2_stream_RS2_STREAM_COLOR//
        format: rs2_format, //rs2_format_RS2_FORMAT_RGB8//
    ) {
        unsafe {
            let mut error = std::ptr::null_mut::<rs2_error>();

            rs2_config_enable_stream(
                self.config,
                stream,
                stream_index,
                width,
                height,
                format,
                fps,
                &mut error,
            );

            self.pipeline_profile =
                rs2_pipeline_start_with_config(self.pipeline, self.config, &mut error);
            check_error(error);
            rs2_free_error(error);
        }
    }

    //This should take in the parameters to configure the stream
}

impl FrameBuffer {
    //please don't call get_curr_frame before calling stream_frames
    pub fn new() -> Self {
        FrameBuffer {
            curr_frame: std::ptr::null_mut::<rs2_frame>(),
            next_frame: std::ptr::null_mut::<rs2_frame>(),
        }
    }

    pub fn pull_frame(&mut self, realsense: &mut RealsenseInstance) -> Result<(), RealsenseError> {
        unsafe {
            let mut error = std::ptr::null_mut::<rs2_error>();

            let frames =
                rs2_pipeline_wait_for_frames(realsense.pipeline, RS2_DEFAULT_TIMEOUT, &mut error);
            check_error(error)?;

            //This num_frame is something worth investigating
            let num_of_frames = rs2_embedded_frames_count(frames, &mut error);
            check_error(error)?;

            for i in 0..num_of_frames {
                let frame = rs2_extract_frame(frames, 0, &mut error);

                check_error(error)?;

                self.swap_frames(frame);
                rs2_release_frame(frame);
            }
            rs2_release_frame(frames);
            rs2_free_error(error);
            println!("end of pull frame");
        }
    }

    pub fn get_curr_frame(&self) -> Option<ImageData> {
        //check if the frame_info and frame_data are valid before making ImageData
        unsafe {
            match get_frame_info(self.curr_frame) {
                Ok(info) => {
                    return Some(ImageData::new(info, frame_data));
                    // return Some(ImageData::new(info, frame_data).copy_data_from_frame(self.curr_frame))
                }
                Err(_) => match get_frame_info(self.next_frame) {
                    Ok(info) => {
                        // return Some(ImageData::new(info).copy_data_from_frame(self.next_frame))
                        return Some(ImageData::new(info, frame_data));
                    }
                    Err(_) => return None,
                },
            }
        }
    }

    //This is just some move the buffer up functin to be called after you update curr_frame
    // fn swap_data(&mut self) {
    //     self.next_data = self.curr_data.;
    // }

    //This was wrong when I fed it a bad frame. The next_frame was dropped
    //and then the curr_frame was invalid
    //curr_frame isn't null but I do not think it is valid
    fn swap_frames(&mut self, curr_frame: *mut rs2_frame) {
        unsafe {
            rs2_release_frame(self.next_frame);
        }
        self.next_frame = self.curr_frame;
        self.curr_frame = curr_frame;
    }
}

impl Drop for RealsenseInstance {
    fn drop(&mut self) {
        unsafe {
            println!("Drop for RealsenseInstance");

            let mut error = std::ptr::null_mut::<rs2_error>();

            rs2_pipeline_stop(self.pipeline, &mut error);
            check_error(error);
            rs2_delete_pipeline_profile(self.pipeline_profile);
            rs2_delete_config(self.config);
            rs2_delete_pipeline(self.pipeline);
            rs2_delete_device(self.device);
            rs2_delete_context(self.context);
            rs2_free_error(error);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            println!("Drop for FrameBuffer");
            rs2_release_frame(self.curr_frame);
            rs2_release_frame(self.next_frame);
        }
    }
}
