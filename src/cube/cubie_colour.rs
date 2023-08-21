use colored::ColoredString;
use colored::Colorize;

#[derive(Copy, Clone)]
pub(super) enum CubieColour {
    Blue,
    Green,
    Orange,
    Red,
    White,
    Yellow,
}

impl CubieColour {
    pub(super) fn colourise_char(&self, char: char) -> ColoredString {
        self.colourise_string(char.into())
    }

    fn colourise_string(&self, string: String) -> ColoredString {
        match self {
            CubieColour::Blue => string.truecolor(0, 0, 255),
            CubieColour::Green => string.truecolor(0, 255, 0),
            CubieColour::Orange => string.truecolor(255, 127, 0),
            CubieColour::Red => string.truecolor(255, 0, 0),
            CubieColour::White => string.truecolor(255, 255, 255),
            CubieColour::Yellow => string.truecolor(255, 255, 0),
        }
    }
}
