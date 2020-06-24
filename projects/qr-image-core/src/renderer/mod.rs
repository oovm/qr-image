use crate::{QrCode, QrImage, QrResult, Version};
use image::{
    imageops::{resize, FilterType},
    DynamicImage, GenericImage, GenericImageView, Rgb, RgbImage,
};

impl QrImage {
    fn target_qr(&self, data: &[u8]) -> QrResult<QrCode> {
        if self.auto_size {
            match QrCode::with_version(data, self.qr_version, self.ec_level) {
                Ok(o) => Ok(o),
                Err(_) => match QrCode::with_error_correction_level(data, self.ec_level) {
                    Ok(o) => Ok(o),
                    Err(_) => QrCode::new(data),
                },
            }
        }
        else {
            QrCode::with_version(data, self.qr_version, self.ec_level)
        }
    }
    pub fn render(&self, data: &[u8], img: &DynamicImage) -> QrResult<DynamicImage> {
        let qr = self.target_qr(data)?;
        let size = qr.width() as u32;
        let out = resize(img, 3 * size, 3 * size, FilterType::Triangle);
        let rgb = unsafe {
            redraw_locations(
                &qr,
                DynamicImage::ImageRgba8(out).into_rgb8(),
                self.dark_color,
                self.light_color,
                self.enhanced,
                !self.enhanced,
            )
        };
        return Ok(DynamicImage::ImageRgb8(rgb));
    }
    pub fn render_frames(&self) {
        unimplemented!()
    }
}

pub unsafe fn get_align_locations(qr: &QrCode) -> Vec<(usize, usize)> {
    let mut aligns = vec![];
    match qr.version() {
        Version::Normal(ver) => {
            let align_location: &[Vec<usize>; 40] = &[
                vec![],
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
            let loc = align_location.get_unchecked(ver as usize - 1);
            for a in 0..loc.len() {
                for b in 0..loc.len() {
                    if !((a == 0 || b == 0) || (a == loc.len() - 1 && b == 0) || (a == 0 && b == loc.len() - 1)) {
                        for i in (loc.get_unchecked(a) * 3 - 6)..(loc.get_unchecked(a) * 3 + 9) {
                            for j in (loc.get_unchecked(b) * 3 - 6)..(loc.get_unchecked(b) * 3 + 9) {
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

#[rustfmt::skip]
pub unsafe fn redraw_locations(qr: &QrCode, bg: RgbImage, dark: Rgb<u8>, light: Rgb<u8>, enhanced: bool, skip_bg: bool) -> RgbImage {
    let aligns = get_align_locations(qr);
    let mut qr_img = qr_render_rgb(qr, dark, light);
    // FIXME:
    // Too slow, maybe the target image should be modified
    for i in 0..qr_img.width() - 0 {
        for j in 0..qr_img.width() - 0 {
            let _ = skip_bg;
            if (i < 21 && j < 21)
            || (i < 21 && j > qr_img.width() - 22)
            || (i > qr_img.width() - 22 && j < 21)
            || (enhanced && [18, 19, 20].contains(&i))
            || (enhanced && [18, 19, 20].contains(&j))
            || (enhanced && aligns.contains(&(i as usize + 0, j as usize + 0)))
            || (i % 3 == 1 && j % 3 == 1)
            //|| (!skip_bg && bg.unsafe_get_pixel(i, j) == dark)
            {
                continue;
            }
            else {
                qr_img.unsafe_put_pixel(i, j, bg.unsafe_get_pixel(i, j))
            }
        }
    }
    return qr_img;
}

pub fn qr_render_rgb(qr: &QrCode, dark: Rgb<u8>, light: Rgb<u8>) -> RgbImage {
    qr.render().quiet_zone(false).module_dimensions(3, 3).dark_color(dark).light_color(light).build()
}
