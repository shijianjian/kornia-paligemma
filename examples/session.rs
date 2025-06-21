use argh::FromArgs;
use std::path::PathBuf;
use std::io::{self, Write};

use kornia_io as F;
use kornia_paligemma::{Paligemma, PaligemmaConfig};

#[derive(FromArgs)]
/// Generate descriptions of an image using Google Paligemma in an interactive session
struct Args {
    /// path to an input image
    #[argh(option, short = 'i')]
    image_path: PathBuf,

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

    println!("Paligemma interactive session started!");
    println!("Image loaded successfully. You can now ask questions about the image.");
    println!("Type 'quit' or 'exit' to end the session.");
    println!("Type 'help' for available commands.");
    println!();

    // Interactive session loop
    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let prompt = input.trim();

        // Handle special commands
        match prompt.to_lowercase().as_str() {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                println!("Available commands:");
                println!("  help     - Show this help message");
                println!("  quit     - Exit the session");
                println!("  exit     - Exit the session");
                println!("  <prompt> - Ask a question about the image");
                println!();
                continue;
            }
            "" => {
                println!("Please enter a prompt or type 'help' for available commands.");
                println!();
                continue;
            }
            _ => {}
        }

        // Generate response
        let response = paligemma.inference(&image, prompt, args.sample_length, false)?;
        println!("Paligemma: {}", response);
        println!();
    }

    Ok(())
}
