use image::*;

use dymo_hid::command_accumulator::CommandAccumulator;
use dymo_hid::image::{PrintableImage, convert_to_bw};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

//use std::path::PathBuf;
use structopt::StructOpt;

pub mod opt;
use opt::Opt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let threshold = opt.threshold;

    let pic = {
        if let Some(image_arg) = opt.image {
            image::open(image_arg)?
        } else
        {
            create_image("testText")?
        }
    };
    println!("Dimensions of image: {:?}", pic.dimensions());

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

    let pic = pic.rotate90();
    let bw_pic = convert_to_bw(&pic, threshold);
    bw_pic.save("output/preview.png")?;

    write_commands(&bw_pic)?;
    Ok(())
}

fn create_image(text: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    use std::process::Command;
    let output = Command::new("convert")
        .arg("-background")
        .arg("white")
        .arg("-fill")
        .arg("black")
        .arg("-font")
        .arg("Ubuntu")
        .arg("-gravity")
        .arg("center")
        .arg("-size")
        .arg("x64")
        .arg(["label:", text].join(""))
        .arg("png:-") //output png image to stdout
        .output()
        .expect("failed to execute process");
    let image = image::load_from_memory(&output.stdout)?;
    Ok(image)
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
