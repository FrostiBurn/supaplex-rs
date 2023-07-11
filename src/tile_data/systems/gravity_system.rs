//let tile_down = grid.got(src.trans((0, 1))).map(|entity| entity.tile).unwrap_or_else(|| TileType::None);

use macroquad::texture::Texture2D;

use crate::{
    grid::{Coord, FCoord, Grid},
    level::LevelData,
    tile_data::{
        tile::Tile,
        tile_interaction::TileInteraction,
        tile_move::TileMove,
        tile_state::TileState,
        tile_type::{simple_draw, TileType},
        tile_update::Updateable,
    },
};

use super::explode_system::set_area_explode;

pub fn gravity_system(coord: &Coord, grid: &mut Grid, level_data: &mut LevelData) {
    let tile = grid.get(coord);

    if tile.upd > 0.0 {
        return;
    }

    let coord_left = &coord.offset(&TileMove::Left);
    let coord_down = &coord.offset(&TileMove::Down);
    let coord_right = &coord.offset(&TileMove::Right);
    let coord_left_down = &coord.offset(&TileMove::Left).offset(&TileMove::Down);
    let coord_right_down = &coord.offset(&TileMove::Right).offset(&TileMove::Down);

    match grid.get(coord_down).typ {
        TileType::Empty => {
            let upd = tile.upd.move_update();
            let tile = Tile::mov_state(tile, upd, TileMove::Down, TileState::Dangerous);
            grid.set(coord_down, tile);
            grid.set(coord, Tile::transitory(upd, TileType::Empty));
        }
        TileType::Infotron
        | TileType::RAMChipsBase
        | TileType::RAMChipsLeft
        | TileType::RAMChipsRight
        | TileType::RAMChipsUp
        | TileType::RAMChipsDown
        | TileType::Zonk => {
            if grid.get(coord_left).typ == TileType::Empty
                && grid.get(coord_left_down).typ == TileType::Empty
            {
                let upd = tile.upd.move_update();
                let tile = Tile::mov_state(tile, upd, TileMove::Left, TileState::Dangerous);
                grid.set(coord_left, tile);
                grid.set(coord, Tile::transitory(upd, TileType::Empty));
            } else if grid.get(coord_right).typ == TileType::Empty
                && grid.get(coord_right_down).typ == TileType::Empty
            {
                let upd = tile.upd.move_update();
                let tile = Tile::mov_state(tile, upd, TileMove::Right, TileState::Dangerous);
                grid.set(coord_right, tile);
                grid.set(coord, Tile::transitory(upd, TileType::Empty));
            } else {
                let tile = grid.get_mut_unchecked(coord);
                tile.upd.dont_update();
                tile.mov = TileMove::None;
                tile.state = match tile.typ {
                    TileType::Infotron => TileState::Eatable,
                    _ => TileState::Moveable,
                };
            }
        }
        TileType::Electron
        | TileType::Murphy
        | TileType::SnikSnak
        | TileType::YellowUtilityDisk
        | TileType::OrangeUtilityDisk => {
            if tile.mov == TileMove::Down && grid.get(coord_down).int != TileInteraction::Pushing {
                set_area_explode(coord, grid, &mut level_data.game_state);
            }
        }
        _ => {
            let tile = grid.get_mut_unchecked(coord);
            tile.upd.dont_update();
            tile.mov = TileMove::None;
            tile.state = match tile.typ {
                TileType::Infotron => TileState::Eatable,
                _ => TileState::Moveable,
            };
        }
    }
}

pub fn draw_gravity_system(tile: &Tile, fcoord: &FCoord, texture: Texture2D) {
    let smooth_dst = fcoord.offset_time(tile);

    let x = match tile.mov {
        TileMove::Right => 7 - (tile.upd * 8.0f32).floor() as i16,
        TileMove::Left => (tile.upd * 8.0f32).floor() as i16,
        _ => 0,
    };

    let mut coord = tile.typ.def_tex_pos();
    coord.x = x;

    simple_draw(texture, &smooth_dst, &coord, false);
}
