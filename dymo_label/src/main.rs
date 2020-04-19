#[macro_use]
extern crate log;

use structopt::StructOpt;

pub mod opt;
use opt::Opt;
use opt::Source;

use dymo_label::picture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opt = Opt::from_args();
    debug!("{:?}", opt);

    let threshold = opt.threshold;

    let pic = match opt.source {
        Source::Image { image } => image::open(image)?,
        Source::Text { text, font } => picture::create_image(&text, &font)?,
    };

    let bw_pic = picture::convert_to_bw(&pic, threshold)?;

    if opt.preview {
        bw_pic.save("preview.png")?;
    } else {
        dymo_label::print_label(&bw_pic)?;
    }
    Ok(())
}
