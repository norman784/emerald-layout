mod internal;

pub mod data;
pub mod enums;
pub mod layout;
pub mod node;
pub mod style;

use enums::Edge;

pub const LEADING: [Edge; 4] = [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right];
pub const TRAILING: [Edge; 4] = [Edge::Bottom, Edge::Top, Edge::Right, Edge::Left];