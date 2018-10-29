use data::Value;
use enums::{Align, Direction, Edge, FlexDirection, NodeType, Unit};
use internal::{computed_edge_value, resolve_value, resolve_value_margin};
use layout::Layout;
use style::Style;
use {LEADING, TRAILING};

pub struct Node {
    has_new_layout: bool,
    node_type: NodeType,
    style: Style,
    layout: Layout,
    line_index: u32,
    children: Vec<Node>,
    is_dirty: bool,
    resolve_dimensions: [Value; 2],
    // TODO: research about circular references to have a reference of the parent node (maintaining safety)
    // TODO: research about the usage of context, YGPrintFunc, YGBaselineFunc, etc and see how to implement
    // TODO: research about missing properties and their usage
}

impl Node {
    pub fn new() -> Node {
        Node {
            has_new_layout: false,
            node_type: NodeType::Default,
            style: Style::new(),
            layout: Layout::new(),
            line_index: 0,
            children: Vec::new(),
            is_dirty: true,
            resolve_dimensions: [Value::default(), Value::default()],
        }
    }

    fn relative_position(&self, axis: FlexDirection, axis_size: f32) -> Option<f32> {
        if self.is_leading_position_defined(axis) {
            return self.get_leading_position(axis, axis_size)
        }

        let mut trailing_position = self.get_trailing_position(axis, axis_size);

        // TODO: check if you can reasing the value of Some() instead of creating a new instance of Some()
        if let Some(mut value) = trailing_position {
            trailing_position = Some(value * -1);
        }

        trailing_position
    }

    // Getters

    pub fn get_has_new_layout(&self) -> bool {
        self.has_new_layout
    }

    pub fn get_node_type(&self) -> NodeType {
        self.node_type
    }

    pub fn get_style(&self) -> Style {
        self.style
    }

    pub fn get_layout(&self) -> Layout {
        self.layout
    }

    pub fn get_line_index(&self) -> u32 {
        self.line_index
    }

    pub fn get_children(&self) -> Vec<Node> {
        self.children
    }

    pub fn get_child(&self, index: usize) -> Option<Node> {
        if self.children.len() <= index {
            Option::None
        } else {
            Option::Some(self.children[index])
        }
    }

    pub fn get_is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn get_resolve_dimensions(&self) -> [Value; 2] {
        self.resolve_dimensions
    }

    pub fn get_resolve_dimension(&self, index: usize) -> Option<Value> {
        if self.resolve_dimensions.len() <= index {
            Option::None
        } else {
            Option::Some(self.resolve_dimensions[index])
        }
    }

    pub fn is_leading_position_defined(&self, axis: FlexDirection) -> bool {
        if axis.is_row() {
            computed_edge_value(
                self.style.get_position(),Edge::Start,Value::default()
            ).get_unit() != Unit::Undefined
            || computed_edge_value(
                self.style.get_position(),LEADING[axis.value()],Value::default()
            ).get_unit() != Unit::Undefined
        } else {
            false
        }
    }

    pub fn is_trailing_position_defined(&self, axis: FlexDirection) -> bool {
        if axis .is_row() {
                computed_edge_value(
                    self.style.get_position(),Edge::End,Value::default()
                ).get_unit() != Unit::Undefined
                    || computed_edge_value(
                    self.style.get_position(),TRAILING[axis.value()],Value::default()
                ).get_unit() != Unit::Undefined
        } else {
            false
        }
    }

    pub fn get_leading_position(&self, axis: FlexDirection, axis_size: f32) -> Option<f32> {
        if axis.is_row() {
            let leading_position = computed_edge_value(
                self.style.get_position(),
                Edge::Start,
                Value::default()
            );

            if leading_position.get_unit() != Unit::Undefined {
                return resolve_value(leading_position, axis_size);
            }
        }

        let leading_position = computed_edge_value(
            self.style.get_position(),
            LEADING[axis.value()],
            Value::default()
        );

        if leading_position.get_unit() == Unit::Undefined {
            Some(0.0)
        } else {
            resolve_value(leading_position, axis_size)
        }
    }

    pub fn get_trailing_position(&self, axis: FlexDirection, axis_size: f32) -> Option<f32> {
        if axis.is_row() {
            let trailing_position = computed_edge_value(
                self.style.get_position(),
                Edge::End,
                Value::default()
            );

            if trailing_position.get_unit() != Unit::Undefined {
                return resolve_value(trailing_position, axis_size);
            }
        }

        let trailing_position = computed_edge_value(
            self.style.get_position(),
            TRAILING[axis.value()],
            Value::default()
        );

        if trailing_position.get_unit() == Unit::Undefined {
            Some(0.0)
        } else {
            resolve_value(trailing_position, axis_size)
        }
    }

    pub fn get_leading_margin(&self, axis: FlexDirection, width_size: f32) -> Option<f32> {
        if axis.is_row() && self.style.get_margin()[Edge::Start.value()].get_unit() != Unit::Undefined {
            resolve_margin_value(self.style.get_margin()[Edge::Start.value()], width_size)
        } else {
            resolve_value_margin(
                computed_edge_value(
                    self.style.get_margin(),
                    LEADING[axis.value()],
                    Value::zero()
                ),
                width_size
            )
        }
    }

