#[derive(Debug, Hash, EnumIter, Eq, PartialEq, Copy, Clone)]
pub enum ParsingMethod {
    ByAnnotation,
    ByThreshold
}