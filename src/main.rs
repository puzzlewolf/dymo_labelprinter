use image::*;
use image::imageops::colorops::ColorMap;

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let pic = image::open("testdata/valodim.png")?;
	if pic.dimensions().1 != 64 {
		println!("height of image must be 64, not {}!", pic.dimensions().1);
		return Ok(())
	}
    if pic.color().has_alpha() {
		println!("image must not have transparency!");
		return Ok(())
    }
    //println!("pic.dimensions: {:?}", pic.dimensions());
    let new_pic = convert_to_bw(&pic).unwrap();
    new_pic.save("output/output.png")?;
    Ok(())
}

fn convert_to_bw(image: &DynamicImage) -> ImageResult<GrayImage> {
    let mut gray_image = image.grayscale().into_luma();
    gray_image.pixels_mut().for_each(|pix| to_black_or_white(pix));
    Ok(gray_image)
}

fn to_black_or_white (pix: &mut Luma<u8>) {
    let colormap = DynamicBiLevel{threshold: 70};
    colormap.map_color(pix);
}

// constants:
// SYN = 0x16 //marks start of line
// ESC = 0x1b //next byte encodes command
//      commands according to imgprint perlscript
//      A getstatus
//      B bytesperline, one argument, used as ESC, B, num_of_bytes e.g. 1b 44 07
//      C tapecolour, one argument, 0 known used
//      D dottab, one argument, 0 known used

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
