use bitvec::prelude::*;
use image::*;

/// represents an image that can be printed on a dymo label printer.
pub struct PrintableImage {
    // TODO private
    pub data: Vec<[u8; 8]>,
}

impl PrintableImage {
    pub fn preview(&self) -> String {
        let mut output = String::new();
        self.data.iter().for_each(|row| {
            row
                .iter()
                .for_each(|byte| output.push_str(&format!("{:08b}", byte)));
            output.push_str("\n");
        });
        output.replace("0", ".").replace("1", "X")
    }

    pub fn printable_from_grey(image: &GrayImage) -> Result<Self, Box<dyn std::error::Error>> {
        let rows = image
            .rows()
            .map(|row| row_to_bitvec(row))
            .collect::<Result<Vec<[u8; 8]>, _>>()?;
        //println!("{:?}", bitvecs);
        Ok(PrintableImage { data: rows })
    }
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
