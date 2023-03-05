use crate::{bindings::*, check_error, get_frame_info, print_device_info, ImageData};

pub struct RealsenseInstance {
    error: *mut rs2_error,
    context: *mut rs2_context,
    device: *mut rs2_device,
    pipeline: *mut rs2_pipeline,
    config: *mut rs2_config,
}

pub struct FrameBuffer {
    curr_frame: *mut rs2_frame,
    next_frame: *mut rs2_frame,
}

impl RealsenseInstance {
    //This might want to take in index, in the case of more than 1 device
    fn new() -> Self {
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

            RealsenseInstance {
                error: error,
                context: context,
                device: device,
                pipeline: pipeline,
                config: config,
            }
        }
    }

    //This should take in the parameters to configure the stream
    fn start_frame_stream(
        &mut self,
        stream_index: i32,
        width: i32,
        height: i32,
        fps: i32,
        stream: rs2_stream, //rs2_stream_RS2_STREAM_COLOR//
        format: rs2_format, //rs2_format_RS2_FORMAT_RGB8//
    ) {
        unsafe {
            rs2_config_enable_stream(
                self.config,
                stream,
                stream_index,
                width,
                height,
                format,
                fps,
                &mut self.error,
            );
        }
    }
}

impl FrameBuffer {
    fn new() -> Self {
        FrameBuffer {
            curr_frame: std::ptr::null_mut::<rs2_frame>(),
            next_frame: std::ptr::null_mut::<rs2_frame>(),
        }
    }

    fn populate_frames(&mut self, realsense: &mut RealsenseInstance) {
        unsafe {
            let pipeline_profile = rs2_pipeline_start_with_config(
                realsense.pipeline,
                realsense.config,
                &mut realsense.error,
            );
            check_error(realsense.error);

            while (true) {
                let frames = rs2_pipeline_wait_for_frames(
                    realsense.pipeline,
                    RS2_DEFAULT_TIMEOUT,
                    &mut realsense.error,
                );
                check_error(realsense.error);

                let num_of_frames = rs2_embedded_frames_count(frames, &mut realsense.error);
                check_error(realsense.error);

                for i in 0..num_of_frames {
                    let frame = rs2_extract_frame(frames, i, &mut realsense.error);
                    check_error(realsense.error);

                    self.swap_frames(frame);

                    rs2_release_frame(frame);
                }
                rs2_release_frame(frames);
            }
            rs2_delete_pipeline_profile(pipeline_profile);
        }
    }

    fn swap_frames(&mut self, curr_frame: *mut rs2_frame) {
        unsafe {
            rs2_release_frame(self.next_frame);
        }
        self.next_frame = self.curr_frame;
        self.curr_frame = curr_frame;
    }

    fn get_curr_frame(&self) -> ImageData {
        unsafe {
            let frame_info = get_frame_info(self.curr_frame); //should probably just give the struct the frame info and extract all the data
            let mut frame_data = ImageData::new(
                frame_info.format,
                frame_info.width as usize,
                frame_info.height as usize,
                frame_info.bits_per_pixel as usize,
                frame_info.stride as usize,
            );

            frame_data.copy_data_from_frame(self.curr_frame);

            return frame_data;
        }
    }
}

impl Drop for RealsenseInstance {
    fn drop(&mut self) {
        unsafe {
            rs2_pipeline_stop(self.pipeline, &mut self.error);
            check_error(self.error);

            rs2_delete_config(self.config);
            rs2_delete_pipeline(self.pipeline);
            rs2_delete_device(self.device);
            rs2_delete_context(self.context);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            rs2_release_frame(self.curr_frame);
            rs2_release_frame(self.next_frame);
        }
    }
}
