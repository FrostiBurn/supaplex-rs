use crate::tile::{PortsType, Tile};

#[derive(Clone, Copy, Debug)]
pub enum InteractionComponent {
    None,
    Pushing,
    Eating(InteractionTile),
    Slurping,
    Tunneling(PortsType),
}

#[derive(Clone, Copy, Debug)]
pub enum InteractionTile {
    None,
    Base,
    Infotron,
    RedUtilityDisk,
}

impl InteractionTile {
    pub const fn from_tile(tile: &Tile) -> Self {
        match tile {
            Tile::Base => Self::Base,
            Tile::Infotron(_, _) => Self::Infotron,
            Tile::RedUtilityDisk(_) => Self::RedUtilityDisk,
            _ => Self::None,
        }
    }
}
