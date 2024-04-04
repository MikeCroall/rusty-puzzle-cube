use colored::ColoredString;
use colored::Colorize;
use CubieColour as CC;

const DEFAULT_CUBIE_CHAR: char = '■';

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) enum CubieColour {
    Blue(Option<char>),
    Green(Option<char>),
    Orange(Option<char>),
    Red(Option<char>),
    White(Option<char>),
    Yellow(Option<char>),
}

impl CubieColour {
    pub(super) fn get_coloured_display_char(self) -> ColoredString {
        match self {
            CC::Blue(Some(c))
            | CC::Green(Some(c))
            | CC::Orange(Some(c))
            | CC::Red(Some(c))
            | CC::White(Some(c))
            | CC::Yellow(Some(c)) => self.colourise_string(&format!("{c}")),

            CC::Blue(None)
            | CC::Green(None)
            | CC::Orange(None)
            | CC::Red(None)
            | CC::White(None)
            | CC::Yellow(None) => self.colourise_string(&format!("{DEFAULT_CUBIE_CHAR}")),
        }
    }

    fn colourise_string(self, string: &str) -> ColoredString {
        match self {
            CC::Blue(_) => string.truecolor(0, 0, 255),
            CC::Green(_) => string.truecolor(0, 255, 0),
            CC::Orange(_) => string.truecolor(255, 127, 0),
            CC::Red(_) => string.truecolor(255, 0, 0),
            CC::White(_) => string.truecolor(255, 255, 255),
            CC::Yellow(_) => string.truecolor(255, 255, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;
    use paste::paste;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_default_char_cubie() {
        let cubie = CubieColour::Red(None);
        let displayed_char = cubie
            .get_coloured_display_char()
            .normal()
            .chars()
            .next()
            .unwrap();

        assert_eq!(DEFAULT_CUBIE_CHAR, displayed_char);
    }

    #[test]
    fn test_custom_char_cubie() {
        let cubie = CubieColour::Red(Some('?'));
        let displayed_char = cubie
            .get_coloured_display_char()
            .normal()
            .chars()
            .next()
            .unwrap();

        assert_eq!('?', displayed_char);
    }

    macro_rules! colour_tests {
        ($($cubie_constructor:ident, $rgb:expr,)*) => {
            paste! {
                colour_tests!(
                    $(
                        [<test_ $cubie_constructor:lower _cubie>] , $cubie_constructor, $rgb,
                    )*
                );
            }
        };
        ($($test_name:ident, $cubie_constructor:ident, $rgb:expr,)*) => {
            $(
                #[test]
                fn $test_name() {
                    let cubie = CubieColour::$cubie_constructor(Some('?'));
                    let display_char = cubie.get_coloured_display_char();
                    let colour_opt = display_char.fgcolor();
                    assert!(colour_opt.is_some());
                    let colour = colour_opt.unwrap();

                    let (r, g, b) = $rgb;
                    let expected_colour = Color::TrueColor { r, g, b };
                    assert_eq!(expected_colour, colour);
                }
            )*
        };
    }

    colour_tests!(
        Blue,
        (0, 0, 255),
        Green,
        (0, 255, 0),
        Orange,
        (255, 127, 0),
        Red,
        (255, 0, 0),
        White,
        (255, 255, 255),
        Yellow,
        (255, 255, 0),
    );
}
