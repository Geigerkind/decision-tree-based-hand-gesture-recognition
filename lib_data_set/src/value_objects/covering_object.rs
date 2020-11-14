#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum CoveringObject {
    Finger,
    Hand,
    Unknown,
    Arm
}