use clap::{App, Arg};
use qr_image_core::{EcLevel, QrError, QrImage, Rgb, Version};
use std::{num::ParseIntError, str::FromStr};

fn main() -> Result<(), QrError> {
    let app = App::new("QR Image Embed")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("INPUT").help("Sets the input image file path").required(true).index(1))
        .arg(Arg::with_name("OUTPUT").help("Sets the output file path").required(true).index(2))
        .arg(Arg::with_name("Size").help("Set output image size").short("s").long("size").takes_value(true).value_name("size"))
        .arg(
            Arg::with_name("Enhance")
                .help("Set enhanced mode")
                .short("e")
                .long("enhance")
                .takes_value(true)
                .value_name("enhance"),
        )
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    println!("{}", input);
    println!("{}", output);
    let mut render = QrImage::default();
    let output_size = match app.value_of("size") {
        Some(o) => match u32::from_str(o) {
            Ok(o) => o,
            Err(_) => None,
        },
        None => None,
    };
    match app.value_of("enhance") {
        Some(o) => {
            match o.chars().next() {
                Some(c) => {
                    match c {
                        // yes | on
                        'y' | 'o' => render.enhanced = true,
                        _ => render.enhanced = false,
                    }
                }
                None => (),
            }
        }
        None => (),
    };
    println!("{}", output_size);
    println!("{:?}", render);
    Ok(())
}
