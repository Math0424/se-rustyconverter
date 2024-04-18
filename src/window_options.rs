extern crate derive_more;
use derive_more::{Add, Display, From, Into};
use iced::widget::combo_box;

#[derive(Debug, Clone)]
pub struct LCDSize(pub String, pub usize, pub usize);

impl LCDSize {
    pub fn all() -> [LCDSize; 5] {[
        LCDSize("Custom".into(), 0, 0),
        LCDSize("Square".into(), 178, 178),
        LCDSize("Wide".into(), 178, 356),
        LCDSize("TextPanel".into(), 107, 178),
        LCDSize("DLCWide".into(), 107, 178),
    ]}
}

impl Default for LCDSize {
    fn default() -> Self {
        LCDSize::all()[1].clone()
    }
}

#[derive(Debug, Clone, Default, Display)]
pub enum BitMode {
    #[default]
    ThreeBit,
    FiveBit,
}

impl BitMode {
    pub const ALL: [BitMode; 2] = [
        BitMode::ThreeBit,
        BitMode::FiveBit,
    ];
}

#[derive(Debug, Clone, Default, Display)]
pub enum DitherMode {
    None,
    Riemersma,
    #[default]
    FloydSteinberg,
}

impl DitherMode {
    pub const ALL: [DitherMode; 3] = [
        DitherMode::None,
        DitherMode::Riemersma,
        DitherMode::FloydSteinberg,
    ];
}

#[derive(Debug, Clone, Default, Display)]
pub enum InterpolationMode {
    #[default]
    None,
    Bilinear,
    Blend,
    Mesh,
    Nearest,
    Spline,
}

impl InterpolationMode {
    pub const ALL: [InterpolationMode; 6] = [
        InterpolationMode::None,
        InterpolationMode::Bilinear,
        InterpolationMode::Blend,
        InterpolationMode::Mesh,
        InterpolationMode::Nearest,
        InterpolationMode::Spline,
    ];
}

#[derive(Debug, Clone)]
pub struct LCDWindowData {
    pub dither_options: combo_box::State<DitherMode>,
    pub interpolation_options: combo_box::State<InterpolationMode>,
    pub lcd_options: combo_box::State<LCDSize>,
    pub bit_options: combo_box::State<BitMode>,

    pub selected_file: Option<String>,
    pub dither: Option<DitherMode>,
    pub interpolation: Option<InterpolationMode>,
    pub bit_mode: Option<BitMode>,
    pub selected_lcd: Option<LCDSize>,
    pub size_x: usize,
    pub size_y: usize,
}

impl Default for LCDWindowData {
    fn default() -> Self {
        Self {
            dither_options: combo_box::State::new(DitherMode::ALL.to_vec()),
            interpolation_options: combo_box::State::new(InterpolationMode::ALL.to_vec()),
            lcd_options: combo_box::State::new(LCDSize::all().to_vec()),
            bit_options: combo_box::State::new(BitMode::ALL.to_vec()),

            selected_file: None, 
            dither: Some(DitherMode::default()),
            interpolation: Some(InterpolationMode::default()),
            bit_mode: Some(BitMode::default()),
            selected_lcd: Some(LCDSize::default()),
            size_x: 0, 
            size_y: 0, 
        }
    }
}

#[derive(Debug, Clone)]
pub enum WindowType {
    LCDConverter(LCDWindowData),
    BlueprintConverter,
    LCDGifConverter,
    SpraysModConverter,
    DDSConverter,
}

impl WindowType {
    pub fn all() -> [WindowType; 5] {[
        WindowType::LCDConverter(LCDWindowData::default()),
        WindowType::BlueprintConverter,
        WindowType::LCDGifConverter,
        WindowType::SpraysModConverter,
        WindowType::DDSConverter,
    ]}
}

impl Default for WindowType {
    fn default() -> Self {
        Self::all()[0].clone()
    }
}

impl std::fmt::Display for WindowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title())
    }
}

impl std::fmt::Display for LCDSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}