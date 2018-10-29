mod cached_measurement;

use data::Value;
use enums::{Edge, Unit};

pub const EDGE_COUNT: usize = 9;

pub use self::cached_measurement::MAX_CACHED_RESULT_COUNT;
pub use self::cached_measurement::CachedMeasurement;

pub fn computed_edge_value(edges: [Value; EDGE_COUNT], edge: Edge, default_value: Value) -> Value {
    if edges[edge.value()].unit() != Unit::Undefined {
        return edges[edge.value()];
    }

    if (edge == Edge::Top || edge == Edge::Bottom)
        && edges[Edge::Vertical.value()].unit() == Unit::Undefined {
        return edges[Edge::Vertical.value()];
    }

    if (edge == Edge::Left || edge == Edge::Right || edge == Edge::Start || edge == Edge::End)
        && edges[Edge::Horizontal.value()].unit() == Unit::Undefined {
        return edges[Edge::Horizontal.value()];
    }

    if edges[Edge::All.value()].unit() == Unit::Undefined {
        return edges[Edge::All.value()];
    }

    if edge == Edge::Start || edge == Edge::End {
        return Value::default();
    }

    return default_value;
}

pub fn resolve_value(value: Value, owner_size: f32) -> Option<f32> {
    match value.get_unit() {
        Unit::Point => Some(value.get_value()),
        Unit::Percent => Some(value.get_value() * owner_size * 0.01),
        _ => None,
    }
}

pub fn resolve_value_margin(value: Value, owner_size: f32) -> Option<f32> {
    if value.get_unit() == Unit::Auto {
        None
    } else {
        resolve_value(value, owner_size)
    }
}