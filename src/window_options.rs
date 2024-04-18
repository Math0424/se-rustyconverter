use std::path::Display;

#[derive(Debug, Clone, Default)]
pub enum LCDSize {
    Custom,
    #[default]
    Square,
    Wide,
    TextPanel,
    DLCWide,
}

#[derive(Debug, Clone, Default)]
pub enum BitMode {
    #[default]
    ThreeBit,
    FiveBit,
}

#[derive(Debug, Clone, Default)]
pub enum DitherMode {
    None,
    Riemersma,
    #[default]
    FloydSteinberg,
}

#[derive(Debug, Clone, Default)]
pub enum InterpolationMode {
    #[default]
    None,
    Bilinear,
    Blend,
    Mesh,
    Nearest,
    Spline,
}

#[derive(Debug, Clone)]
pub enum WindowType {
    LCDConverter {
        selected_file: Option<String>,
        dither: DitherMode,
        interpolation: InterpolationMode,
        bit_mode: BitMode,
        selected_lcd: LCDSize,
        size_x: usize,
        size_y: usize,
    },
    BlueprintConverter,
    LCDGifConverter,
    SpraysModConverter,
    DDSConverter,
}

impl WindowType {
    pub const ALL: [WindowType; 5] = [
        WindowType::LCDConverter {
            selected_file: None, 
            dither: DitherMode::FloydSteinberg, 
            interpolation: InterpolationMode::Bilinear, 
            bit_mode: BitMode::ThreeBit, 
            selected_lcd: LCDSize::Square, 
            size_x: 0, 
            size_y: 0, 
        },
        WindowType::BlueprintConverter,
        WindowType::LCDGifConverter,
        WindowType::SpraysModConverter,
        WindowType::DDSConverter,
    ];
}

impl Default for WindowType {
    fn default() -> Self {
        Self::ALL[0].clone()
    }
}

impl std::fmt::Display for WindowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title())
    }
}