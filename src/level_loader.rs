use std::{fs::{File, self}, io, time::Duration};

use crate::{level::{Level, LevelData}, grid::Grid};

pub fn load_level_dat(path: &str) -> io::Result<Vec<Level>> {
    let data = fs::read(path)?;

    let mut levels = Vec::new();
    
    for i in 0..111 {
        let level_offset = i * 1536;

        // tiles
        let tiles_array = &data[level_offset..(1440 + level_offset)];

        // gravity enabled
        let gravity = data[1444 + level_offset];

        // Name
        let name = &data[(1444 + level_offset)..(1467 + level_offset)];

        // zonks frozen
        let zonk_buf = data[1469 + level_offset];

        // infotron count
        let infotrons = data[1470 + level_offset];

        let tiles = Grid::new(60, 24, tiles_array.to_vec());
        let gravity_enabled = gravity == 0;
        let name = String::from_utf8(name.to_vec()).unwrap();
        let zonks_frozen = zonk_buf == 0;

        let level = Level {
            data: LevelData {
                move_priority: Vec::new(),
                name,
                speed: 8.0,
                delta_time: 0.0,
                update_duration: Duration::ZERO,
                camera_target: None,
                game_state: crate::level::GameState::Active,
                infotrons_required: infotrons as i32,
                red_utility_disks: 0,
                gravity_enabled,
                zonks_frozen,
            },
            grid: tiles,
        };
        levels.push(level);
    }

    Ok(levels)
}