use colored::ColoredString;
use colored::Colorize;

const DEFAULT_CUBIE_CHAR: char = '■';

#[derive(Copy, Clone, Debug)]
pub(super) enum CubieColour {
    Blue(Option<char>),
    Green(Option<char>),
    Orange(Option<char>),
    Red(Option<char>),
    White(Option<char>),
    Yellow(Option<char>),
}

impl CubieColour {
    pub(super) fn get_coloured_display_char(&self) -> ColoredString {
        match self {
            CubieColour::Blue(Some(c))
            | CubieColour::Green(Some(c))
            | CubieColour::Orange(Some(c))
            | CubieColour::Red(Some(c))
            | CubieColour::White(Some(c))
            | CubieColour::Yellow(Some(c)) => self.colourise_string(c.to_string()),

            CubieColour::Blue(None)
            | CubieColour::Green(None)
            | CubieColour::Orange(None)
            | CubieColour::Red(None)
            | CubieColour::White(None)
            | CubieColour::Yellow(None) => self.colourise_string(DEFAULT_CUBIE_CHAR.into()),
        }
    }

    fn colourise_string(&self, string: String) -> ColoredString {
        match self {
            CubieColour::Blue(_) => string.truecolor(0, 0, 255),
            CubieColour::Green(_) => string.truecolor(0, 255, 0),
            CubieColour::Orange(_) => string.truecolor(255, 127, 0),
            CubieColour::Red(_) => string.truecolor(255, 0, 0),
            CubieColour::White(_) => string.truecolor(255, 255, 255),
            CubieColour::Yellow(_) => string.truecolor(255, 255, 0),
        }
    }
}
