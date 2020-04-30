use crate::{QrImage, QrResult};
use image::DynamicImage;

impl QrImage {
    pub fn export_image(&self)-> QrResult<()> {

    }

    fn img_from_matrix(&self, m: Vec<Vec<f32>>) -> DynamicImage {

    }


}

impl QrImage {
    pub fn export_svg(&self)->QrResult<()> {

    }

    fn svg_from_matrix(&self, m: Vec<Vec<f32>>) -> DynamicImage {

    }

}