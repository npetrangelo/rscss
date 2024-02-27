pub struct Flexbox {
    pub direction: Direction,
    pub align_items: Alignment,
    pub justify_content: Justify,
    pub wrap: Wrap
}

impl From<Flexbox> for String {
    fn from(flex: Flexbox) -> Self {
        format!("display: flex; {} {} {} {}",
                String::from(flex.direction),
                String::from(flex.align_items),
                String::from(flex.justify_content),
                String::from(flex.wrap))
    }
}

pub enum Direction { Row, Column, RowReverse, ColumnReverse }

impl From<Direction> for String {
    fn from(direction: Direction) -> Self {
        format!("flex-direction: {};", match direction {
            Direction::Row => "row",
            Direction::Column => "column",
            Direction::RowReverse => "row-reverse",
            Direction::ColumnReverse => "column-reverse"
        })
    }
}
pub enum Alignment {
    Center
}

impl From<Alignment> for String {
    fn from(alignment: Alignment) -> Self {
        format!("align-items: {};", match alignment {
            Alignment::Center => "center"
        })
    }
}

pub enum Justify {
    Start,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly
}

impl From<Justify> for String {
    fn from(justify: Justify) -> Self {
        format!("justify-content: {};", match justify {
            Justify::Start => "start",
            Justify::Center => "center",
            Justify::SpaceBetween => "space-between",
            Justify::SpaceAround => "space-around",
            Justify::SpaceEvenly => "space-evenly"
        })
    }
}

pub enum Wrap {
    NoWrap
}

impl From<Wrap> for String {
    fn from(wrap: Wrap) -> Self {
        format!("flex-wrap: {};", match wrap {
            Wrap::NoWrap => "nowrap"
        })
    }
}
