#[derive(Debug, Clone, PartialEq)]
pub struct DetailedTiming {
    pub pixel_clock_khz: u32,
    pub horizontal_active: u16,
    pub horizontal_blanking: u16,
    pub vertical_active: u16,
    pub vertical_blanking: u16,
    pub raw: [u8; 18],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeLimits {
    pub min_vertical_hz: u8,
    pub max_vertical_hz: u8,
    pub min_horizontal_khz: u8,
    pub max_horizontal_khz: u8,
    pub max_pixel_clock_mhz: u16,
    pub raw: [u8; 18],
}

#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
    DetailedTiming(DetailedTiming),
    MonitorName(String),
    MonitorSerial(String),
    RangeLimits(RangeLimits),
    Unknown([u8; 18]),
}
