use crate::color_map::DynamicBiLevel;
use bitvec::prelude::*;
use image::imageops::colorops::ColorMap;
use image::*;
use std::io::{Error, ErrorKind};

/// represents an image that can be printed on a dymo label printer.
pub struct PrintableImage {
    // TODO private
    pub data: Vec<[u8; 8]>,
}

impl PrintableImage {
    pub fn preview(&self) -> String {
        let mut output = String::new();
        self.data.iter().for_each(|row| {
            row.iter()
                .for_each(|byte| output.push_str(&format!("{:08b}", byte)));
            output.push('\n');
        });
        // print spaces and boxes
        output
            .replace("0", " ")
            .replace("1", &'\u{2588}'.to_string())
    }

    pub fn printable_from_grey(image: &GrayImage) -> Result<Self, Box<dyn std::error::Error>> {
        let rows = image
            .rows()
            .map(row_to_bitvec)
            .collect::<Result<Vec<[u8; 8]>, _>>()?;
        debug!("Rows: {:?}", rows);
        Ok(PrintableImage { data: rows })
    }
}

fn row_to_bitvec(
    row: image::buffer::Pixels<Luma<u8>>,
) -> Result<[u8; 8], Box<dyn std::error::Error>> {
    let bitvec: BitVec<Msb0, u8> = row.map(|pix| !is_pixel_white(*pix)).collect();
    let bytevec = &bitvec.into_vec();
    let mut result = [0_u8; 8];
    let bytes = &bytevec.as_slice()[..result.len()]; // panics if not enough data
    result.copy_from_slice(bytes);
    Ok(result)
}

fn is_pixel_white(pixel: Luma<u8>) -> bool {
    pixel.to_luma()[0] == 0xFF
}

fn prepare_image(pic: &DynamicImage) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    info!("Dimensions of image: {:?}", pic.dimensions());

    let pic = match pic.dimensions() {
        (_, 64) => pic.rotate90(),
        (64, _) => pic.to_owned(),
        _ => {
            let errortext = format!(
                "One dimension of the image must be exactly 64, not {:?}!",
                pic.dimensions()
            );
            error!("{}", errortext);
            return Err(Error::new(ErrorKind::Other, errortext).into());
        }
    };
    if pic.color().has_alpha() {
        let errortext = "image must not have transparency!".to_string();
        error!("{}", errortext);
        return Err(Error::new(ErrorKind::Other, errortext).into());
    }
    Ok(pic)
}

pub fn convert_to_bw(
    image: &DynamicImage,
    threshold: u8,
) -> Result<GrayImage, Box<dyn std::error::Error>> {
    let image = prepare_image(image)?;
    let mut gray_image = image.grayscale().into_luma8();
    gray_image
        .pixels_mut()
        .for_each(|pixel| px_to_black_or_white(pixel, threshold));
    Ok(gray_image)
}

fn px_to_black_or_white(pixel: &mut Luma<u8>, threshold: u8) {
    let colormap = DynamicBiLevel { threshold };
    colormap.map_color(pixel);
}

pub fn encode_png(image: &GrayImage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::<u8>::new();
    let encoder = image::png::PngEncoder::new(&mut buffer);
    encoder.encode(
        &image.clone().into_raw(),
        image.width(),
        image.height(),
        ColorType::L8,
    )?;
    Ok(buffer)
}

pub fn create_image(text: &str, font: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    use std::process::Command;
    let output = Command::new(crate::IM_CONVERT)
        .args(&["-background", "white"])
        .args(&["-fill", "black"])
        .args(&["-font", font])
        .args(&["-gravity", "center"])
        .args(&["-size", "x64"])
        .arg(["label:", text].join(""))
        .arg("png:-") //output png image to stdout
        .output()
        .expect("failed to execute imagemagick");
    if output.status.success() {
        let image = image::load_from_memory(&output.stdout)?;
        Ok(image)
    } else {
        error!("{}", String::from_utf8_lossy(&output.stderr));
        let errortext = "Error during imagemagick rendering.";
        Err(Error::new(ErrorKind::Other, errortext).into())
    }
}

pub fn create_bw_image(
    text: &str,
    font: &str,
    threshold: u8,
) -> Result<GrayImage, Box<dyn std::error::Error>> {
    let pic = create_image(text, font)?;
    convert_to_bw(&pic, threshold)
}

pub fn convert_memory_bw_image(
    data: &[u8],
    threshold: u8,
) -> Result<GrayImage, Box<dyn std::error::Error>> {
    let pic = image::load_from_memory(&data)?;
    convert_to_bw(&pic, threshold)
}
