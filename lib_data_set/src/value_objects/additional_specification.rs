#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum AdditionalSpecification {
    Fast,
    Slow,
    White,
    NoPadding
}