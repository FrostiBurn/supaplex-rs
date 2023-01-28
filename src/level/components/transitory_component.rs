use crate::tile::Tile;

#[derive(Clone, Copy, Debug)]
pub enum TransitoryComponent {
    None,
    Base,
    Infotron,
    RedUtilityDisk,
}

impl TransitoryComponent {
    pub fn from_tile(tile: &Tile) -> Self {
        match tile {
            Tile::Base => Self::Base,
            Tile::Infotron(_, _) => Self::Infotron,
            Tile::RedUtilityDisk(_) => Self::RedUtilityDisk,
            _ => Self::None,
        }
    }
}
