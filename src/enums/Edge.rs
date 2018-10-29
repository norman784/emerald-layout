#[derive(Eq, PartialEq)]
pub enum Edge {
    Left,
    Top,
    Right,
    Bottom,
    Start,
    End,
    Horizontal,
    Vertical,
    All,
}

impl Edge {
    pub fn value(&self) -> usize {
        match self {
            Edge::Left => 0,
            Edge::Top => 1,
            Edge::Right => 2,
            Edge::Bottom => 3,
            Edge::Start => 4,
            Edge::End => 5,
            Edge::Horizontal => 6,
            Edge::Vertical => 7,
            Edge::All => 8,
        }
    }
}