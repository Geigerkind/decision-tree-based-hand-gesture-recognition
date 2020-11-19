/// The values of this enum map to the values found in the collected data files.
#[derive(Debug, FromPrimitive, Eq, PartialEq, PartialOrd, Ord, Copy, Clone, EnumIter)]
#[repr(u8)]
pub enum GestureType {
    None = 0,
    LeftToRight = 1,
    RightToLeft = 2,
    TopToBottom = 3,
    BottomToTop = 4,
    NotGesture = 9,
    NotLabeled = 99
}