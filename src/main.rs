use image::*;
use image::imageops::colorops::ColorMap;

use bitvec::prelude::*;

use std::fs::File;
use std::io::prelude::*;

struct PrintableImage {
    data: Vec<[u8; 8]>,
}

impl PrintableImage {
    const SYN: u8 = 0x16;
    const ESC: u8 = 0x1b;

    pub fn to_commands(&self) -> Vec<u8> {
        let mut command_vec = PrintableImage::preamble();
        command_vec.append(&mut PrintableImage::bytes_per_line(8));
        for row in self.data.iter() {
            PrintableImage::append_data_row(&mut command_vec, &row);
        }
        command_vec.append(&mut PrintableImage::bytes_per_line(0));
        command_vec.append(&mut vec![PrintableImage::SYN; 56]);
        command_vec.append(&mut vec![PrintableImage::SYN; 56]);
        command_vec.append(&mut PrintableImage::get_status());
        command_vec
    }

    fn append_data_row(command_vec: &mut Vec<u8>, row: &[u8; 8]) {
        command_vec.push(PrintableImage::SYN);
        command_vec.extend(row);
    }

    fn preamble() -> Vec<u8> {
        let mut preamble = PrintableImage::tapecolor();
        preamble.append(&mut PrintableImage::dottab());
        preamble
    }

    fn print_commands(&self) {
        let mut c = self.to_commands();
        while c.len() > 8 {
            let tmp = c.split_off(8);
            println!("{:x?}", &c[0..8]);
            //c.iter().for_each(|byte| print!("{:x?} ", byte));
            c = tmp;
        }
    }

    /// The number of bytes in the following row(s).
    /// Seems to take no arguments.
    fn get_status() -> Vec<u8> {
        vec![PrintableImage::ESC, 'A' as u8]
    }

    /// The number of bytes in the following row(s).
    /// Seems to take one byte argument.
    fn bytes_per_line(num: u8) -> Vec<u8> {
        vec![PrintableImage::ESC, 'D' as u8, num]
    }

    /// The tape's color. Encoding unknown.
    /// Seems to take one byte argument.
    fn tapecolor() -> Vec<u8> {
        vec![PrintableImage::ESC, 'C' as u8, 0]
    }

    /// Probably whether (or how?) to print the tab character.
    /// Seems to take one byte argument.
    fn dottab() -> Vec<u8> {
        vec![PrintableImage::ESC, 'B' as u8, 0]
    }

// constants:
// SYN = 0x16 //marks start of line
// ESC = 0x1b //next byte encodes command
//      commands according to imgprint perlscript
//      A getstatus
//      D bytesperline, one argument, used as ESC, B, num_of_bytes e.g. 1b 44 07
//      C tapecolour, one argument, 0 known used
//      B dottab, one argument, 0 known used

}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let pic = image::open("testdata/bold.png")?;
	if pic.dimensions().1 != 64 {
		println!("height of image must be 64, not {}!", pic.dimensions().1);
		return Ok(())
	}
    if pic.color().has_alpha() {
		println!("image must not have transparency!");
		return Ok(())
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
    gray_image.pixels_mut().for_each(|pix| px_to_black_or_white(pix));
    Ok(gray_image)
}

fn px_to_black_or_white (pix: &mut Luma<u8>) {
    let colormap = DynamicBiLevel{threshold: 128};
    colormap.map_color(pix);
}

fn print(image: &GrayImage) -> Result<(), Box<dyn std::error::Error>> { 
    let rows: Vec<[u8; 8]> = image.rows().map(|row| row_to_bitvec(row).unwrap()).collect();
    //println!("{:?}", bitvecs);
    let pi = PrintableImage {
        data: rows
    };
    //pi.data.iter().for_each(|row| println!("{:?}", row));
    let commands = pi.to_commands();
    let mut f = File::create("commands")?;
    f.write_all(commands.as_slice())?;
    Ok(())
}

fn row_to_bitvec(row: image::buffer::Pixels<Luma<u8>>) -> Result<[u8; 8], Box<dyn std::error::Error>> {
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

#[test]
fn test_append_row() {
  let mut command: Vec<u8> = vec![17u8; 7];
  PrintableImage::append_data_row(&mut command, &mut [0u8, 1u8, 2u8, 3u8, 4u8, 5u8,6u8, 7u8]);
  assert_eq!(command[0..7], [17u8; 7]);
  assert_eq!(command[7], 0x16); 
  assert_eq!(command[8..16], [0,1,2,3,4,5,6,7]);
}

#[test]
fn test_preamble() {
  let preamble = PrintableImage::preamble();
  assert_eq!(preamble[0..6], [0x1b, 0x43, 0, 0x1b, 0x44, 0]);
}

#[test]
fn test_get_status() {
  let get_status = PrintableImage::get_status();
  assert_eq!(get_status[0..2], [0x1b, 0x41]);
}

#[test]
fn test_bytes_per_line() {
  let bytes_per_line = PrintableImage::bytes_per_line(8);
  assert_eq!(bytes_per_line[0..3], [0x1b, 0x42, 0x08]);
}

#[test]
fn test_tapecolor() {
  let tapecolor = PrintableImage::tapecolor();
  assert_eq!(tapecolor[0..3], [0x1b, 0x43, 0]);
}

#[test]
fn test_dottab() {
  let dottab = PrintableImage::dottab();
  assert_eq!(dottab[0..3], [0x1b, 0x44, 0]);
}
