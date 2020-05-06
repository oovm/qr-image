mod drawer;
mod renderer;

pub use crate::drawer::Canvas;
pub use image::{Luma, Rgb};
pub use qrcode::{types::QrError, EcLevel, QrCode, Version};

pub type QrResult<T> = std::result::Result<T, QrError>;

#[derive(Debug, Clone)]
pub struct QrImage {
    pub qr_version: Version,
    pub ec_level: EcLevel,
    pub dark_color: Rgb<u8>,
    pub light_color: Rgb<u8>,
    pub enhanced: bool,
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
