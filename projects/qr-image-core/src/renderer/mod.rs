use crate::{QrCode, QrImage, Version};
use image::{DynamicImage, GenericImage, GenericImageView, GrayImage, Rgb, RgbImage};
use qrcode::render::Pixel;
use std::ops::Mul;

impl QrImage {
    pub fn render(&self) {
        unimplemented!()
    }
    pub fn render_frames(&self) {
        unimplemented!()
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
            let loc: &Vec<usize> = align_location.get_unchecked(ver as usize - 2);
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

pub unsafe fn redraw_rgb(qr: &mut QrCode, bg: &RgbImage) {
    let aligs = get_align_locations(qr);
    let mut qr_img = qr_render_rgb(qr);
    for i in 0..qr.width() - 24 {
        for j in 0..qr.width() - 24 {
            if !(([18, 19, 20].contains(&i)) || [18, 19, 20].contains(&j))
                || (i < 24 && j < 24)
                || (i < 24 && j > qr.width() - 49)
                || (i > qr.width() - 49 && j < 24)
                || aligs.contains(&(i as usize, j as usize))
                || (i % 3 == 1 && j % 3 == 1)
            //|| (bg.unsafe_get_pixel(i as u32, j as u32).0.get_unchecked(2) == &0)
            {
                qr_img.unsafe_put_pixel(i as u32 + 12, j as u32 + 12, bg.unsafe_get_pixel(i as u32, j as u32))
            }
        }
    }
}

pub fn qr_render_rgb(qr: &QrCode, dark: Rgb<u8>, light: Rgb<u8>) -> RgbImage {
    qr.render().quiet_zone(false).module_dimensions(1, 1).dark_color(dark).light_color(light).build()
}
