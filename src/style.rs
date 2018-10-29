use internal::EDGE_COUNT;
use data::{Value};
use enums::{Align, Direction, Display, FlexDirection, Justify, Overflow, PositionType, Wrap};

pub struct Style {
    direction: Direction,
    flex_direction: FlexDirection,
    justify_content: Justify,
    align_content: Align,
    align_items: Align,
    align_self: Align,
    position_type: PositionType,
    flex_wrap: Wrap,
    overflow: Overflow,
    display: Display,
    flex: Option<f32>,
    flex_grow: Option<f32>,
    flex_shrink: Option<f32>,
    flex_basis: Value,
    margin: [Value; EDGE_COUNT],
    position: [Value; EDGE_COUNT],
    padding: [Value; EDGE_COUNT],
    border: [Value; EDGE_COUNT],
    dimensions: [f32; 2],
    min_dimensions: [f32; 2],
    max_dimensions: [f32; 2],
    aspect_ratio: Option<f32>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            direction: Direction::Inherit,
            flex_direction: FlexDirection::Row,
            justify_content: Justify::FlexStart,
            align_content: Align::Stretch,
            align_items: Align::Stretch,
            align_self: Align::Auto,
            position_type: PositionType::Relative,
            flex_wrap: Wrap::NoWrap,
            overflow: Overflow::Visible,
            display: Display::Flex,
            flex: None,
            flex_grow: None,
            flex_shrink: None,
            flex_basis: Value::default(),
            margin: [
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default()
            ],
            position: [
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default()
            ],
            padding: [
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default()
            ],
            border: [
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default(),
                Value::default()
            ],
            dimensions: [0.0, 0.0],
            max_dimensions: [0.0, 0.0],
            min_dimensions: [0.0, 0.0],
            aspect_ratio: None,
        }
    }

    // Getters

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_flex_direction(&self) -> FlexDirection {
        self.flex_direction
    }

    pub fn get_justify_content(&self) -> Justify {
        self.justify_content
    }

    pub fn get_align_content(&self) -> Align {
        self.align_content
    }

    pub fn get_align_items(&self) -> Align {
        self.align_items
    }

    pub fn get_align_self(&self) -> Align {
        self.align_self
    }

    pub fn get_position_type(&self) -> PositionType {
        self.position_type
    }

    pub fn get_flex_wrap(&self) -> Wrap {
        self.flex_wrap
    }

    pub fn get_overflow(&self) -> Overflow {
        self.overflow
    }

    pub fn get_display(&self) -> Display {
        self.display
    }

    pub fn get_flex(&self) -> Option<f32> {
        self.flex
    }

    pub fn get_flex_grow(&self) -> Option<f32> {
        self.flex_grow
    }

    pub fn get_flex_shrink(&self) -> Option<f32> {
        self.flex_shrink
    }

    pub fn get_flex_basis(&self) -> Value {
        self.flex_basis
    }

    pub fn get_margin(&self) -> [Value; EDGE_COUNT] {
        self.margin
    }

    pub fn get_position(&self) -> [Value; EDGE_COUNT] {
        self.position
    }

    pub fn get_padding(&self) -> [Value; EDGE_COUNT] {
        self.padding
    }

    pub fn get_border(&self) -> [Value; EDGE_COUNT] {
        self.border
    }

    pub fn get_dimensions(&self) -> [f32; 2] {
        self.dimensions
    }

    pub fn get_max_dimensions(&self) -> [f32; 2] {
        self.max_dimensions
    }

    pub fn get_min_dimensions(&self) -> [f32; 2] {
        self.min_dimensions
    }

    pub fn get_aspect_ratio(&self) -> Option<f32> {
        self.aspect_ratio
    }

    // Setters

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_flex_direction(&mut self, direction: FlexDirection) {
        self.flex_direction = direction;
    }

    pub fn set_justify_content(&mut self, justify_content: Justify) {
        self.justify_content = justify_content;
    }

    pub fn set_align_content(&mut self, align: Align) {
        self.align_content = align;
    }

    pub fn set_align_items(&mut self, align: Align) {
        self.align_items = align;
    }

    pub fn set_align_self(&mut self, align: Align) {
        self.align_self = align;
    }

    pub fn set_position_type(&mut self, position_type: PositionType) {
        self.position_type = position_type;
    }

    pub fn set_flex_wrap(&mut self, wrap: Wrap) {
        self.flex_wrap = wrap;
    }

    pub fn set_overflow(&mut self, overflow: Overflow) {
        self.overflow = overflow;
    }

    pub fn set_display(&mut self, display: Display) {
        self.display = display;
    }

    pub fn set_flex(&mut self, flex: Option<f32>) {
        self.flex = flex;
    }

    pub fn set_flex_grow(&mut self, flex_grow:Option<f32>) {
        self.flex_grow = flex_grow;
    }

    pub fn set_flex_shrink(&mut self, flex_shrink:Option<f32>) {
        self.flex_shrink = flex_shrink;
    }

    pub fn set_flex_basis(&mut self, flex_basis: Value) {
        self.flex_basis = flex_basis;
    }

    pub fn set_margin(&mut self, margin: [Value; EDGE_COUNT]) {
        self.margin = margin;
    }

    pub fn set_position(&mut self, position: [Value; EDGE_COUNT]) {
        self.position = position;
    }

    pub fn set_padding(&mut self, padding: [Value; EDGE_COUNT]) {
        self.padding = padding;
    }

    pub fn set_border(&mut self, border: [Value; EDGE_COUNT]) {
        self.border = border;
    }

    pub fn set_dimensions(&mut self, dimensions: [f32; 2]) {
        self.dimensions = dimensions;
    }

    pub fn set_max_dimensions(&mut self, max_dimensions: [f32; 2]) {
        self.max_dimensions = max_dimensions;
    }

    pub fn set_min_dimensions(&mut self, min_dimensions: [f32; 2]) {
        self.min_dimensions = min_dimensions;
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: Option<f32>) {
        self.aspect_ratio = aspect_ratio;
    }
}