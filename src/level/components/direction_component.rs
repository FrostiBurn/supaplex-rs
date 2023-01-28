#[derive(Clone, Copy, Debug)]
pub enum DirectionComponent {
    Left,
    Right,
}

impl DirectionComponent {
    pub const fn as_bool(&self) -> bool {
        matches!(self, Self::Left)
    }
}
