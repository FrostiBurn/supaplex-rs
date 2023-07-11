use crate::grid::{ToCoord, Coord};

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub enum TileMove {
    Up,
    Right,
    Down,
    Left,
    #[default]
    None,
}

impl TileMove {
    pub const fn rotate_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::None => Self::None,
        }
    }

    pub const fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::None => Self::None,
        }
    }

    pub const fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::None => Self::None,
        }
    }
}

impl ToCoord for TileMove {
    fn to_coord(&self) -> Coord {
        match self {
            TileMove::Up => Coord::new(0, -1),
            TileMove::Right => Coord::new(1, 0),
            TileMove::Down => Coord::new(0, 1),
            TileMove::Left => Coord::new(-1, 0),
            TileMove::None => Coord::new(0, 0),
        }
    }
}