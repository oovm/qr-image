use crate::{Luma, QrImage, QrResult};
use image::{DynamicImage, GenericImage, GenericImageView, GrayImage};
use qrcode::{Color, QrCode};
use std::ops::{Index, IndexMut};

impl QrImage {
    // FIXME: use u8 arithmetic
    pub(crate) fn dithering(&self, qr: &QrCode, img: &mut GrayImage) {
        let size = img.width();
        for j in 1..=size {
            for i in 1..=size {
                let tmp1 = if i % 3 == 2 && j % 3 == 2 {
                    self.get_qr_pixel(qr, (i + 1) / 3, (j + 1) / 3)
                }
                else {
                    self.unit_step(img.get_pixel(i, j))
                };
                let tmp2 = self.clip(img.get_pixel(i, j), &tmp1);
                *img.get_pixel_mut(i, j) = tmp1;
                *img.get_pixel_mut(i, j + 1).index_mut(0) += (0.4375 * tmp2[0] as f32) as u8;

                if j != 1 {
                    *img.get_pixel_mut(i + 1, j - 1).index_mut(0) += (0.1875 * tmp2[0] as f32) as u8
                }
                *img.get_pixel_mut(i + j, j).index_mut(0) += (0.3125 * tmp2[0] as f32) as u8;
                *img.get_pixel_mut(i + 1, j + 1).index_mut(0) += (0.0625 * tmp2[0] as f32) as u8;
            }
        }
    }

    fn get_qr_pixel(&self, qr: &QrCode, i: u32, j: u32) -> Luma<u8> {
        match qr.index((i as usize, j as usize)) {
            Color::Light => Luma::from([0]),
            Color::Dark => Luma::from([255]),
        }
    }

    fn clip(&self, x: &Luma<u8>, y: &Luma<u8>) -> Luma<u8> {
        let n = if x[0] < y[0] - 127 {
            0
        }
        else if x[0] - 127 > y[0] {
            255
        }
        else {
            x[0] - y[0]
        };
        Luma::from([n])
    }

    fn unit_step(&self, x: &Luma<u8>) -> Luma<u8> {
        let n = if x[0] > 127 { 255 } else { 0 };
        Luma::from([n])
    }
}

impl QrImage {
    // replicate = (Flatten[ConstantArray[#, {3, 3}], {{3, 1}, {4, 2}}] &);
    //
    // refineqr[qrdat_] :=
    // Block[{qrd = qrdat,
    // d = Length[
    // qrdat]},(*Corner*)(qrd[[#1 ;; 24 #1 ;; #1, #2 ;; 24 #2 ;; #2]] =
    // replicate[
    // qrd[[2 #1 ;; 23 #1 ;; 3 #1,
    // 2 #2 ;; 23 #2 ;; 3 #2]]]) & @@@ {{1, 1}, {1, -1}, {-1, 1}};
    // (*Edge*)qrd[[22 ;; d - 21, 19 ;; 21]] =
    // Transpose[
    // qrd[[19 ;; 21, 22 ;; d - 21]] =
    // replicate[{Mod[Range[(d + 1)/3 - 14], 2]}]];
    // qrd]
    //

    fn refine(&self, qrdat: Vec<Vec<f32>>) {
        unimplemented!()
    }
}

impl QrImage {
    // refineqr[qrdat_] :=
    // Block[{qrd = qrdat, d = Length[qrdat],
    // temp = Fold[ArrayPad[#1, 1, #2] &, {{{0}}, 1, 0}], p},
    // p = Position[
    // Round@ListCorrelate[temp,
    // qrdat[[2 ;; ;; 3, 2 ;; ;; 3]], {{1, 1}, {-1, -1}}, 0,
    // Abs@*Subtract], 0, 2];
    // (*Corner*)(qrd[[#1 ;; 24 #1 ;; #1, #2 ;; 24 #2 ;; #2]] =
    // replicate[
    // qrd[[2 #1 ;; 23 #1 ;; 3 #1,
    // 2 #2 ;; 23 #2 ;; 3 #2]]]) & @@@ {{1, 1}, {1, -1}, {-1, 1}};
    // (*Edge*)qrd[[22 ;; d - 21, 19 ;; 21]] =
    // Transpose[
    // qrd[[19 ;; 21, 22 ;; d - 21]] =
    // replicate[{Mod[Range[(d + 1)/3 - 14], 2]}]];
    // (*Special*)(qrd[[3 #1 - 2 ;; 3 #1 + 12, 3 #2 - 2 ;; 3 #2 + 12]] =
    // replicate@temp) & @@@ p;
    // qrd]
    //

    fn refine_enhanced(&self, r: Vec<Vec<f32>>) {
        unimplemented!()
    }
}
