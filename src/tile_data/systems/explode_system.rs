use macroquad::texture::Texture2D;

use crate::{
    grid::{Coord, FCoord, Grid},
    level::{LevelData, GameState},
    tile_data::{
        tile::Tile,
        tile_interaction::TileInteraction,
        tile_move::TileMove,
        tile_state::TileState,
        tile_type::{simple_draw, TileType}, tile_update::Updateable,
    },
};

pub fn explode_system(coord: &Coord, grid: &mut Grid, level_data: &mut LevelData) {
    let explosion = grid.get(coord);

    if explosion.upd > 0.0 {
        return;
    }

    match explosion.typ {
        TileType::Explosion => {
            grid.set(coord, TileType::to_tile(TileType::Empty));
        }
        TileType::Explosion2 => {
            set_area_explode(coord, grid, &mut level_data.game_state);
        }
        _ => {}
    }
}

pub fn draw_explode_system(explosion: &Tile, fcoord: &FCoord, texture: Texture2D) {
    let interval = 7.0 / 2.5;
    let time = (interval * explosion.upd).floor() as i16;
    simple_draw(texture, fcoord, &Coord::new(time, 18), false);
}

pub fn set_area_explode(coord: &Coord, grid: &mut Grid, game_state: &mut GameState) {
    let tile = grid.get(coord);
    let upd = tile.upd + 2.5;

    for y in -1..=1 {
        for x in -1..=1 {
            let curr_coord = coord.offset(&(x, y));
            if x == 0 && y == 0 {
                let explosion1 = Tile {
                    typ: TileType::Explosion,
                    state: TileState::Dangerous,
                    mov: TileMove::None,
                    mov2: TileMove::None,
                    int: TileInteraction::None,
                    upd,
                };
                grid.set(&curr_coord, explosion1);
                continue;
            }

            match grid.get(&curr_coord).typ {
                TileType::Electron
                | TileType::Murphy
                | TileType::OrangeUtilityDisk
                | TileType::SnikSnak
                | TileType::YellowUtilityDisk => {
                    if grid.get(&curr_coord).typ == TileType::Murphy {
                        *game_state = GameState::Died;
                    }
                    let explosion2 = Tile {
                        typ: TileType::Explosion2,
                        state: TileState::Dangerous,
                        mov: TileMove::None,
                        mov2: TileMove::None,
                        int: TileInteraction::None,
                        upd,
                    };
                    grid.set(&curr_coord, explosion2);
                }
                TileType::Explosion2 => {}
                _ => {
                    if grid.get(&curr_coord).state != TileState::Indestructible {
                        let explosion1 = Tile {
                            typ: TileType::Explosion,
                            state: TileState::Dangerous,
                            mov: TileMove::None,
                            mov2: TileMove::None,
                            int: TileInteraction::None,
                            upd,
                        };
                        grid.set(&curr_coord, explosion1);
                    }
                }
            }

            //grid.set(&curr_coord, )
        }
    }
}
