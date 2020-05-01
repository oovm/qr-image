use crate::{Luma, QrCode, QrImage, QrResult, Version};
use image::{
    imageops::{resize, FilterType},
    DynamicImage, GrayImage,
};
use std::ops::Mul;

#[test]
fn test() {
    let cfg = QrImage::default();

    // Encode some data into bits.
    let code = cfg.target_qr(b"01234567").unwrap();
    // cfg.image(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<f32>>().build();

    let img = DynamicImage::ImageLuma8(image);
    // Save the image.
    image.save("./qrcode.png").unwrap();
}

impl QrImage {
    fn target_qr(&self, data: &[u8]) -> QrResult<QrCode> {
        match QrCode::with_version(data, self.qr_version, self.ec_level) {
            Ok(o) => Ok(o),
            Err(_) => match QrCode::with_error_correction_level(data, self.ec_level) {
                Ok(o) => Ok(o),
                Err(_) => QrCode::new(data),
            },
        }
    }
    fn target_image(&self, data: &[u8], img: &DynamicImage) -> QrResult<GrayImage> {
        let qr = self.target_qr(data)?;
        let size = get_size(&qr);
        let out = resize(img, 3 * size, 3 * size, FilterType::Lanczos3);
        let mut img = DynamicImage::ImageRgba8(out).into_luma();
        // self.dithering(&qr, &mut img);
        return Ok(img);
    }
}

fn get_size(qr: &QrCode) -> u32 {
    match qr.version() {
        Version::Normal(i) => 17 + 4 * i as u32,
        Version::Micro(_) => unimplemented!(),
    }
}

pub unsafe fn get_align_locations(qr: &QrCode) -> Vec<(usize, usize)> {
    let mut aligns = vec![];
    match qr.version() {
        Version::Normal(ver) => {
            let align_location: &[Vec<usize>; 39] = &[
                vec![6, 18],
                vec![6, 22],
                vec![6, 26],
                vec![6, 30],
                vec![6, 34],
                vec![6, 22, 38],
                vec![6, 24, 42],
                vec![6, 26, 46],
                vec![6, 28, 50],
                vec![6, 30, 54],
                vec![6, 32, 58],
                vec![6, 34, 62],
                vec![6, 26, 46, 66],
                vec![6, 26, 48, 70],
                vec![6, 26, 50, 74],
                vec![6, 30, 54, 78],
                vec![6, 30, 56, 82],
                vec![6, 30, 58, 86],
                vec![6, 34, 62, 90],
                vec![6, 28, 50, 72, 94],
                vec![6, 26, 50, 74, 98],
                vec![6, 30, 54, 78, 102],
                vec![6, 28, 54, 80, 106],
                vec![6, 32, 58, 84, 110],
                vec![6, 30, 58, 86, 114],
                vec![6, 34, 62, 90, 118],
                vec![6, 26, 50, 74, 98, 122],
                vec![6, 30, 54, 78, 102, 126],
                vec![6, 26, 52, 78, 104, 130],
                vec![6, 30, 56, 82, 108, 134],
                vec![6, 34, 60, 86, 112, 138],
                vec![6, 30, 58, 86, 114, 142],
                vec![6, 34, 62, 90, 118, 146],
                vec![6, 30, 54, 78, 102, 126, 150],
                vec![6, 24, 50, 76, 102, 128, 154],
                vec![6, 28, 54, 80, 106, 132, 158],
                vec![6, 32, 58, 84, 110, 136, 162],
                vec![6, 26, 54, 82, 110, 138, 166],
                vec![6, 30, 58, 86, 114, 142, 170],
            ];
            let loc = align_location.get_unchecked(ver - 2);
            for a in loc.to_vec() {
                for b in loc.to_vec() {
                    if !((a == 0 || b == 0) || (a == loc.len() - 1 && b == 0) || (a == 0 && b == loc.len() - 1)) {
                        for i in (loc.get_unchecked(a) - 2).mul(3)..(loc.get_unchecked(a) + 3).mul(3) {
                            for j in (loc.get_unchecked(b) - 2).mul(3)..(loc.get_unchecked(b) + 3).mul(3) {
                                aligns.push((i, j))
                            }
                        }
                    }
                }
            }
        }
        Version::Micro(ver) => {
            let _ = ver;
            unimplemented!()
        }
    }
    return aligns;
}

pub fn redraw_enhance() {
    unimplemented!()
}
