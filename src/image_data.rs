use std::slice;

use ndarray::Array2;

use crate::{
    check_error, format::Rs2Format, get_frame_info, rs2_error, rs2_frame, rs2_get_frame_data,
    BITS_IN_A_BYTE,
};

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
            a: u8::MAX,
        }
    }

    pub fn from_ka() -> BetterRawPixel {}

    pub fn from_rgb(r: u8, g: u8, b: u8) -> BetterRawPixel {
        BetterRawPixel {
            k: (r + g + b) / 3,
            r: r,
            g: g,
            b: b,
            a: u8::MAX,
        }
    }

    pub fn from_rgba() -> BetterRawPixel {}
    pub fn from_lumchrom() -> BetterRawPixel {}
}

struct ImageData {
    raw_data: Array2<u8>, //this size should be height * stride, where stride is width*bytes per pixel
    pub format: Rs2Format,
    pub width: usize,
    pub height: usize,
    pub bits_per_pixel: usize,
    pub bytes_per_pixel: usize,
    pub stride: usize,
}

impl ImageData {
    pub fn new(
        format: Rs2Format,
        width: usize,
        height: usize,
        bits_per_pixel: usize,
        bytes_per_pixel: usize,
        stride: usize,
    ) -> ImageData {
        Self {
            raw_data: Array2::u8(height, stride),
            format: format,
            width: width,
            height: height,
            bits_per_pixel: bits_per_pixel,
            bytes_per_pixel: bits_per_pixel / (BITS_IN_A_BYTE as usize),
            stride: stride,
        }
    }

    pub unsafe fn copy_data_from_frame(&mut self, frame: *mut rs2_frame) {
        let mut error = std::ptr::null_mut::<rs2_error>();

        let frame_info = get_frame_info(frame);
        check_error(error);

        let frame_data = rs2_get_frame_data(frame, &mut error);
        check_error(error);

        let slice = slice::from_raw_parts(frame_data.cast::<u8>(), self.bits_per_pixel as usize);

        for row in 0..self.height {
            for col in 0..self.stride {
                self.raw_data[[row, col]] =
                    *slice.get_unchecked(row * self.stride + col * self.bytes_per_pixel);
            }
        }
    }

    pub fn get_better_raw_pixel(&self, row: usize, col: usize) -> BetterRawPixel {
        match self.format {
            Rs2Format::RGB8 => {
                return BetterRawPixel::from_rgb(
                    self.raw_data[[row, col]],
                    self.raw_data[[row, col + 1]],
                    self.raw_data[[row, col + 2]],
                );
            }

            Rs2Format::Y16 => {
                //I could make this just return the u16s but I don't really think it would be all that useful in regards to
                //displayng the data
                let temp: u16 = ((self.raw_data[[row, col]] as u16) << BITS_IN_A_BYTE as u16)
                    | self.raw_data[[row, col + 1]] as u16;
                return BetterRawPixel::from_k(temp / u16::MAX * u8::MAX as u16);
            }

            Rs2Format::Z16 => {
                //Depth data is seemingly also just in black and white
                let temp: u16 = ((self.raw_data[[row, col]] as u16) << 8)
                    | self.raw_data[[row, col + 1]] as u16;
                return BetterRawPixel::from_k((temp / u16::MAX * u8::MAX));
            }

            _ => {
                unimplemented!("I have not worked on this format yet");
            }
        }
    }
}
