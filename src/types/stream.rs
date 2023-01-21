use crate::bindings::*;
use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rs2StreamKind {
    Any = rs2_stream_RS2_STREAM_ANY,

    Depth = rs2_stream_RS2_STREAM_DEPTH,

    Color = rs2_stream_RS2_STREAM_COLOR,

    Infrared = rs2_stream_RS2_STREAM_INFRARED,

    Fisheye = rs2_stream_RS2_STREAM_FISHEYE,

    Gyro = rs2_stream_RS2_STREAM_GYRO,

    Accel = rs2_stream_RS2_STREAM_ACCEL,

    Gpio = rs2_stream_RS2_STREAM_GPIO,

    Pose = rs2_stream_RS2_STREAM_POSE,

    Confidence = rs2_stream_RS2_STREAM_CONFIDENCE,
}
