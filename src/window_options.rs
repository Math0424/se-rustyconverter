pub enum ConverterWindow {
    DisplayImage {
        image: i32,
    },
    ImageScale {
        max_x: i32,
        max_y: i32,
        curr_x: i32,
        curr_y: i32,
        slider_val: f32,
    },
    ImageOptions {
        dither: DitherMode,
    },
    LCDImageOptions {
        size: LCDSize,
        size_x: i32,
        size_y: i32,
        bit: BitMode,
    },
}

pub enum LCDSize {
    Custom,
    Square,
    Wide,
    TextPanel,
    DLCWide,
}

pub enum BitMode {
    ThreeBit,
    FiveBit,
}

pub enum DitherMode {
    None,
    Riemersma,
    FloydSteinberg,
}

pub enum InterpolationMode {
    None,
    Bilinear,
    Blend,
    Mesh,
    Nearest,
    Spline,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowType {
    #[default]
    LCDConverter,
    BlueprintConverter,
    LCDGifConverter,
    SpraysModConverter,
    DDSConverter,
}

impl WindowType {
    pub const ALL: [WindowType; 5] = [
        WindowType::LCDConverter,
        WindowType::BlueprintConverter,
        WindowType::LCDGifConverter,
        WindowType::SpraysModConverter,
        WindowType::DDSConverter,
    ];
}

impl std::fmt::Display for WindowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WindowType::LCDConverter => "LCDConverter",
                WindowType::BlueprintConverter => "BlueprintConverter",
                WindowType::LCDGifConverter => "LCDGifConverter",
                WindowType::SpraysModConverter => "SpraysModConverter",
                WindowType::DDSConverter => "DDSConverter",
            }
        )
    }
}