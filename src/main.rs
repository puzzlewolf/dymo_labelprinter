use image::imageops::colorops::ColorMap;
use image::*;

use bitvec::prelude::*;

use dymo_hid::color_map::DynamicBiLevel;
use dymo_hid::command_accumulator::CommandAccumulator;
use dymo_hid::printable_image::PrintableImage;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pic = image::open("testdata/bold.png")?;
    if pic.dimensions().1 != 64 {
        let errortext = format!(
            "Height of image must be exactly 64, not {}!",
            pic.dimensions().1
        );
        return Err(Error::new(ErrorKind::Other, errortext))?;
    }
    if pic.color().has_alpha() {
        let errortext = format!("image must not have transparency!");
        return Err(Error::new(ErrorKind::Other, errortext))?;
    }
    println!("Dimensions of image: {:?}", pic.dimensions());
    let pic = pic.rotate90();
    let new_pic = convert_to_bw(&pic).unwrap();
    new_pic.save("output/output.png")?;
    print_file(&new_pic)?;
    Ok(())
}

fn print_file(image: &GrayImage) -> Result<(), Box<dyn std::error::Error>> {
    let pi = grey_to_printable(image)?;
    print!("{}", pi.preview());
    let mut ca = CommandAccumulator::new();
    ca.generate_commands(&pi);
    let commands = ca.accu;
    let mut f = File::create("commands")?;
    f.write_all(commands.as_slice())?;
    Ok(())
}

fn convert_to_bw(image: &DynamicImage) -> ImageResult<GrayImage> {
    let mut gray_image = image.grayscale().into_luma();
    gray_image
        .pixels_mut()
        .for_each(|pix| px_to_black_or_white(pix));
    Ok(gray_image)
}

fn px_to_black_or_white(pix: &mut Luma<u8>) {
    let colormap = DynamicBiLevel { threshold: 128 };
    colormap.map_color(pix);
}

fn grey_to_printable(image: &GrayImage) -> Result<PrintableImage, Box<dyn std::error::Error>> {
    let rows = image
        .rows()
        .map(|row| row_to_bitvec(row))
        .collect::<Result<Vec<[u8; 8]>, _>>()?;
    //println!("{:?}", bitvecs);
    Ok(PrintableImage { data: rows })
}

fn row_to_bitvec(
    row: image::buffer::Pixels<Luma<u8>>,
) -> Result<[u8; 8], Box<dyn std::error::Error>> {
    let bitvec: BitVec<Msb0, u8> = row.map(|pix| !is_pixel_white(pix)).collect();
    let bytevec = &bitvec.into_vec();
    let mut result = [0 as u8; 8];
    let bytes = &bytevec.as_slice()[..result.len()]; // panics if not enough data
    result.copy_from_slice(bytes);
    Ok(result)
}

fn is_pixel_white(pixel: &Luma<u8>) -> bool {
    pixel.to_luma()[0] == 0xFF
}
