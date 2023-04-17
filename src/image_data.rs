use std::slice;

use image::{DynamicImage, GrayImage, RgbImage};
use ndarray::Array2;

use crate::{
    check_error, format::Rs2Format, pthread_spinlock_t, rs2_error, rs2_frame, rs2_free_error,
    rs2_get_frame_data, FrameInfo, BITS_IN_A_BYTE,
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

    //pub fn from_ka() -> BetterRawPixel {}

    pub fn from_rgb(r: u8, g: u8, b: u8) -> BetterRawPixel {
        BetterRawPixel {
            k: (r + g + b) / 3,
            r: r,
            g: g,
            b: b,
            a: u8::MAX,
        }
    }

    //pub fn from_rgba() -> BetterRawPixel {}
    //pub fn from_lumchrom() -> BetterRawPixel {}
}

#[derive(Debug)]
pub struct ImageData {
    raw_data: Array2<u8>, //this size should be height * stride, where stride is width*bytes per pixel
    pub format: Rs2Format,
    pub width: usize,
    pub height: usize,
    pub bits_per_pixel: usize,
    pub bytes_per_pixel: usize,
    pub stride: usize,
}

impl Default for ImageData {
    fn default() -> Self {
        ImageData {
            raw_data: Array2::<u8>::zeros((480 as usize, 1920 as usize)),
            format: Rs2Format::BGR8,
            width: 640,
            height: 480,
            bits_per_pixel: 24,
            bytes_per_pixel: (24 / BITS_IN_A_BYTE) as usize,
            stride: 1920,
        }
    }
}

impl ImageData {
    pub fn new(frame_info: FrameInfo) -> ImageData {
        Self {
            raw_data: Array2::<u8>::zeros((frame_info.height as usize, frame_info.stride as usize)),
            format: frame_info.format,
            width: frame_info.width as usize,
            height: frame_info.height as usize,
            bits_per_pixel: frame_info.bits_per_pixel as usize,
            bytes_per_pixel: (frame_info.bits_per_pixel / (BITS_IN_A_BYTE)) as usize,
            stride: frame_info.stride as usize,
        }
    }

    pub unsafe fn copy_data_from_frame(&mut self, frame: *mut rs2_frame) {
        let mut error = std::ptr::null_mut::<rs2_error>();

        let frame_data = rs2_get_frame_data(frame, &mut error);

        if !frame_data.is_null() {
            match check_error(error) {
                Ok(_) => {
                    let slice = slice::from_raw_parts(
                        frame_data.cast::<u8>(),
                        self.bits_per_pixel as usize,
                    );

                    for row in 0..self.height {
                        for col in 0..self.stride {
                            self.raw_data[[row, col]] =
                                *slice.get_unchecked(row * self.stride + col);
                        }
                    }
                }
                Err(e) => e.print_error(),
            }
            rs2_free_error(error);
            drop(frame_data); //Do I even need this? Rust should just drop the pointer
        }
    }

    pub fn get_better_raw_pixel(&self, row: usize, col: usize) -> BetterRawPixel {
        match self.format {
            Rs2Format::RGB8 => {
                return BetterRawPixel::from_rgb(
                    self.raw_data[[row, col * self.bytes_per_pixel]],
                    self.raw_data[[row, col * self.bytes_per_pixel + 1]],
                    self.raw_data[[row, col * self.bytes_per_pixel + 2]],
                );
            }

            Rs2Format::Y16 => {
                //I could make this just return the u16s but I don't really think it would be all that useful in regards to
                //displayng the data
                let temp: u16 = ((self.raw_data[[row, col]] as u16) << BITS_IN_A_BYTE as u16)
                    | self.raw_data[[row, col + 1]] as u16;
                return BetterRawPixel::from_k(
                    (temp / u16::MAX * u8::MAX as u16).try_into().unwrap(),
                );
            }

            Rs2Format::Z16 => {
                //Depth data is seemingly also just in black and white
                let temp: u16 = ((self.raw_data[[row, col]] as u16) << 8)
                    | self.raw_data[[row, col + 1]] as u16;
                return BetterRawPixel::from_k(
                    (temp / u16::MAX * u8::MAX as u16).try_into().unwrap(),
                );
            }

            _ => {
                unimplemented!("I have not worked on this format yet");
            }
        }
    }

    pub fn to_luma_image(&self) -> GrayImage {
        let mut result = GrayImage::new(self.width as u32, self.height as u32);

        //Wtf is this use better raw pixel
        self.raw_data.indexed_iter().for_each(|((row, col), data)| {
            result.put_pixel(row as u32, col as u32, image::Luma::<u8>([*data]))
        });

        return result;
    }

    pub fn to_rgb_image(&self) -> RgbImage {
        let mut result = RgbImage::new(self.width as u32, self.height as u32);

        //This is wrong because the iterator will be over the bytes but I need to do every 4 bytes here because stride
        //Use better raw pixel
        self.raw_data
            .indexed_iter()
            .step_by(3)
            .for_each(|((row, col), _data)| {
                result.put_pixel(
                    (col / 3) as u32,
                    row as u32,
                    image::Rgb::<u8>([
                        self.raw_data[[row, col]],
                        self.raw_data[[row, col + 1]],
                        self.raw_data[[row, col + 2]],
                    ]),
                )
            });

        return result;
    }

    pub fn to_image(&self) -> DynamicImage {
        match self.format {
            Rs2Format::RGB8 => return image::DynamicImage::ImageRgb8(self.to_rgb_image()),

            Rs2Format::Y16 => return image::DynamicImage::ImageLuma8(self.to_luma_image()),

            Rs2Format::Z16 => return image::DynamicImage::ImageLuma8(self.to_luma_image()),

            _ => {
                unimplemented!("I have not worked on this format yet");
            }
        }
    }
}
