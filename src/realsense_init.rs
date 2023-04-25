use std::collections::{self, VecDeque};

use crate::{
    bindings::*, check_error, print_device_info, FrameData, FrameInfo, ImageData, RealsenseError,
    SafeFrame,
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
    queue: collections::VecDeque<SafeFrame>,
    // next_frame: ImageData,
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
            queue: VecDeque::new(),
        }
    }

    pub fn populate_queue(
        &mut self,
        realsense: &mut RealsenseInstance,
    ) -> Result<(), RealsenseError> {
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

                self.queue.push_back(SafeFrame { frame });
            }
            rs2_release_frame(frames);
            rs2_free_error(error);
            return Ok(());
        }
    }

    pub fn get_curr_frame(&mut self) -> Option<ImageData> {
        //check if the frame_info and frame_data are valid before making ImageData
        unsafe {
            let mut frame_info = FrameInfo::default();

            if let Some(front) = self.queue.pop_front() {
                if let Ok(current) = FrameInfo::new(&front) {
                    frame_info = current;
                } else {
                    return None;
                }

                if let Ok(data) = FrameData::new(
                    &front,
                    frame_info.height as usize,
                    frame_info.stride as usize,
                    frame_info.bits_per_pixel as usize,
                ) {
                    return Some(ImageData::new(frame_info, data));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
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
