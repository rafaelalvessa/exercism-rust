use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, IntEnum, IntoEnumIterator, PartialEq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

/// Returns the numerical value associated with the given color band.
pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value()
}

/// Returns the string representing the color with the given numerical value.
pub fn value_to_color_string(value: usize) -> String {
    ResistorColor::from_int(value)
        .map(|color| format!("{:?}", color))
        .unwrap_or_else(|_| String::from("value out of range"))
}

/// Returns a vector with the different band colors.
pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
