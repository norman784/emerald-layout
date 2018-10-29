use enums::Direction;
use internal::MAX_CACHED_RESULT_COUNT;
use internal::CachedMeasurement;

pub struct Layout {
    position: [f32; 4],
    dimensions: Option<[f32; 2]>,
    margin: [f32; 6],
    border: [f32; 6],
    padding: [f32; 6],
    direction: Direction,

    computed_flex_basis_generation: u32,
    computed_flex_basis: Option<f32>,
    had_overflow: bool,

    generation_count: u32,
    last_owner_direction: Direction,

    next_cached_measurements_index: u32,
    cached_measurements: Option<[CachedMeasurement; MAX_CACHED_RESULT_COUNT]>,
    measured_dimensions: [f32; 2],

    cached_layout: CachedMeasurement,
    // did_use_legacy_flag: bool, // TODO: check if this is really necessary
    // does_legacy_stretch_flag_affects_layout: bool, // TODO: check if this is really necessary
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            position: [0.0, 0.0, 0.0, 0.0],
            dimensions: None,
            margin: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            border: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            padding: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            direction: Direction::Inherit,
            computed_flex_basis_generation: 0,
            computed_flex_basis: None,
            had_overflow: false,
            generation_count: 0,
            last_owner_direction: Direction::Inherit,
            next_cached_measurements_index: 0,
            cached_measurements: None,
            measured_dimensions: [0.0, 0.0],
            cached_layout: CachedMeasurement::new(),
//            did_use_legacy_flag: false,
//            does_legacy_stretch_flag_affects_layout: false,
        }
    }

    // Getters
    
    pub fn get_position(&self) -> [f32; 4] {
        self.position
    }

    pub fn get_dimensions(&self) -> Option<[f32; 2]> {
        self.dimensions
    }

    pub fn get_margin(&self) -> [f32; 6] {
        self.margin
    }

    pub fn get_border(&self) -> [f32; 6] {
        self.border
    }

    pub fn get_padding(&self) -> [f32; 6] {
        self.padding
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_computed_flex_basis_generation(&self) -> u32 {
        self.computed_flex_basis_generation
    }

    pub fn get_computed_flex_basis(&self) -> Option<f32> {
        self.computed_flex_basis
    }

    pub fn had_overflow(&self) -> bool {
        self.had_overflow
    }

    pub fn get_generation_count(&self) -> u32 {
        self.generation_count
    }

    pub fn get_last_owner_direction(&self) -> Direction {
        self.last_owner_direction
    }

    pub fn get_next_cached_measurements_index(&self) -> u32 {
        self.next_cached_measurements_index
    }

    pub fn get_cached_measurements(&self) -> Option<[CachedMeasurement; MAX_CACHED_RESULT_COUNT]> {
        self.cached_measurements
    }

    pub fn get_measured_dimensions(&self) -> [f32; 2] {
        self.measured_dimensions
    }

    pub fn get_cached_layout(&self) -> CachedMeasurement {
        self.cached_layout
    }

    // Setters

    pub fn set_position(&mut self, position: f32, index: usize) {
        self.position[index] = position
    }

    pub fn set_dimensions(&mut self, dimensions: Option<[f32; 2]>) {
        self.dimensions = dimensions;
    }

    pub fn set_dimension(&mut self, measured_dimension: f32, index: usize) {
        // TODO: check how to update the value inside an Option
        match self.dimensions {
            None => {
                self.dimensions = Some([0.0, 0.0]);
                self.set_dimension(measured_dimension, index);
            },
            Some(mut dimensions) => {
                dimensions[index] = measured_dimension;
                self.dimensions = Some(dimensions);
            }
        }
    }

    pub fn set_margin(&mut self, margin: f32, index: usize) {
        self.margin[index] = margin;
    }

    pub fn set_border(&mut self, border: f32, index: usize) {
        self.border[index] = border;
    }

    pub fn set_padding(&mut self, padding: f32, index: usize) {
        self.padding[index] = padding;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_computed_flex_basis_generation(&mut self, computed_flex_basis_generation: u32) {
        self.computed_flex_basis_generation = computed_flex_basis_generation;
    }

    pub fn set_computed_flex_basis(&mut self, computed_flex_basis: Option<f32>) {
        self.computed_flex_basis = computed_flex_basis;
    }

    pub fn set_had_overflow(&mut self, had_overflow: bool) {
        self.had_overflow = had_overflow;
    }

    pub fn set_last_owner_direction(&mut self, last_owner_direction: Direction) {
        self.last_owner_direction = last_owner_direction;
    }

    pub fn set_next_cached_measurements_index(&mut self, next_cached_measurements_index: u32) {
        self.next_cached_measurements_index = next_cached_measurements_index;
    }

    pub fn set_cached_measurements(&mut self, cached_measurements: Option<[CachedMeasurement; MAX_CACHED_RESULT_COUNT]>) {
        self.cached_measurements = cached_measurements;
    }

    pub fn set_measured_dimensions(&mut self, measured_dimensions: [f32; 2]) {
        self.measured_dimensions = measured_dimensions;
    }

    pub fn set_measured_dimension(&mut self, measured_dimension: f32, index: usize) {
        self.measured_dimensions[index] = measured_dimension;
    }

    pub fn set_cached_layout(&mut self, cached_layout: CachedMeasurement) {
        self.cached_layout = cached_layout;
    }
}

impl PartialEq for Layout {
    // TODO: implement eq function (https://github.com/facebook/yoga/blob/master/yoga/YGLayout.cpp#L35)
    fn eq(&self, other: &Layout) -> bool {
        unimplemented!()
    }
}