use colored::ColoredString;
use colored::Colorize;
use CubieColour as CC;

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
