use colored::ColoredString;
use colored::Colorize;
use CubieFace as CF;

const DEFAULT_CUBIE_CHAR: char = '■';

/// Representing a single tile on a single side of a cube.
///
/// Optionally contains a `char` that will be used instead of the default square char when rendering as text.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CubieFace {
    /// Blue CubieFace is the default for the front face.
    Blue(Option<char>),
    /// Green CubieFace is the default for the back face.
    Green(Option<char>),
    /// Orange CubieFace is the default for the right face.
    Orange(Option<char>),
    /// Red CubieFace is the default for the left face.
    Red(Option<char>),
    /// White CubieFace is the default for the up face.
    White(Option<char>),
    /// Yellow CubieFace is the default for the down face.
    Yellow(Option<char>),
}

impl CubieFace {
    /// Creates a `ColoredString` that can be terminal printed, using this `CubieFace`s custom display `char` if present, or the default square `char` if not.
    #[must_use]
    pub fn get_coloured_display_char(self) -> ColoredString {
        match self {
            CF::Blue(Some(c))
            | CF::Green(Some(c))
            | CF::Orange(Some(c))
            | CF::Red(Some(c))
            | CF::White(Some(c))
            | CF::Yellow(Some(c)) => self.colourise_string(&format!("{c}")),

            CF::Blue(None)
            | CF::Green(None)
            | CF::Orange(None)
            | CF::Red(None)
            | CF::White(None)
            | CF::Yellow(None) => self.colourise_string(&format!("{DEFAULT_CUBIE_CHAR}")),
        }
    }

    fn colourise_string(self, string: &str) -> ColoredString {
        match self {
            CF::Blue(_) => string.truecolor(0, 0, 255),
            CF::Green(_) => string.truecolor(0, 255, 0),
            CF::Orange(_) => string.truecolor(255, 127, 0),
            CF::Red(_) => string.truecolor(255, 0, 0),
            CF::White(_) => string.truecolor(255, 255, 255),
            CF::Yellow(_) => string.truecolor(255, 255, 0),
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
        let cubie = CubieFace::Red(None);
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
        let cubie = CubieFace::Red(Some('?'));
        let displayed_char = cubie
            .get_coloured_display_char()
            .normal()
            .chars()
            .next()
            .unwrap();

        assert_eq!('?', displayed_char);
    }

    macro_rules! colour_tests {
        ($($cubie_constructor:ident, $rgb:expr),* $(,)?) => {
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
                    let cubie = CubieFace::$cubie_constructor(Some('?'));
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
