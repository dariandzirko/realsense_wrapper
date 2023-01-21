pub enum Rs2StreamKind {
    Any = rs2_stream_RS2_STREAM_ANY as i32,

    Depth = rs2_stream_RS2_STREAM_DEPTH as i32,

    Color = rs2_stream_RS2_STREAM_COLOR as i32,

    Infrared = rs2_stream_RS2_STREAM_INFRARED as i32,

    Fisheye = rs2_stream_RS2_STREAM_FISHEYE as i32,

    Gyro = rs2_stream_RS2_STREAM_GYRO as i32,

    Accel = rs2_stream_RS2_STREAM_ACCEL as i32,

    Gpio = rs2_stream_RS2_STREAM_GPIO as i32,

    Pose = rs2_stream_RS2_STREAM_POSE as i32,

    Confidence = rs2_stream_RS2_STREAM_CONFIDENCE as i32,
}
