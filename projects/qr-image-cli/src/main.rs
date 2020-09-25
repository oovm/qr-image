use clap::{App, Arg};
use qr_image_core::{EcLevel, QrError, QrImage, Version};
use std::{path::PathBuf, str::FromStr};

fn main() -> Result<(), QrError> {
    let app = App::new("QR Image Embed")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("INPUT").help("Sets the input image file path").required(true).index(1))
        .arg(Arg::with_name("Text").help("Sets the qr text for encoding").required(true).index(2))
        .arg(Arg::with_name("Size").help("Set output image size").short("s").long("size").takes_value(true).value_name("size"))
        .arg(
            // flag, ture if turn on
            Arg::with_name("Enhance")
                .help("Set enhanced mode")
                .short("e")
                .long("enhance")
                .takes_value(true)
                .value_name("enhance"),
        )
        .arg(
            Arg::with_name("EC Level")
                .help("Set EC level")
                //.short("e")
                .long("ec")
                .takes_value(true)
                .value_name("ec"),
        )
        .arg(
            Arg::with_name("QR Version")
                .help("Set QR Version")
                //.short("e")
                .long("qr")
                .takes_value(true)
                .value_name("qr"),
        )
        .get_matches();
    let input = app.value_of("INPUT").and_then(|p| PathBuf::from_str(p).ok()).unwrap();
    let text = app.value_of("TEXT").unwrap();

    println!("{:?}", input);
    println!("{}", text);

    let output_size = app.value_of("size").and_then(|o| u32::from_str(o).ok());
    println!("{:?}", output_size);

    let mut render = QrImage::default();
    match app.value_of("enhance").and_then(|o| o.chars().next()) {
        // yes | on | true
        Some('y') | Some('o') | Some('t') => render.enhanced = true,
        _ => render.enhanced = false,
    }
    match app.value_of("ec").and_then(|o| o.to_uppercase().chars().next()) {
        Some('L') => render.ec_level = EcLevel::L,
        Some('M') => render.ec_level = EcLevel::M,
        Some('H') => render.ec_level = EcLevel::H,
        Some('Q') => render.ec_level = EcLevel::Q,
        _ => (),
    }
    if let Some(i) = app.value_of("qr").and_then(|o| i16::from_str(o).ok()) {
        render.qr_version = Version::Normal(i)
    }
    println!("{:?}", render);

    Ok(())
}
