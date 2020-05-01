use crate::{QrImage, QrResult};
use image::DynamicImage;

impl QrImage {
    pub fn export_image(&self) -> QrResult<()> {
        unimplemented!()
    }

    fn img_from_matrix(&self, m: Vec<Vec<f32>>) -> DynamicImage {
        unimplemented!()
    }
}

impl QrImage {
    pub fn export_svg(&self) -> QrResult<()> {
        unimplemented!()
    }

    fn svg_from_matrix(&self, m: Vec<Vec<f32>>) -> DynamicImage {
        unimplemented!()
    }
}
