#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TileState {
    Eatable,
    Moveable,
    Dangerous,
    Tunnelable,
    Destructible,
    #[default]
    Indestructible,
}