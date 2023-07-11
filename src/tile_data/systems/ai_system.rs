use macroquad::{
    prelude::{Rect, Vec2, WHITE},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
    grid::{Coord, FCoord, Grid},
    level::LevelData,
    tile_data::{
        tile::Tile, tile_interaction::TileInteraction, tile_move::TileMove, tile_type::TileType,
        tile_update::Updateable,
    },
};

use super::explode_system::set_area_explode;

pub fn ai_system(coord: &Coord, grid: &mut Grid, level_data: &mut LevelData) {
    let ai_tile = grid.get(coord);

    let left = ai_tile.mov.rotate_left();
    let right = ai_tile.mov.rotate_right();

    let coord_left = &coord.offset(&left);
    let coord_front = &coord.offset(&ai_tile.mov);
    let coord_right = &coord.offset(&right);

    if ai_tile.upd <= 0.0 {
        if ai_tile.mov == ai_tile.mov2 {
            if grid.get(coord_left).typ == TileType::Empty {
                let ai = grid.get_mut_unchecked(coord);
                ai.upd += 0.25;
                ai.mov = left;
                ai.int = TileInteraction::Rotating;
            } else if grid.get(coord_left).typ == TileType::Murphy {
                set_area_explode(coord, grid, &mut level_data.game_state);
            } else if grid.get(coord_front).typ == TileType::Empty {
                let upd = ai_tile.upd.move_update();
                let ai = Tile::ai(
                    ai_tile,
                    upd,
                    ai_tile.mov,
                    ai_tile.mov2,
                    TileInteraction::Moving,
                );
                grid.set(coord_front, ai);
                grid.set(coord, Tile::transitory(upd, TileType::Empty));
            } else if grid.get(coord_front).typ == TileType::Murphy {
                set_area_explode(coord, grid, &mut level_data.game_state);
            } else if grid.get(coord_right).typ == TileType::Empty {
                let ai = grid.get_mut_unchecked(coord);
                ai.upd += 0.25;
                ai.mov = right;
                ai.int = TileInteraction::Rotating;
            } else if grid.get(coord_right).typ == TileType::Murphy {
                set_area_explode(coord, grid, &mut level_data.game_state);
            } else {
                let ai = grid.get_mut_unchecked(coord);
                ai.upd += 0.25;
                ai.mov = left;
                ai.int = TileInteraction::Rotating;
            }
        } else if grid.get(coord_front).typ == TileType::Empty {
            let upd = ai_tile.upd.move_update();
            let ai = Tile::ai(
                ai_tile,
                upd,
                ai_tile.mov,
                ai_tile.mov,
                TileInteraction::Moving,
            );
            grid.set(coord_front, ai);
            grid.set(coord, Tile::transitory(upd, TileType::Empty));
        } else if grid.get(coord_front).typ == TileType::Murphy {
            set_area_explode(coord, grid, &mut level_data.game_state);
        } else {
            let ai = grid.get_mut_unchecked(coord);
            ai.upd += 0.25;
            ai.mov2 = ai.mov;
            ai.int = TileInteraction::Rotating;
        }
    } else if ai_tile.upd <= 0.125 && ai_tile.mov == ai_tile.mov2 {
        if grid.get(coord_left).typ == TileType::Empty || grid.get(coord_left).typ == TileType::Murphy {
            let ai = grid.get_mut_unchecked(coord);
            ai.upd += 0.25;
            ai.mov = left;
            ai.int = TileInteraction::Rotating;
        } else if grid.get(coord_front).typ == TileType::Empty || grid.get(coord_front).typ == TileType::Murphy {
        } else if grid.get(coord_right).typ == TileType::Empty || grid.get(coord_right).typ == TileType::Murphy {
            let ai = grid.get_mut_unchecked(coord);
            ai.upd += 0.25;
            ai.mov = right;
            ai.int = TileInteraction::Rotating;
        } else {
            let ai = grid.get_mut_unchecked(coord);
            ai.upd += 0.25;
            ai.mov = left;
            ai.int = TileInteraction::Rotating;
        }
    } else if grid.get(coord).upd < 0.0 {
        grid.get_mut_unchecked(coord).upd.dont_update();
    }
}

pub fn draw_ai_system(ai: &Tile, fcoord: &FCoord, texture: Texture2D) {
    let x = match (ai.mov, ai.mov2) {
        (TileMove::Up, TileMove::Right) => 3,
        (TileMove::Up, TileMove::Left) => 0,
        (TileMove::Right, TileMove::Up) => 3,
        (TileMove::Right, TileMove::Down) => 2,
        (TileMove::Down, TileMove::Right) => 2,
        (TileMove::Down, TileMove::Left) => 1,
        (TileMove::Left, TileMove::Up) => 0,
        (TileMove::Left, TileMove::Down) => 1,
        (TileMove::Up, _) => 4,
        (TileMove::Down, _) => 4,
        (TileMove::Left, _) => 8,
        (TileMove::Right, _) => 8,
        _ => return,
    };

    let flip_x = ai.mov == TileMove::Left && ai.mov2 == TileMove::Left;
    let flip_y = ai.mov == TileMove::Down && ai.mov2 == TileMove::Down;

    let time = if (ai.upd <= 1.0 && ai.upd > 0.875) || (ai.upd <= 0.125 && ai.upd > 0.0) {
        0
    } else if (ai.upd <= 0.875 && ai.upd > 0.75) || (ai.upd <= 0.25 && ai.upd > 0.125) {
        1
    } else if (ai.upd <= 0.75 && ai.upd > 0.625) || (ai.upd <= 0.375 && ai.upd > 0.25) {
        2
    } else if (ai.upd <= 0.625 && ai.upd > 0.5) || (ai.upd <= 0.5 && ai.upd > 0.375) {
        3
    } else {
        0
    };

    let offset = if ai.mov == ai.mov2 { time } else { 0 };

    let smooth_dst = if ai.int == TileInteraction::Moving {
        fcoord.offset_time(ai)
    } else {
        *fcoord
    };

    draw_texture_ex(
        texture,
        smooth_dst.x * 16.0,
        smooth_dst.y * 16.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 16.0, y: 16.0 }),
            source: Some(Rect {
                x: (x + offset) as f32 * 17.0,
                y: 12_f32 * 17.0,
                w: 16.0,
                h: 16.0,
            }),
            rotation: 0.0,
            flip_x,
            flip_y,
            pivot: None,
        },
    );
}
