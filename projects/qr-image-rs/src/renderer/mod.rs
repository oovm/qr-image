mod refine;
mod render;

// use qrcode_generator::QrCodeEcc;

use crate::{Luma, QrCode, QrImage, QrResult, Version};
use image::{imageops::{resize, FilterType}, DynamicImage, GrayImage};

#[test]
fn test() {
    let cfg = QrImage::default();

    // Encode some data into bits.
    let code = cfg.target_qr(b"01234567").unwrap();
    //cfg.image(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<f32>>().build();

    let img = DynamicImage::ImageLuma8(image);
    // Save the image.
    image.save("./qrcode.png").unwrap();
}


impl QrImage {
    pub fn target_qr(&self, data: &[u8]) -> QrResult<QrCode> {
        match QrCode::with_version(data, self.qr_version, self.ec_level) {
            Ok(o) => Ok(o),
            Err(_) => match QrCode::with_error_correction_level(data, self.ec_level) {
                Ok(o) => Ok(o),
                Err(_) => QrCode::new(data),
            },
        }
    }
    pub fn target_image(&self, data: &[u8], img: &DynamicImage) -> QrResult<GrayImage> {
        let qr = self.target_qr(data)?;
        let size = get_size(&qr);
        let out = resize(img, 3 * size, 3 * size, FilterType::Lanczos3);
        let mut img = DynamicImage::ImageRgba8(out).into_luma();
        self.dithering(&qr, &mut img);
        return Ok(img)
    }
}

fn get_size(qr: &QrCode) -> u32 {
    match qr.version() {
        Version::Normal(i) => 17 + 4 * i as u32,
        Version::Micro(_) => unimplemented!(),
    }
}