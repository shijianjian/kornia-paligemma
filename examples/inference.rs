use argh::FromArgs;
use std::path::PathBuf;

use kornia_io as F;
use kornia_paligemma::{Paligemma, PaligemmaConfig};

#[derive(FromArgs)]
/// Generate a description of an image using Google Paligemma
struct Args {
    /// path to an input image
    #[argh(option, short = 'i')]
    image_path: PathBuf,

    /// prompt to ask the model
    #[argh(option, short = 'p')]
    text_prompt: String,

    /// the length of the generated text
    #[argh(option, default = "100")]
    sample_length: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();

    // read the image
    let image = if args.image_path.extension().unwrap_or_default() == "jpg" {
        F::jpeg::read_image_jpeg_rgb8(args.image_path)?
    } else {
        F::png::read_image_png_rgb8(args.image_path)?
    };

    // create the paligemma model
    let mut paligemma = Paligemma::new(PaligemmaConfig::default())?;

    // generate a caption of the image
    let _caption = paligemma.inference(&image, &args.text_prompt, args.sample_length, true)?;

    Ok(())
}
