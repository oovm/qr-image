use crate::{Luma, QrCode, QrImage, QrResult, Version};
use image::{
    imageops::{resize, FilterType},
    DynamicImage, GrayImage,
};
use std::ops::Range;

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
        self.dithering(&qr, &mut img);
        return Ok(img);
    }
}

fn get_size(qr: &QrCode) -> u32 {
    match qr.version() {
        Version::Normal(i) => 17 + 4 * i as u32,
        Version::Micro(_) => unimplemented!(),
    }
}

fn c() {
    let align_location = [
        [6, 18],
        [6, 22],
        [6, 26],
        [6, 30],
        [6, 34],
        [6, 22, 38],
        [6, 24, 42],
        [6, 26, 46],
        [6, 28, 50],
        [6, 30, 54],
        [6, 32, 58],
        [6, 34, 62],
        [6, 26, 46, 66],
        [6, 26, 48, 70],
        [6, 26, 50, 74],
        [6, 30, 54, 78],
        [6, 30, 56, 82],
        [6, 30, 58, 86],
        [6, 34, 62, 90],
        [6, 28, 50, 72, 94],
        [6, 26, 50, 74, 98],
        [6, 30, 54, 78, 102],
        [6, 28, 54, 80, 106],
        [6, 32, 58, 84, 110],
        [6, 30, 58, 86, 114],
        [6, 34, 62, 90, 118],
        [6, 26, 50, 74, 98, 122],
        [6, 30, 54, 78, 102, 126],
        [6, 26, 52, 78, 104, 130],
        [6, 30, 56, 82, 108, 134],
        [6, 34, 60, 86, 112, 138],
        [6, 30, 58, 86, 114, 142],
        [6, 34, 62, 90, 118, 146],
        [6, 30, 54, 78, 102, 126, 150],
        [6, 24, 50, 76, 102, 128, 154],
        [6, 28, 54, 80, 106, 132, 158],
        [6, 32, 58, 84, 110, 136, 162],
        [6, 26, 54, 82, 110, 138, 166],
        [6, 30, 58, 86, 114, 142, 170],
    ];
    let mut aligs = vec![];
    if ver > 1 {
        let aloc: Vec<usize> = align_location[ver - 2];
        for a in aloc {
            for b in aloc {
                if !((a == 0 || b == 0) || a == aloc.len() - 1 && b == 0) || (a == 0 && b == aloc.len() - 1) {
                    for i in 3 * (aloc[a] - 2)..3 * (aloc[a] + 3) {
                        for j in 3 * (aloc[b] - 2)..3 * (aloc[b] + 3) {
                            aligs.push((i, j))
                        }
                    }
                }
            }
        }
    }

    for i in get_size(qr) - 24 {
        for j in get_size(qr) - 24 {
            if !(([18, 19, 20].contains(i)) || [18, 19, 20].contains(j))
                || (i < 24 && j < 24)
                || (i < 24 && j > get_size(qr) - 49)
                || (i > get_size(qr) - 49 && j < 24)
                || aligs.contains((i, j))
                || (i % 3 == 1 && j % 3 == 1)
                || (bg0.getpixel((i, j))[3] == 0)
            {
                qr.putpixel((i + 12, j + 12), bg.getpixel((i, j)))
            }
        }
    }
}
