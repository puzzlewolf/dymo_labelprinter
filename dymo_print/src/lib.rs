#[macro_use]
extern crate log;

pub mod color_map;
pub mod command_accumulator;
pub mod picture;
pub mod fonts;

use crate::command_accumulator::CommandAccumulator;
use std::fs::File;
use std::io::prelude::*;

static IM_CONVERT: &str = env!("IM_CONVERT");

pub fn print_label(image: &image::GrayImage) -> Result<(), Box<dyn std::error::Error>> {
    let pi = crate::picture::PrintableImage::printable_from_grey(image)?;
    debug!("{}", pi.preview());
    let mut ca = CommandAccumulator::new();
    ca.generate_commands(&pi);
    let commands = ca.accu;
    let mut f = File::create("/dev/dymoprint")?;
    f.write_all(commands.as_slice())?;
    Ok(())
}
