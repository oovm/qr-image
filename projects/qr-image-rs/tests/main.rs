use image::{
    imageops::{resize, FilterType},
    open, DynamicImage, GrayImage,
};

#[test]
fn test() {
    let rgba = open("tests/wolfram-wolf.png").unwrap().into_rgba();
    let gray = DynamicImage::ImageRgba8(rgba).into_luma();
    let out = resize(&gray, 177, 177, FilterType::Lanczos3);
    let out = DynamicImage::ImageLuma8(out).into_luma();
    println!("{}", out)
}
