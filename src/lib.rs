use crate::flex::Flexbox;

mod flex;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::flex::{AlignItems, Direction, Flexbox, Justify, Wrap};
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
                    align_items: AlignItems::Center,
                    justify_content: Justify::Center,
                    wrap: Wrap::NoWrap,
                }
            )),
        }.into();
        let css = "display: flex; \
        flex-direction: row; \
        align-items: center; \
        justify-content: center; \
        flex-wrap: nowrap;";
        assert_eq!(style, css);
    }
}

struct Style {
    display: Option<Display>,
}

impl From<Style> for String {
    fn from(style: Style) -> Self {
        match style.display {
            None => "".to_owned(),
            Some(display) => String::from(display)
        }
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
