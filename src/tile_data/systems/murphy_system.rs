use macroquad::{
    prelude::{is_key_down, KeyCode, Vec2},
    texture::Texture2D,
};

use crate::{
    grid::{Coord, FCoord, Grid},
    level::{GameState, LevelData},
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

pub fn murphy_system(coord: &Coord, grid: &mut Grid, level_data: &mut LevelData) -> Coord {
    if let Some(tile) = grid.get_mut(coord) {
        if tile.upd > 0.0 {
            return *coord;
        }
    }

    for mov_input in &level_data.move_priority {
        let murphy = grid.get(coord);
        let coord2 = coord.offset(mov_input);
        let tile2 = grid.get(&coord2);
        if tile2.typ == TileType::Exit && level_data.infotrons_required <= 0 {
            level_data.game_state = GameState::Finished;
        }
        if tile2.typ == TileType::Terminal {
            for y in 0..grid.height {
                for x in 0..grid.width {
                    let special_coord = Coord::new(x, y);
                    if grid.get(&special_coord).typ == TileType::YellowUtilityDisk {
                        set_area_explode(&special_coord, grid, &mut level_data.game_state);
                    }
                }
            }
            return *coord;
        }
        match &tile2.state {
            TileState::Dangerous => {
                // explode !!!
                set_area_explode(coord, grid, &mut level_data.game_state);
                //return;
            }
            TileState::Eatable => {
                let dir = match mov_input {
                    TileMove::Right | TileMove::Left => *mov_input,
                    _ => murphy.mov2,
                };
                let upd = murphy.upd.move_update();
                if tile2.typ == TileType::Infotron {
                    level_data.infotrons_required -= 1;
                } else if tile2.typ == TileType::RedUtilityDisk {
                    level_data.red_utility_disks += 1;
                }
                if is_key_down(KeyCode::Space) {
                    let murphy = Tile::murphy(upd, *mov_input, dir, TileInteraction::Slurping);
                    grid.set(&coord2, Tile::transitory(upd, tile2.typ));
                    grid.set(coord, murphy);
                    return *coord;
                } else {
                    let murphy =
                        Tile::murphy(upd, *mov_input, dir, TileInteraction::Eating(tile2.typ));
                    grid.set(&coord2, murphy);
                    grid.set(coord, Tile::transitory(upd, TileType::Empty));
                    return coord2;
                }
            }
            TileState::Moveable => {
                if mov_input == &TileMove::Left || mov_input == &TileMove::Right {
                    let coord3 = coord2.offset(mov_input);
                    let tile3 = grid.get(&coord3);
                    if tile3.typ == TileType::Empty {
                        if murphy.upd > -1.0 {
                            let murphy = grid.get_mut_unchecked(coord);
                            murphy.int = TileInteraction::Pushing;
                            murphy.mov = *mov_input;
                            murphy.mov2 = match mov_input {
                                TileMove::Right | TileMove::Left => *mov_input,
                                _ => murphy.mov2,
                            };
                            return *coord;
                        }
                        let dir = match mov_input {
                            TileMove::Right | TileMove::Left => *mov_input,
                            _ => murphy.mov2,
                        };
                        let upd = murphy.upd + 2.0;
                        let murphy = Tile::murphy(upd, *mov_input, dir, TileInteraction::Pushing);
                        grid.set(&coord3, tile2.moving(upd, *mov_input));
                        grid.set(&coord2, murphy);
                        grid.set(coord, Tile::transitory(upd, TileType::Empty));
                        return coord2;
                    }
                }
            }
            TileState::Tunnelable => {
                let coord3 = coord2.offset(mov_input);
                let tile3 = grid.get(&coord3);
                if tile3.typ == TileType::Empty
                    && matches!(
                        (tile2.typ, mov_input),
                        (TileType::PortsAll, _)
                            | (TileType::PortsAllBlue, _)
                            | (TileType::PortsDown, TileMove::Down)
                            | (TileType::PortsDownBlue, TileMove::Down)
                            | (TileType::PortsHorizontal, TileMove::Right)
                            | (TileType::PortsHorizontal, TileMove::Left)
                            | (TileType::PortsHorizontalBlue, TileMove::Right)
                            | (TileType::PortsHorizontalBlue, TileMove::Left)
                            | (TileType::PortsLeft, TileMove::Left)
                            | (TileType::PortsLeftBlue, TileMove::Left)
                            | (TileType::PortsRight, TileMove::Right)
                            | (TileType::PortsRightBlue, TileMove::Right)
                            | (TileType::PortsUp, TileMove::Up)
                            | (TileType::PortsUpBlue, TileMove::Up)
                            | (TileType::PortsVertical, TileMove::Up)
                            | (TileType::PortsVertical, TileMove::Down)
                            | (TileType::PortsVerticalBlue, TileMove::Up)
                            | (TileType::PortsVerticalBlue, TileMove::Down)
                    )
                {
                    let dir = match mov_input {
                        TileMove::Right | TileMove::Left => *mov_input,
                        _ => murphy.mov2,
                    };
                    let upd = murphy.upd.move_update();
                    let murphy =
                        Tile::murphy(upd, *mov_input, dir, TileInteraction::Tunneling(tile2.typ));
                    grid.set(&coord3, murphy);
                    grid.set(coord, Tile::transitory(upd, TileType::Empty));
                    return coord3;
                }
            }
            _ => {}
        }
    }

    if let Some(murphy) = grid.get_mut(coord) {
        murphy.upd.dont_update();
        murphy.int = TileInteraction::None;
    }
    *coord
}

pub fn set_cam_pos(murphy: &Tile, coord: &Coord, level_data: &mut LevelData) {
    let pos = if murphy.int == TileInteraction::None || murphy.int == TileInteraction::Slurping {
        Vec2::new(coord.x as f32, coord.y as f32)
    } else {
        let fcoord = coord.to_fcoord(murphy);
        Vec2::new(fcoord.x, fcoord.y)
    };

    level_data.camera_target = Some(pos * 16.0 + 8.0);
}

pub fn draw_murphy(murphy: &Tile, fcoord: &FCoord, texture: Texture2D) {
    let flip_x = matches!(&murphy.mov2, TileMove::Left);

    match murphy.int {
        TileInteraction::Eating(tile) => {
            let smooth_dst = &fcoord.offset_time(murphy);
            let anim_pos = 13 - (murphy.upd.max(0.0) * 4.0f32).floor() as i16;
            simple_draw(texture, fcoord, &tile.def_tex_pos(), false);
            simple_draw(texture, smooth_dst, &Coord::new(anim_pos, 7), flip_x);
        }
        TileInteraction::Pushing => {
            let smooth_dst = &fcoord.offset_time(murphy);
            simple_draw(texture, smooth_dst, &Coord::new(3, 7), flip_x);
        }
        TileInteraction::Slurping => {
            let pos = match &murphy.mov {
                TileMove::Up => Coord::new(4, 7),
                TileMove::Down => Coord::new(5, 7),
                _ => Coord::new(6, 7),
            };
            simple_draw(texture, fcoord, &pos, flip_x);
        }
        TileInteraction::Tunneling(port) => {
            let smooth_dst = &fcoord.offset_time(murphy);
            let dst2 = &fcoord.offset(&murphy.mov.opposite());
            let smooth_dst3 = &dst2.offset_time(murphy);
            let anim_pos = 13 - (murphy.upd.max(0.0) * 4.0f32).floor() as i16;
            simple_draw(texture, smooth_dst, &Coord::new(anim_pos, 7), flip_x);
            simple_draw(texture, smooth_dst3, &Coord::new(anim_pos, 7), flip_x);
            simple_draw(texture, dst2, &port.def_tex_pos(), false);
        }
        _ => simple_draw(texture, fcoord, &Coord::new(0, 7), false),
    }
}
