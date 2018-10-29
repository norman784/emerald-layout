#[derive(Eq, PartialEq)]
pub enum FlexDirection {
    Column,
    ColumnReverse,
    Row,
    RowReverse,
}

impl FlexDirection {
    pub fn value(&self) -> usize {
        match self {
            FlexDirection::Column => 0,
            FlexDirection::ColumnReverse => 1,
            FlexDirection::Row => 2,
            FlexDirection::RowReverse => 3,
        }
    }

    pub fn is_row(&self) -> bool {
        return self == FlexDirection::Row || self == FlexDirection::RowReverse
    }
}