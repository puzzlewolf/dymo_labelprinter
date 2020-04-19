use image::*;

use dymo_hid::command_accumulator::CommandAccumulator;
use dymo_hid::image::{PrintableImage, convert_to_bw};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

use structopt::StructOpt;

pub mod opt;
use opt::Opt;
use opt::Source;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let threshold = opt.threshold;

    let pic = match opt.source {
        Source::Image{image} => {
            image::open(image)?
        },
        Source::Text{text, font} => {
                create_image(&text, &font)?
        },
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

    if opt.preview {
        bw_pic.save("preview.png")?;
    } else {
        print_label(&bw_pic)?;
    }
    Ok(())
}

fn create_image(text: &str, font: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    use std::process::Command;
    let output = Command::new("convert")
        .args(&["-background", "white"])
        .args(&["-fill", "black"])
        .args(&["-font", font])
        .args(&["-gravity", "center"])
        .args(&["-size", "x64"])
        .arg(["label:", text].join(""))
        .arg("png:-") //output png image to stdout
        .output()
        .expect("failed to execute imagemagick");
    let image = image::load_from_memory(&output.stdout)?;
    Ok(image)
}

fn print_label(image: &GrayImage) -> Result<(), Box<dyn std::error::Error>> {
    let pi = PrintableImage::printable_from_grey(image)?;
    print!("{}", pi.preview());
    let mut ca = CommandAccumulator::new();
    ca.generate_commands(&pi);
    let commands = ca.accu;
    let mut f = File::create("/dev/hidraw0")?;
    f.write_all(commands.as_slice())?;
    Ok(())
}
