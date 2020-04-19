use structopt::StructOpt;
use std::path::PathBuf;

/// Print a label on a Dymo label printer.
#[derive(StructOpt, Debug)]
pub(crate) struct Opt {
    /// A "white" pixel's minimum luma value
    #[structopt(short, long, default_value = "128")]
    pub threshold: u8,

    /// Shows preview instead of printing
    #[structopt(short, long)]
    pub preview: bool,

    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    pub source: Source 
}

#[derive(StructOpt, Debug)]
pub enum Source {
    /// Creates label from image
    Image {
        /// Path to image
        #[structopt(parse(from_os_str))]
        image: PathBuf,
    },
    /// Creates label from text.
    ///
    /// Use `convert -list font` for possible fonts.
    Text {
        /// Label text
        #[structopt()]
        text: String,

        /// Font
        #[structopt(short, long, default_value = "Ubuntu")]
        font: String,
    },
}
