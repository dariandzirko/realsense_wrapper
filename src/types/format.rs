use crate::bindings::*;
use num_derive::{FromPrimitive, ToPrimitive};

#[repr(i32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rs2Format {
    // #[doc = "< When passed to enable stream, librealsense will try to provide best suited format"]
    Any = rs2_format_RS2_FORMAT_ANY as i32,
    // #[doc = "< 16-bit linear depth values. The depth is meters is equal to depth scale * pixel value."]
    Z16 = rs2_format_RS2_FORMAT_Z16 as i32,
    // #[doc = "< 16-bit float-point disparity values. Depth->Disparity conversion : Disparity = Baseline*FocalLength/Depth."]
    Disparity16 = rs2_format_RS2_FORMAT_DISPARITY16 as i32,
    // #[doc = "< 32-bit floating point 3D coordinates."]
    XYZ32F = rs2_format_RS2_FORMAT_XYZ32F as i32,
    // #[doc = "< 32-bit y0, u, y1, v data for every two pixels. Similar to YUV422 but packed in a different order - https://en.wikipedia.org/wiki/YUV"]
    YUYV = rs2_format_RS2_FORMAT_YUYV as i32,
    // #[doc = "< 8-bit red, green and blue channels"]
    RGB8 = rs2_format_RS2_FORMAT_RGB8 as i32,
    // #[doc = "< 8-bit blue, green, and red channels -- suitable for OpenCV"]
    BGR8 = rs2_format_RS2_FORMAT_BGR8 as i32,
    // #[doc = "< 8-bit red, green and blue channels + constant alpha channel equal to FF"]
    RGBA8 = rs2_format_RS2_FORMAT_RGBA8 as i32,
    // #[doc = "< 8-bit blue, green, and red channels + constant alpha channel equal to FF"]
    BGRA8 = rs2_format_RS2_FORMAT_BGRA8 as i32,
    // #[doc = "< 8-bit per-pixel grayscale image"]
    Y8 = rs2_format_RS2_FORMAT_Y8 as i32,
    // #[doc = "< 16-bit per-pixel grayscale image"]
    Y16 = rs2_format_RS2_FORMAT_Y16 as i32,
    // #[doc = "< Four 10 bits per pixel luminance values packed into a 5-byte macropixel"]
    RAW10 = rs2_format_RS2_FORMAT_RAW10 as i32,
    // #[doc = "< 16-bit raw image"]
    RAW16 = rs2_format_RS2_FORMAT_RAW16 as i32,
    // #[doc = "< 8-bit raw image"]
    RAW8 = rs2_format_RS2_FORMAT_RAW8 as i32,
    // #[doc = "< Similar to the standard YUYV pixel format, but packed in a different order"]
    UYVY = rs2_format_RS2_FORMAT_UYVY as i32,
    // #[doc = "< Raw data from the motion sensor"]
    RAW = rs2_format_RS2_FORMAT_MOTION_RAW as i32,
    // #[doc = "< Motion data packed as 3 32-bit float values, for X, Y, and Z axis"]
    MOTION_XYZ32F = rs2_format_RS2_FORMAT_MOTION_XYZ32F as i32,
    // #[doc = "< Raw data from the external sensors hooked to one of the GPIO's"]
    GPIO_RAW = rs2_format_RS2_FORMAT_GPIO_RAW as i32,
    // #[doc = "< Pose data packed as floats array, containing translation vector, rotation quaternion and prediction velocities and accelerations vectors"]
    DOF6 = rs2_format_RS2_FORMAT_6DOF as i32,
    // #[doc = "< 32-bit float-point disparity values. Depth->Disparity conversion : Disparity = Baseline*FocalLength/Depth"]
    DISPARITY32 = rs2_format_RS2_FORMAT_DISPARITY32 as i32,
    // #[doc = "< 16-bit per-pixel grayscale image unpacked from 10 bits per pixel packed ([8:8:8:8:2222]) grey-scale image. The data is unpacked to LSB and padded with 6 zero bits"]
    Y10BPACK = rs2_format_RS2_FORMAT_Y10BPACK as i32,
    // #[doc = "< 32-bit float-point depth distance value."]
    DISTANCE = rs2_format_RS2_FORMAT_DISTANCE as i32,
    // #[doc = "< Bitstream encoding for video in which an image of each frame is encoded as JPEG-DIB"]
    MJPEG = rs2_format_RS2_FORMAT_MJPEG as i32,
    // #[doc = "< 8-bit per pixel interleaved. 8-bit left, 8-bit right."]
    Y8I = rs2_format_RS2_FORMAT_Y8I as i32,
    // #[doc = "< 12-bit per pixel interleaved. 12-bit left, 12-bit right. Each pixel is stored in a 24-bit word in little-endian order."]
    Y12I = rs2_format_RS2_FORMAT_Y12I as i32,
    // #[doc = "< multi-planar Depth 16bit + IR 10bit."]
    INZI = rs2_format_RS2_FORMAT_INZI as i32,
    // #[doc = "< 8-bit IR stream."]
    INVI = rs2_format_RS2_FORMAT_INVI as i32,
    // #[doc = "< Grey-scale image as a bit-packed array. 4 pixel data stream taking 5 bytes"]
    W10 = rs2_format_RS2_FORMAT_W10 as i32,
    // #[doc = "< Variable-length Huffman-compressed 16-bit depth values."]
    Z16H = rs2_format_RS2_FORMAT_Z16H as i32,
    // #[doc = "< 16-bit per-pixel frame grabber format."]
    FG = rs2_format_RS2_FORMAT_FG as i32,
    // #[doc = "< 12-bit per-pixel."]
    Y411 = rs2_format_RS2_FORMAT_Y411 as i32,
    // #[doc = "< 12-bit per pixel interleaved. 12-bit left, 12-bit right."]
    Y16I = rs2_format_RS2_FORMAT_Y16I as i32,
}
