crate::helpers::simple_enum! {
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum Color {
        White,
        Black
    }
}

crate::helpers::enum_char_conv! {
    Color, ColorParseError {
        White = 'w',
        Black = 'b'
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White
        }
    }
}
