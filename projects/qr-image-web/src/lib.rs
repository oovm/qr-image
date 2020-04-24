mod renderer;

pub enum CorrectionLevel {
    /// Level Low: 7% of data bytes can be restored.
    L,
    /// Level Medium: 15% of data bytes can be restored.
    M,
    /// Level Quartile: 25% of data bytes can be restored.
    Q,
    /// Level High: 30% of data bytes can be restored.
    H,
}

#[derive(Debug, Clone)]
pub struct QREmbed {
    correction_level: CorrectionLevel
}

impl Default for CorrectionLevel {
    fn default() -> Self {
        Self::L
    }
}

impl Default for QREmbed {
    fn default() -> Self {
        Self {
            correction_level: Default::default()
        }
    }
}


