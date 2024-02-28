#[cfg(test)]
mod tests {
    use crate::flex::{Direction, Flexbox, Wrap};

    #[test]
    fn test_flex() {
        let style: String = Flexbox {
            direction: Direction::Row,
            wrap: Wrap::NoWrap,
        }.into();
        let css = "display: flex; \
        flex-direction: row; \
        flex-wrap: nowrap;";
        assert_eq!(style, css);
    }
}

pub struct Flexbox {
    pub direction: Direction,
    pub wrap: Wrap
}

impl From<Flexbox> for String {
    fn from(flex: Flexbox) -> Self {
        format!("display: flex; {} {}",
                String::from(flex.direction),
                String::from(flex.wrap))
    }
}

pub enum Direction { Row, Column, RowReverse, ColumnReverse }

impl Default for Direction {
    fn default() -> Self {
        Self::Row
    }
}

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

pub enum Wrap {
    NoWrap,
    Wrap,
    WrapReverse
}

impl Default for Wrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

impl From<Wrap> for String {
    fn from(wrap: Wrap) -> Self {
        format!("flex-wrap: {};", match wrap {
            Wrap::NoWrap => "nowrap",
            Wrap::Wrap => "wrap",
            Wrap::WrapReverse => "wrap-reverse"
        })
    }
}
