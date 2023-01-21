pub struct StreamProfile {
    //stream: Rs2StreamKind,
    format: Rs2Format,
    index: i32,
    unique_id: i32,
    frame_rate: i32,
    is_default: bool,
    should_drop: bool,
}

// ptr: stream_profile_ptr,
// stream: Rs2StreamKind::from_i32(stream.assume_init() as i32).unwrap(),
// format: Rs2Format::from_i32(format.assume_init() as i32).unwrap(),
// index: index.assume_init() as usize,
// unique_id: unique_id.assume_init(),
// framerate: framerate.assume_init(),
// is_default: is_default != 0,
// should_drop: false,
