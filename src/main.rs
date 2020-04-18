use image::*;

use dymo_hid::command_accumulator::CommandAccumulator;
use dymo_hid::image::{PrintableImage, convert_to_bw};
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
    let new_pic = convert_to_bw(&pic, 128);
    new_pic.save("output/preview.png")?;

    write_commands(&new_pic)?;
    Ok(())
}

fn write_commands(image: &GrayImage) -> Result<(), Box<dyn std::error::Error>> {
    let pi = PrintableImage::printable_from_grey(image)?;
    print!("{}", pi.preview());
    let mut ca = CommandAccumulator::new();
    ca.generate_commands(&pi);
    let commands = ca.accu;
    let mut f = File::create("commands")?;
    f.write_all(commands.as_slice())?;
    Ok(())
}
