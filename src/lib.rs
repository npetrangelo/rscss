use crate::flex::Flexbox;

mod flex;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::flex::{Direction, Flexbox, Wrap};
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_style() {
        let style: String = Style {
            display: Some(Display::Flex(
                Flexbox {
                    direction: Direction::Row,
                    wrap: Wrap::NoWrap,
                }
            )),
            align_items: AlignItems::Center,
            justify_content: Justify::Center,
        }.into();
        let css = "display: flex; \
        flex-direction: row; \
        flex-wrap: nowrap; \
        align-items: center; \
        justify-content: center;";
        assert_eq!(style, css);
    }
}

struct Style {
    display: Option<Display>,
    pub align_items: AlignItems,
    pub justify_content: Justify,
}

impl From<Style> for String {
    fn from(style: Style) -> Self {
        let display = match style.display {
            None => "".to_owned(),
            Some(display) => String::from(display)
        };
        format!("{} {} {}", display, String::from(style.align_items), String::from(style.justify_content))
    }
}

enum Display {
    Flex(Flexbox)
}

impl From<Display> for String {
    fn from(display: Display) -> Self {
        format!("{}", match display {
            Display::Flex(flex) => String::from(flex)
        })
    }
}

pub enum AlignItems {
    Stretch,
    Center,
    Start,
    End
}

impl From<AlignItems> for String {
    fn from(alignment: AlignItems) -> Self {
        format!("align-items: {};", match alignment {
            AlignItems::Stretch => "stretch",
            AlignItems::Center => "center",
            AlignItems::Start => "start",
            AlignItems::End => "end"
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
