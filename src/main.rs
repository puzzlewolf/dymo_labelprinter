use image::imageops::colorops::ColorMap;
use image::*;

use bitvec::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use dymo_hid::command_accumulator::CommandAccumulator;
use dymo_hid::printable_image::PrintableImage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pic = image::open("testdata/bold.png")?;
    if pic.dimensions().1 != 64 {
        println!("height of image must be 64, not {}!", pic.dimensions().1);
        return Ok(());
    }
    if pic.color().has_alpha() {
        println!("image must not have transparency!");
        return Ok(());
    }
    println!("Dimensions of image: {:?}", pic.dimensions());
    let pic = pic.rotate90();
    let new_pic = convert_to_bw(&pic).unwrap();
    new_pic.save("output/output.png")?;
    print(&new_pic)?;
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

fn print(image: &GrayImage) -> Result<(), Box<dyn std::error::Error>> {
    let rows: Vec<[u8; 8]> = image
        .rows()
        .map(|row| row_to_bitvec(row).unwrap())
        .collect();
    //println!("{:?}", bitvecs);
    let pi = PrintableImage { data: rows };
    //pi.data.iter().for_each(|row| println!("{:?}", row));
    let mut ca = CommandAccumulator::new();
    ca.generate_commands(&pi);
    let commands = ca.accu;
    let mut f = File::create("commands")?;
    f.write_all(commands.as_slice())?;
    Ok(())
}

fn row_to_bitvec(
    row: image::buffer::Pixels<Luma<u8>>,
) -> Result<[u8; 8], Box<dyn std::error::Error>> {
    let bitvec: BitVec<Msb0, u8> = row.map(|pix| !is_pixel_white(pix)).collect();
    println!("{}", bitvec);
    let bytevec = &bitvec.into_vec();
    println!("{:?}", bytevec);
    let mut result = [0 as u8; 8];
    let bytes = &bytevec.as_slice()[..result.len()]; // panics if not enough data
    result.copy_from_slice(bytes);
    Ok(result)
}

fn is_pixel_white(pixel: &Luma<u8>) -> bool {
    pixel.to_luma()[0] == 0xFF
}

/// A bi-level color map with parameterized threshold
#[derive(Clone, Copy)]
pub struct DynamicBiLevel {
    threshold: u8,
}

impl ColorMap for DynamicBiLevel {
    type Color = Luma<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Luma<u8>) -> usize {
        let luma = color.0;
        if luma[0] > self.threshold {
            1
        } else {
            0
        }
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Luma<u8>) {
        let new_color = 0xFF * self.index_of(color) as u8;
        let luma = &mut color.0;
        luma[0] = new_color;
    }
}
