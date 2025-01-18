#[derive(Debug, Clone)]
pub enum Mode {
    Normal,
    Insert,
}

#[derive(Debug, Clone)]
pub struct Screen {
    pub mode: Mode,
}
