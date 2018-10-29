use enums::MeasureMode;

pub const MAX_CACHED_RESULT_COUNT: usize = 16;

pub struct CachedMeasurement {
    available_width: f32,
    available_height: f32,
    width_measure_mode: MeasureMode,
    height_measure_mode: MeasureMode,
    computed_width: f32,
    computed_height: f32,
}

impl CachedMeasurement {
    pub fn new() -> CachedMeasurement {
        CachedMeasurement {
            available_width: 0.0,
            available_height: 0.0,
            width_measure_mode: MeasureMode::Undefined,
            height_measure_mode: MeasureMode::Undefined,
            computed_width: 0.0,
            computed_height: 0.0,
        }
    }
}

impl PartialEq for CachedMeasurement {
    fn eq(&self, other: &CachedMeasurement) -> bool {
        self.width_measure_mode == other.width_measure_mode &&
        self.height_measure_mode == other.height_measure_mode &&
        self.available_width == other.available_width &&
        self.available_height == other.available_height &&
        self.computed_width == other.computed_width &&
        self.computed_height == other.computed_height
    }
}