#[macro_use]
extern crate log;

pub mod opt;

use env_logger::Env;
use opt::Opt;
use opt::Source;
use structopt::StructOpt;

use dymo_print::picture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

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
        dymo_print::print_label(&bw_pic)?;
    }
    Ok(())
}
