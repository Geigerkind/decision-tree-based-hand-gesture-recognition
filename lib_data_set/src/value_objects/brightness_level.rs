#[derive(Debug)]
pub enum BrightnessLevel {
    Low,
    Half,
    Full,
    High,
    Monotop,
    Monotop60,
    Monotop90,
    Ceiling,
    CeilingDim,
    CeilingVarious,
    CeilingLit, // ?
    Wall, //?
    Unknown
}