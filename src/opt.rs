use structopt::StructOpt;
use std::path::PathBuf;

/// A basic example
#[derive(StructOpt, Debug)]
pub(crate) struct Opt {
    /// A "white" pixel's minimum luma value
    #[structopt(short, long, default_value = "128")]
    pub threshold: u8,

    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    pub source: Source 
}

#[derive(StructOpt, Debug)]
pub enum Source {
    Image {
        /// Input image
        #[structopt(short, long, parse(from_os_str))]
        image: PathBuf,
    },
    Text {
        /// Input text
        #[structopt()]
        text: String,
        
        /// Input text
        #[structopt(short, long, default_value = "Ubuntu")]
        font: String,
    },
}
