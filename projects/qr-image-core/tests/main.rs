#[test]
fn ready() {
    println!("it works!")
}

use image::{open, DynamicImage};
use qr_image_core::QrImage;

#[test]
fn test() {
    let cfg = QrImage::default();
    let img = DynamicImage::ImageRgba8(open("tests/wolfram-wolf.png").unwrap().into_rgba8());
    // Encode some data into bits.
    let code = cfg.render("苟利国家生死以".as_bytes(), &img).unwrap();
    code.save("./tests/wolfram-wolf.qr.png").unwrap();
}
