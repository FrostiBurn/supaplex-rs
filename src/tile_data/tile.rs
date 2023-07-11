use macroquad::texture::Texture2D;

use crate::grid::FCoord;

use super::{
    systems::{
        ai_system::draw_ai_system, explode_system::draw_explode_system,
        gravity_system::draw_gravity_system, murphy_system::draw_murphy,
        transitory_system::draw_transitory,
    },
    tile_interaction::TileInteraction,
    tile_move::TileMove,
    tile_state::TileState,
    tile_type::{simple_draw, TileType},
    tile_update::{TileUpdate, Updateable},
};

#[derive(Clone)]
pub struct Tile {
    pub typ: TileType,
    pub state: TileState,
    pub mov: TileMove,
    pub mov2: TileMove,
    pub int: TileInteraction,
    pub upd: TileUpdate,
}

impl Tile {
    pub fn update_time(&mut self, time: f32) {
        if let TileType::Bug
        | TileType::Electron
        | TileType::Infotron
        | TileType::Murphy
        | TileType::OrangeUtilityDisk
        | TileType::RedUtilityDisk
        | TileType::Terminal
        | TileType::Transitory
        | TileType::YellowUtilityDisk
        | TileType::SnikSnak
        | TileType::Zonk
        | TileType::Explosion
        | TileType::Explosion2 = self.typ
        {
            self.upd.normal_update(time);
        }
    }

    pub fn draw(&self, fcoord: &FCoord, texture: Texture2D) {
        match self.typ {
            TileType::Murphy => draw_murphy(self, fcoord, texture),
            TileType::SnikSnak => draw_ai_system(self, fcoord, texture),
            TileType::Transitory => draw_transitory(self, fcoord, texture),
            TileType::Infotron | TileType::Zonk => draw_gravity_system(self, fcoord, texture),
            TileType::Explosion | TileType::Explosion2 => {
                draw_explode_system(self, fcoord, texture)
            }
            TileType::OrangeUtilityDisk | TileType::YellowUtilityDisk => {
                let fcoord = fcoord.offset_time(self);
                simple_draw(texture, &fcoord, &self.typ.def_tex_pos(), false);
            }
            TileType::None | TileType::Empty => {}
            _ => simple_draw(texture, fcoord, &self.typ.def_tex_pos(), false),
        }
    }

    pub fn transitory(upd: TileUpdate, typ: TileType) -> Self {
        Self {
            upd,
            typ: TileType::Transitory,
            state: TileState::Destructible,
            int: TileInteraction::Eating(typ),
            ..Default::default()
        }
    }

    pub fn moving(&self, upd: TileUpdate, mov: TileMove) -> Self {
        Self {
            upd,
            typ: self.typ,
            state: self.state,
            int: self.int,
            mov,
            ..Default::default()
        }
    }

    pub fn mov_state(&self, upd: TileUpdate, mov: TileMove, state: TileState) -> Self {
        Self {
            upd,
            typ: self.typ,
            state,
            int: self.int,
            mov,
            ..Default::default()
        }
    }

    pub fn murphy(upd: TileUpdate, mov: TileMove, mov2: TileMove, int: TileInteraction) -> Self {
        Self {
            upd,
            typ: TileType::Murphy,
            state: TileState::Destructible,
            int,
            mov,
            mov2,
        }
    }

    pub fn ai(&self, upd: TileUpdate, mov: TileMove, mov2: TileMove, int: TileInteraction) -> Self {
        Self {
            upd,
            typ: self.typ,
            state: self.state,
            mov,
            mov2,
            int,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            upd: 0.0,
            typ: TileType::None,
            state: TileState::Indestructible,
            int: TileInteraction::None,
            mov: TileMove::None,
            mov2: TileMove::Left,
        }
    }
}
