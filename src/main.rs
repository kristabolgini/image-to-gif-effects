// TODO: Implement everything
// TODO: Rethink "manipulation" and "manipulation method"

extern crate image;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Image to Gif")
                      .version("v0.0.0")
                      .author("Logan Saso <logan.saso2016@gmail.com>")
                      .about("Takes an image input and allows a few output options in GIF form.")
                      .arg(
                          Arg::with_name("input")
                              .short("i")
                              .long("input")
                              .value_name("IMAGE")
                              .takes_value(true)
                              .help("Source file"))
                      .arg(
                          Arg::with_name("output")
                              .short("o")
                              .long("output")
                              .value_name("OUTPUT")
                              .takes_value(true)
                              .help("Destination file"))
                      .arg(
                          Arg::with_name("method")
                              .short("m")
                              .long("method")
                              .value_name("METHOD")
                              .takes_value(true)
                              .help("Manipulation methods are: shake, zoom, rotate"))
                      .get_matches();
                    
    if let Some(image) = matches.value_of("input") {
        // Try and load source image with location in variable 'image'
        println!("Using {} as source image", image)
    } else {
        println!("You must include a source image.");
        ::std::process::exit(1)
    }

    if let Some(output) = matches.value_of("output") {
        println!("Writing file to {}", output);
        // Save output location
    } else {
        println!("Writing file to output.gif")
    }

    match matches.value_of("method") {
        Some("zoom") => {
            println!("Zooming on out of here!");
        },
        Some("shake") => {
            println!("Rocking the boat.")
        },
        Some("rotate") => {
            println!("You spin me right round baby right round.")
        }
        Some(m) => println!("{} is not a supported method. Run with --methods flag to list optional methods.", m),
        None => println!("You must include a maniuplation method.")
    }

}