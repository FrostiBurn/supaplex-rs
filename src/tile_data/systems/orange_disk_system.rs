use crate::{
    grid::{Coord, Grid},
    level::LevelData,
    tile_data::{
        tile::Tile, tile_move::TileMove, tile_state::TileState, tile_type::TileType,
        tile_update::Updateable,
    },
};

use super::explode_system::set_area_explode;

pub fn orange_disk_system(coord: &Coord, grid: &mut Grid, level_data: &mut LevelData) {
    let disk = grid.get(coord);
    if disk.upd > 0.0 {
        return;
    }

    let coord_down = coord.offset(&TileMove::Down);

    if grid.get(&coord_down).typ == TileType::Empty {
        let upd = disk.upd.move_update();
        let tile = Tile::mov_state(disk, upd, TileMove::Down, TileState::Dangerous);
        grid.set(&coord_down, tile);
        grid.set(coord, Tile::transitory(upd, TileType::Empty));
    } else if disk.mov == TileMove::Down && grid.get(&coord_down).mov == TileMove::None {
        set_area_explode(coord, grid, &mut level_data.game_state);
    } else {
        grid.get_mut_unchecked(coord).upd.dont_update();
    }
}
