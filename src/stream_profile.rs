pub struct StreamProfile {
    //stream: Rs2StreamKind,
    format: Rs2Format,
    index: i32,
    unique_id: i32,
    frame_rate: i32,
    is_default: bool,
    should_drop: bool,
}
