mod drawer;
mod renderer;

pub use crate::drawer::Canvas;
pub use image::Luma;
use image::Rgb;
pub use qrcode::{types::QrError, EcLevel, QrCode, Version};

pub type QrResult<T> = std::result::Result<T, QrError>;

#[derive(Debug, Clone)]
pub struct QrImage {
    qr_version: Version,
    ec_level: EcLevel,
    dark_color: Rgb<u8>,
    light_color: Rgb<u8>,
    enhanced: bool,
}

impl Default for QrImage {
    fn default() -> Self {
        Self {
            qr_version: Version::Normal(2),
            ec_level: EcLevel::L,
            dark_color: Rgb([0, 0, 0]),
            light_color: Rgb([255, 255, 255]),
            enhanced: true,
        }
    }
}
