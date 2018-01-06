// TODO: Implement everything
// TODO: Rethink "manipulation" and "manipulation method"
extern crate clap;
extern crate gif;
extern crate image;

use clap::{Arg, App};

// use std::fs::File;
use std::process::exit;

use image::{GenericImage, DynamicImage};

mod animations;
use animations::{zoom, shake};

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

    let source_image: DynamicImage;
    if let Some(file_location) = matches.value_of("input") {
        // Try and load source image with location in variable 'image'
        source_image = match image::open(file_location) {
            Ok(result) => result,
            Err(_) => {
                println!("Image load failed, does the file exist?");
                exit(1);
            }
        };
        println!("Using {} as source image with dimensions {:?}", file_location, source_image.dimensions());
    } else {
        println!("You must include a source image.");
        exit(1)
    }

    let output_path: String;
    if let Some(output) = matches.value_of("output") {
        println!("Writing file to {}", output);
        output_path = output.to_string();
        // Save output location
    } else {
        output_path = String::from("./output.gif");
        println!("Writing file to output.gif");
    }

    match matches.value_of("method") {
        Some("zoom") => {
            println!("Zooming on out of here!");
            zoom::process(source_image, output_path);
            println!("Zooming in the here dud!");
        },
        Some("shake") => {
            shake::process(source_image, output_path);
            println!("Rocking the boat.");
        },
        Some("rotate") => {
            println!("You spin me right round baby right round.")
        }
        Some(m) => println!("{} is not a supported method. Run with --methods flag to list optional methods.", m),
        None => println!("You must include a maniuplation method.")
    }

    return;

}
