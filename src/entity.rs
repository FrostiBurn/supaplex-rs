use crate::{
    bug_state::BugState,
    gravity::Gravity,
    murphy::Murphy,
    tiles::{Moveable, Tile},
};

#[derive(Clone, Copy)]
pub struct Entity {
    pub tile: Tile,
    pub moveable: Moveable,
    pub is_destructible: bool,
    pub is_moveable: bool,
    pub is_eatable: bool,
    pub gravity: Option<Gravity>,
    pub bug_state: Option<BugState>,
    pub murphy: Option<Murphy>,
}

impl Entity {
    pub fn new(
        tile: Tile,
        is_destructible: bool,
        is_moveable: bool,
        is_eatable: bool,
        gravity: Option<Gravity>,
        bug_state: Option<BugState>,
        murphy: Option<Murphy>,
    ) -> Self {
        Self {
            tile,
            moveable: Moveable::Stationary,
            is_destructible,
            is_moveable,
            is_eatable,
            gravity,
            bug_state,
            murphy,
        }
    }

    pub fn from(entity: Entity, moveable: Moveable) -> Self {
        Self {
            tile: entity.tile,
            moveable: moveable,
            is_destructible: entity.is_destructible,
            is_moveable: entity.is_moveable,
            is_eatable: entity.is_eatable,
            gravity: entity.gravity,
            bug_state: entity.bug_state,
            murphy: entity.murphy,
        }
    }

    pub fn from_tile(tile: Tile) -> Self {
        match tile {
            Tile::Base => Entity::new(tile, true, false, true, None, None, None),
            Tile::Bug => Entity::new(tile, true, false, true, None, Some(BugState::new()), None),
            Tile::Electron => Entity::new(tile, true, false, false, None, None, None),
            Tile::Empty => Entity::new(tile, true, false, true, None, None, None),
            Tile::Exit => Entity::new(tile, true, false, false, None, None, None),
            Tile::Hardware(_) => Entity::new(tile, false, false, false, None, None, None),
            Tile::Infotron => Entity::new(
                tile,
                true,
                false,
                true,
                Some(Gravity { slideable: true }),
                None,
                None,
            ),
            Tile::Murphy => Entity::new(tile, true, false, false, None, None, Some(Murphy)),
            Tile::None => Entity::new(tile, false, false, false, None, None, None),
            Tile::Ports(_) => Entity::new(tile, true, false, false, None, None, None),
            Tile::RAMChips(_) => Entity::new(tile, true, false, false, None, None, None),
            Tile::SnikSnak => Entity::new(tile, true, false, false, None, None, None),
            Tile::Terminal => Entity::new(tile, true, false, false, None, None, None),
            Tile::UtilityDisks(_) => Entity::new(
                tile,
                true,
                true,
                true,
                Some(Gravity { slideable: true }),
                None,
                None,
            ),
            Tile::Zonk => Entity::new(
                tile,
                true,
                true,
                false,
                Some(Gravity { slideable: true }),
                None,
                None,
            ),
        }
    }
}
