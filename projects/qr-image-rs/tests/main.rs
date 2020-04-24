use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

#[test]
fn test() {
    // Construct a new RGB ImageBuffer with the specified width and height.
    let img: RgbImage = ImageBuffer::new(512, 512);

    // Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::from_fn(512, 512, |x, y| if x % 2 == 0 { image::Luma([0u8]) } else { image::Luma([255u8]) });

    // Obtain the image's width and height.
    let (width, height) = img.dimensions();

    // Access the pixel at coordinate (100, 100).
    let pixel = img[(100, 100)];

    // Or use the `get_pixel` method from the `GenericImage` trait.
    let pixel = *img.get_pixel(100, 100);

    // Put a pixel at coordinate (100, 100).
    img.put_pixel(100, 100, pixel);

    // Iterate over all pixels in the image.
    for pixel in img.pixels() {
        // Do something with pixel.
    }
}
