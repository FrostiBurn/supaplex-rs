use super::tile_type::TileType;

#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum TileInteraction {
    #[default]
    None,
    Eating(TileType),
    Pushing,
    Slurping,
    Tunneling(TileType),
    Moving,
    Rotating
}
