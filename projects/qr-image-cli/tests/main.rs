use image::{
    imageops::{resize, FilterType},
    open, DynamicImage, GrayImage,
};
use qr_image::QrImage;

#[test]
fn test() {
    let cfg = QrImage::default();
    let img = DynamicImage::ImageRgba8(open("tests/wolfram-wolf.png").unwrap().into_rgba());
    // Encode some data into bits.
    let code = cfg.target_image(b"01234567", &img).unwrap();
    code.save("./qrcode.png").unwrap();
}
