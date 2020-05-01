mod renderer;

pub use image::Luma;
pub use qrcode::{types::QrError, EcLevel, QrCode, Version};

pub type QrResult<T> = std::result::Result<T, QrError>;

#[derive(Debug, Clone)]
pub struct QrImage {
    grayscale: bool,
    qr_version: Version,
    ec_level: EcLevel,
    enhanced: bool,
}

impl Default for QrImage {
    fn default() -> Self {
        Self { grayscale: true, qr_version: Version::Normal(1), ec_level: EcLevel::L, enhanced: false }
    }
}
