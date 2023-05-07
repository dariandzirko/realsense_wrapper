use image::{DynamicImage, GrayImage, RgbImage};
use ndarray::Array2;

use crate::{format::Rs2Format, FrameData, FrameInfo, BITS_IN_A_BYTE};

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

#[derive(Debug, Default)]
pub struct ImageData {
    frame_info: FrameInfo,
    frame_data: FrameData,
}

impl ImageData {
    pub fn new(frame_info: FrameInfo, frame_data: FrameData) -> ImageData {
        Self {
            frame_info,
            frame_data,
        }
    }

    pub fn get_better_raw_pixel(&self, row: usize, col: usize) -> BetterRawPixel {
        match self.frame_info.format {
            Rs2Format::RGB8 => {
                return BetterRawPixel::from_rgb(
                    self.frame_data.raw_data
                        [[row, col * (self.frame_info.bits_per_pixel as usize)]],
                    self.frame_data.raw_data
                        [[row, col * (self.frame_info.bits_per_pixel + 1) as usize]],
                    self.frame_data.raw_data
                        [[row, col * (self.frame_info.bits_per_pixel + 2) as usize]],
                );
            }

            Rs2Format::Y16 => {
                //I could make this just return the u16s but I don't really think it would be all that useful in regards to
                //displayng the data
                let temp: u16 = ((self.frame_data.raw_data[[row, col]] as u16)
                    << BITS_IN_A_BYTE as u16)
                    | self.frame_data.raw_data[[row, col + 1]] as u16;
                return BetterRawPixel::from_k(
                    (temp / u16::MAX * u8::MAX as u16).try_into().unwrap(),
                );
            }

            Rs2Format::Z16 => {
                //Depth data is seemingly also just in black and white
                let temp: u16 = ((self.frame_data.raw_data[[row, col]] as u16) << 8)
                    | self.frame_data.raw_data[[row, col + 1]] as u16;
                return BetterRawPixel::from_k(
                    (temp / u16::MAX * u8::MAX as u16).try_into().unwrap(),
                );
            }

            _ => {
                unimplemented!("I have not worked on this format yet");
            }
        }
    }

    //This needs to eventually return a 16bit image.
    pub fn to_depth_image(&self) -> GrayImage {
        let mut result =
            GrayImage::new(self.frame_info.width as u32, self.frame_info.height as u32);
        //Wtf is this use better raw pixel
        self.frame_data
            .raw_data
            .indexed_iter()
            .for_each(|((row, col), data)| {
                result.put_pixel(
                    col as u32,
                    row as u32,
                    image::Luma::<u8>([*data]),
                    // image::Luma::<u8>([(temp_data & 0x0ff) as u8]),
                )
            });

        return result;
    }

    pub fn to_luma_image(&self) -> GrayImage {
        let mut result =
            GrayImage::new(self.frame_info.width as u32, self.frame_info.height as u32);
        //Wtf is this use better raw pixel
        self.frame_data
            .raw_data
            .indexed_iter()
            .step_by(2)
            .for_each(|((row, col), _data)| {
                result.put_pixel(
                    (col / 2) as u32,
                    row as u32,
                    image::Luma::<u8>([self.frame_data.raw_data[[row, col + 1]]]),
                    // image::Luma::<u8>([(temp_data & 0x0ff) as u8]),
                )
            });

        return result;
    }

    pub fn to_rgb_image(&self) -> RgbImage {
        let mut result = RgbImage::new(self.frame_info.width as u32, self.frame_info.height as u32);

        //This is wrong because the iterator will be over the bytes but I need to do every 4 bytes here because stride
        //Use better raw pixel
        self.frame_data
            .raw_data
            .indexed_iter()
            .step_by(3)
            .for_each(|((row, col), _data)| {
                result.put_pixel(
                    (col / 3) as u32,
                    row as u32,
                    image::Rgb::<u8>([
                        self.frame_data.raw_data[[row, col]],
                        self.frame_data.raw_data[[row, col + 1]],
                        self.frame_data.raw_data[[row, col + 2]],
                    ]),
                )
            });

        return result;
    }

    pub fn to_image(&self) -> Option<DynamicImage> {
        match self.frame_info.format {
            Rs2Format::RGB8 => return Some(image::DynamicImage::ImageRgb8(self.to_rgb_image())),

            Rs2Format::Y16 => return Some(image::DynamicImage::ImageLuma8(self.to_luma_image())),

            //Going to try to rewrite this
            Rs2Format::Z16 => return Some(image::DynamicImage::ImageLuma8(self.to_depth_image())),

            _ => {
                return None;
            }
        }
    }
}
