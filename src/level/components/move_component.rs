#[derive(Clone, Copy, Debug)]
pub enum MoveComponent {
    Stationary,
    Up,
    Down,
    Left,
    Right,
}

impl MoveComponent {
    pub fn from_i8(value: i8) -> Self {
        match value {
            1 => Self::Right,
            -1 => Self::Left,
            2 => Self::Down,
            -2 => Self::Up,
            _ => Self::Stationary,
        }
    }

    pub fn as_tuple(&self) -> (i16, i16) {
        match self {
            Self::Stationary => (0, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    pub fn rotate_left(&self) -> MoveComponent {
        match self {
            Self::Stationary => Self::Stationary,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn rotate_right(&self) -> MoveComponent {
        match self {
            Self::Stationary => Self::Stationary,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}
