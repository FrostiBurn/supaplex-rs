use macroquad::texture::Texture2D;

use crate::{grid::{Grid, Coord, FCoord}, tile_data::{tile::Tile, tile_type::{TileType, simple_draw}, tile_interaction::TileInteraction}, level::LevelData};

pub fn transitory_system(coord: &Coord, grid: &mut Grid, _level_data: &mut LevelData) {
    if let Some(tile) = grid.get_mut(coord) {
        if tile.upd > 0.0 {
            return;
        }
        grid.set(coord, TileType::to_tile(TileType::Empty));
    }
}

pub fn draw_transitory(tile: &Tile, fcoord: &FCoord, texture: Texture2D) {
    let (y, offset) = match &tile.int {
        TileInteraction::Eating(tile) => match tile {
            TileType::Base => (0, 0),
            TileType::Infotron => (6, 1),
            TileType::RedUtilityDisk => (14, 0),
            _ => return,
        },
        _ => return,
    };

    let x = (7 - (tile.upd * 8.0f32).floor() as i16).max(0) + offset;

    let pos = Coord::new(x, y);

    simple_draw(texture, fcoord, &pos, false);
}


// todo add explosing interaction type and animations