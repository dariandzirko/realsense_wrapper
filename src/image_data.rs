use ndarray::Array2;

pub struct BetterRawPixel {
    k: u8,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl BetterRawPixel {
    pub fn from_k(k: u8) -> BetterRawPixel {
        BetterRawPixel {
            k: k,
            r: 0,
            g: 0,
            b: 0,
            a: u8::max,
        }
    }

    pub fn from_ka() -> BetterRawPixel {}

    pub fn from_rgb(r: u8, g: u8, b: u8) -> BetterRawPixel {
        BetterRawPixel {
            k: (r + g + b) / 3,
            r: r,
            g: g,
            b: b,
            a: u8::max,
        }
    }

    pub fn from_rgba() -> BetterRawPixel {}
    pub fn from_lumchrom() -> BetterRawPixel {}
}

struct ImageData {
    raw_data: Array2<u8>, //this size should be height * stride, where stride is width*bytes per pixel
    pub format: Rs2Format,
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u32,
    pub stride: u32,
}

impl ImageData {
    pub fn new(
        format: Rs2Format,
        width: u32,
        height: u32,
        bits_per_pixel: u32,
        bytes_per_pixel: u32,
        stride: u32,
    ) -> ImageData {
        Self {
            raw_data: Array2::u8(height, stride),
            format: format,
            width: width,
            height: height,
            bits_per_pixel: bits_per_pixel,
            bytes_per_pixel: bits_per_pixel / BITS_IN_A_BYTE,
            stride: stride,
        }
    }

    pub unsafe fn copy_data_from_frame(frame: *mut rs2_frame) {
        for row in 0..height {
            for col in 0..stride {
                raw_data[[row, col]] = slice.get_unchecked((row * stride + col * bytes_per_pixel));
            }
        }
    }

    pub fn get_better_raw_pixel(row: usize, col: usize) -> BetterRawPixel {
        match format {
            Rs2Format::RGB8 => {
                return from_rgb(
                    raw_data[[row, col]],
                    raw_data[[row, col + 1]],
                    raw_data[[row, col + 2]],
                );
            }

            Rs2Format::Y16 => {
                //I could make this just return the u16s but I don't really think it would be all that useful in regards to
                //displayng the data
                let temp = ((raw_data[[row, col]] << 8) | raw_data[[row, col + 1]]);
                return from_k((temp / u16::max * u8::max));
            }

            Rs2Format::Z16 => {
                //Depth data is seemingly also just in black and white
                let temp = ((raw_data[[row, col]] << 8) | raw_data[[row, col + 1]]);
                return from_k((temp / u16::max * u8::max));
            }

            _ => {
                println!("I have not worked on this case yet")
            }
        }
    }
}
