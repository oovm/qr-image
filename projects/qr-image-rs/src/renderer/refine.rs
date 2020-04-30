use crate::{QrImage, QrResult};
use image::{DynamicImage, GenericImageView};
use qrcode::QrCode;

impl QrImage {
    fn test(&self, data: &[u8], img: &DynamicImage) -> QrResult<()> {
        let qr = self.target_qr(data)?;
        let img = self.target_image(&qr, img);
        let _ = img;
        unimplemented!()
    }

    fn get_qr_matrix(&self, qr: &QrCode)->Vec<Vec<f32>> {
        unimplemented!()
    }

    fn get_img_matrix(&self, img: &DynamicImage)->Vec<Vec<f32>> {
        let img = match img {
            DynamicImage::ImageLuma8(img) => {img}
            _ => unreachable!()
        };

        unimplemented!()
    }
}


impl QrImage {
    // FIXME: use u8 arithmetic
    fn dithering(&self, qr: &QrCode, img: &mut DynamicImage) -> &mut DynamicImage {
        let mut qr = self.get_qr_matrix(qr);
        let mut img = self.get_img_matrix(img);
        let size = img.width();
        for j in 1..=size {
            for i in 1..=size {
                let tmp1 = if i % 3 == 2 && j % 3 == 2 {
                    qr[((i + 1) / 3, (j + 1) / 3)]
                } else {
                    self.unit_step(img[(i, j)])
                };
                let tmp2 = self.clip(img[(i, j)] - tmp1);
                img[(i, j)] = tmp1;
                img[(i, j + 1)] += 0.4375 * tmp2;

                if j != 1 {
                    img[(i + 1, j - 1)] += 0.1875 * tmp2
                }

                img[(i + 1, j)] += 0.3125 * tmp2;
                img[(i + 1, j + 1)] += 0.0625 * tmp2;
            }
        }
        return img;
    }

    fn clip(&self, x:f32) -> f32 {
        if x < -0.5 {
            -0.5
        } else if x > 0.5 {
            0.5
        } else {
            x
        }
    }

    fn unit_step(&self, x:f32) -> f32 {
        if x > 0.5 {
            1.0
        } else {
            0.0
        }
    }
}


impl QrImage {
    /*
replicate = (Flatten[ConstantArray[#, {3, 3}], {{3, 1}, {4, 2}}] &);

refineqr[qrdat_] :=
Block[{qrd = qrdat,
d = Length[
 qrdat]},(*Corner*)(qrd[[#1 ;; 24 #1 ;; #1, #2 ;; 24 #2 ;; #2]] =
  replicate[
   qrd[[2 #1 ;; 23 #1 ;; 3 #1,
     2 #2 ;; 23 #2 ;; 3 #2]]]) & @@@ {{1, 1}, {1, -1}, {-1, 1}};
(*Edge*)qrd[[22 ;; d - 21, 19 ;; 21]] =
Transpose[
qrd[[19 ;; 21, 22 ;; d - 21]] =
 replicate[{Mod[Range[(d + 1)/3 - 14], 2]}]];
qrd]

 */

    fn refine(&self, qrdat: Vec<Vec<f32>>) {
        unimplemented!()
    }
}

impl QrImage {
    /*

    refineqr[qrdat_] :=
 Block[{qrd = qrdat, d = Length[qrdat],
   temp = Fold[ArrayPad[#1, 1, #2] &, {{{0}}, 1, 0}], p},
  p = Position[
    Round@ListCorrelate[temp,
      qrdat[[2 ;; ;; 3, 2 ;; ;; 3]], {{1, 1}, {-1, -1}}, 0,
      Abs@*Subtract], 0, 2];
  (*Corner*)(qrd[[#1 ;; 24 #1 ;; #1, #2 ;; 24 #2 ;; #2]] =
      replicate[
       qrd[[2 #1 ;; 23 #1 ;; 3 #1,
         2 #2 ;; 23 #2 ;; 3 #2]]]) & @@@ {{1, 1}, {1, -1}, {-1, 1}};
  (*Edge*)qrd[[22 ;; d - 21, 19 ;; 21]] =
   Transpose[
    qrd[[19 ;; 21, 22 ;; d - 21]] =
     replicate[{Mod[Range[(d + 1)/3 - 14], 2]}]];
  (*Special*)(qrd[[3 #1 - 2 ;; 3 #1 + 12, 3 #2 - 2 ;; 3 #2 + 12]] =
      replicate@temp) & @@@ p;
  qrd]

     */

    fn refine_enhanced(&self, r: Vec<Vec<f32>>) {
        unimplemented!()
    }

}