    pub fn get_trailing_margin(&self, axis: FlexDirection, width_size: f32) -> Option<f32> {
        if axis.is_row() && self.style.get_margin()[Edge::End.value()].get_unit() != Unit::Undefined {
            resolve_margin_value(self.style.get_margin()[Edge::End.value()], width_size)
        } else {
            resolve_value_margin(
                computed_edge_value(
                    self.style.get_margin(),
                    TRAILING[axis.value()],
                    Value::zero()
                ),
                width_size
            )
        }
    }

    pub fn get_leading_border(&self, axis: FlexDirection) -> Option<f32> {
        unimplemented!()
    }

    pub fn get_trailing_border(&self, axis: FlexDirection) -> Option<f32> {
        unimplemented!()
    }

    pub fn get_leading_padding(&self, axis: FlexDirection, width_size: f32) -> Option<f32> {
        unimplemented!()
    }

    pub fn get_trailing_padding(&self, axis: FlexDirection, width_size: f32) -> Option<f32> {
        unimplemented!()
    }

    pub fn get_leading_padding_and_border(&self, axis: FlexDirection, width_size: f32) -> Option<f32> {
        unimplemented!()
    }

    pub fn get_trailing_padding_and_border(&self, axis: FlexDirection, width_size: f32) -> Option<f32> {
        unimplemented!()
    }

    pub fn get_margin_for_axis(&self, axis: FlexDirection, width_size: f32) -> Option<f32> {
        // TODO: check Option<f32> + Option<f32> works
        self.get_leading_margin(axis, width_size) + self.get_trailing_margin(axis, width_size)
    }

    // Setters

    pub fn set_has_new_layout(&mut self, has_new_layout: bool) {
        self.has_new_layout = has_new_layout;
    }

    pub fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }

    pub fn set_style_flex_direction(&mut self, direction: FlexDirection) {
        self.style.set_flex_direction(direction);
    }

    pub fn set_style_align_content(&mut self, align_content: Align) {
        self.style.set_align_content(align_content);
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }

    pub fn set_line_index(&mut self, index: u32) {
        self.line_index = index;
    }

    pub fn set_children(&mut self, children: Vec<Node>) {
        self.children = children;
    }

    pub fn set_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
        unimplemented!();
        // TODO: check how it works https://github.com/facebook/yoga/blob/master/yoga/YGNode.cpp#L134
    }

    pub fn set_layout_last_owner_direction(&mut self, direction: Direction) {
        self.layout.set_last_owner_direction(direction);
    }

    pub fn set_layout_computed_flex_basis(&mut self, computed_flex_basis: Option<f32>) {
        self.layout.set_computed_flex_basis(computed_flex_basis);
    }

    pub fn set_layout_computed_flex_basis_generation(&mut self, computed_flex_basis_generation: u32) {
        self.layout.set_computed_flex_basis_generation(computed_flex_basis_generation);
    }

    pub fn set_layout_measured_dimensions(&mut self, measured_dimension: f32, index: usize) {
        self.layout.set_measured_dimension(measured_dimension, index);
    }

    pub fn set_layout_had_overflow(&mut self, had_overflow: bool) {
        self.layout.set_had_overflow(had_overflow);
    }

    pub fn set_layout_dimension(&mut self, dimension: f32, index: usize) {
        self.layout.set_dimension(dimension, index);
    }

    pub fn set_layout_direction(&mut self, direction: Direction) {
        self.layout.set_direction(direction);
    }

    pub fn set_layout_margin(&mut self, margin: f32, index: usize) {
        self.layout.set_margin(margin, index);
    }

    pub fn set_layout_padding(&mut self, padding: f32, index: usize) {
        self.layout.set_padding(padding, index);
    }

    pub fn set_layout_position(&mut self, position: f32, index: usize) {
        self.layout.set_position(position, index);
    }

    pub fn set_position(&mut self, direction: Direction, main_size: f32, cross_size: f32, owner_width: f32) {
        unimplemented!();
    }

    pub fn mark_dirty_and_propagate_downwards(&mut self) {
        self.set_dirty(true);
        unimplemented!();
    }

    pub fn margin_leading_value(&self, axis: FlexDirection) -> Value {
        unimplemented!();
    }

    pub fn margin_trailing_value(&self, axis: FlexDirection) -> Value {
        unimplemented!();
    }

    pub fn resolve_dimension() {
        unimplemented!();
    }

    pub fn resolve_direction(&mut self, axis: FlexDirection) -> Direction {
        unimplemented!()
    }

    pub fn clear_children(&mut self) {
        self.children = Vec::new();
    }

    // TODO: implement replace_child and remove_child(NodeRef) methods

    pub fn remove_children(&mut self, index: usize) {
        self.children.remove(index);
    }

    pub fn clone_children_if_needed(&mut self) {
        unimplemented!();
    }

    pub fn mark_dirty_and_propagate(&mut self) {
        self.set_dirty(true);
        unimplemented!();
    }

    pub fn resolve_flex_grow(&mut self) -> f32 {
        unimplemented!()
    }

    pub fn resolve_flex_shrink(&mut self) -> f32 {
        unimplemented!()
    }

    pub fn is_node_flexible() -> bool {
        unimplemented!()
    }

    pub fn is_layout_tree_equal_to_node(&self, node: Node) -> bool {
        unimplemented!()
    }

    // TODO: implement missing methods (https://github.com/facebook/yoga/blob/master/yoga/YGNode.h)
}