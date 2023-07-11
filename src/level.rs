use std::time::{Duration, Instant};

use macroquad::{
    prelude::{is_key_pressed, is_key_released, KeyCode, Vec2},
    text::{draw_text_ex, TextParams, measure_text},
    texture::Texture2D,
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    grid::{Coord, FCoord, Grid},
    tile_data::{
        systems::{
            ai_system::ai_system,
            explode_system::explode_system,
            gravity_system::gravity_system,
            murphy_system::{murphy_system, set_cam_pos},
            orange_disk_system::orange_disk_system,
            transitory_system::transitory_system,
        },
        tile_move::TileMove,
        tile_type::{draw_time, TileType},
        tile_update::Updateable,
    },
};

#[derive(Clone)]
pub struct Level {
    pub data: LevelData,
    pub grid: Grid,
}

impl Level {
    pub fn new(speed: f32, width: i16, height: i16, array: Vec<u8>) -> Self {
        let grid = Grid::new(width, height, array);

        Self {
            data: LevelData {
                move_priority: Vec::new(),
                name: "N/A".to_string(),
                speed,
                delta_time: 0.0,
                update_duration: Duration::ZERO,
                camera_target: None,
                game_state: GameState::Active,
                infotrons_required: 51,
                red_utility_disks: 7,
                gravity_enabled: false,
                zonks_frozen: false,
            },
            grid,
        }
    }

    pub fn update(&mut self) {
        self.data.delta_time = get_frame_time() * self.data.speed;

        if is_key_released(KeyCode::Up) {
            self.data
                .move_priority
                .retain(|mc| !matches!(mc, TileMove::Up));
        }
        if is_key_released(KeyCode::Down) {
            self.data
                .move_priority
                .retain(|mc| !matches!(mc, TileMove::Down));
        }
        if is_key_released(KeyCode::Left) {
            self.data
                .move_priority
                .retain(|mc| !matches!(mc, TileMove::Left));
        }
        if is_key_released(KeyCode::Right) {
            self.data
                .move_priority
                .retain(|mc| !matches!(mc, TileMove::Right));
        }

        if is_key_pressed(KeyCode::Up) {
            self.data.move_priority.insert(0, TileMove::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            self.data.move_priority.insert(0, TileMove::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            self.data.move_priority.insert(0, TileMove::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            self.data.move_priority.insert(0, TileMove::Right);
        }

        let now = Instant::now();
        self.grid.array.iter_mut().for_each(|tile| {
            tile.update_time(self.data.delta_time);
        });
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let coord = Coord::new(x, y);
                match self.grid.get(&coord).typ {
                    TileType::Bug => {}
                    TileType::Electron => {}
                    TileType::Infotron => gravity_system(&coord, &mut self.grid, &mut self.data),
                    TileType::Murphy => {
                        let coord = murphy_system(&coord, &mut self.grid, &mut self.data);
                        let murphy = self.grid.get(&coord);
                        set_cam_pos(murphy, &coord, &mut self.data);
                    }
                    TileType::OrangeUtilityDisk => {
                        orange_disk_system(&coord, &mut self.grid, &mut self.data)
                    }
                    TileType::RedUtilityDisk => {}
                    TileType::Terminal => {}
                    TileType::Transitory => {
                        transitory_system(&coord, &mut self.grid, &mut self.data)
                    }
                    TileType::YellowUtilityDisk => {}
                    TileType::SnikSnak => ai_system(&coord, &mut self.grid, &mut self.data),
                    TileType::Zonk => gravity_system(&coord, &mut self.grid, &mut self.data),
                    TileType::Explosion | TileType::Explosion2 => {
                        explode_system(&coord, &mut self.grid, &mut self.data);
                    }
                    _ => {}
                }
            }
        }

        if self.data.game_state == GameState::Died {
            //self.data.speed = 0.0;
        }

        // You might ask: why tf 2 loops???
        // because update_time for every tile needs to happen before a tile moves to a different position, iterating is very negliable at 1440 tiles const.
        self.data.update_duration = now.elapsed();
    }

    pub fn draw(&self, texture: Texture2D) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let coord = Coord::new(x, y);
                self.grid.get(&coord).draw(&coord.as_fcoord(), texture);
            }
        }
    }

    pub fn draw_text(&self) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                let coord = Coord::new(x, y);
                let tile = self.grid.get(&coord);
                draw_time(
                    &FCoord::new(coord.x as f32 * 16.0, coord.y as f32 * 16.0),
                    tile.upd,
                );
            }
        }
    }

    pub fn draw_info(&self, text_params: TextParams) {
        draw_text_ex(
            &format!("Infotrons required: {}", self.data.infotrons_required),
            10.0,
            20.0,
            text_params,
        );
        draw_text_ex(
            &format!("Red Utility Disks: {}", self.data.red_utility_disks),
            10.0,
            40.0,
            text_params,
        );

        if self.data.game_state == GameState::Finished {
            let mut text_params = text_params;
            text_params.font_scale = 1.0;
            text_params.font_size = 64;

            let text1 = "YOU FINISHED THE GAME!!!";
            let text1_size = measure_text(text1, Some(text_params.font), text_params.font_size, text_params.font_scale);
            let text2 = "A 10 IS NOW A REQUIREMENT";
            let text2_size = measure_text(text2, Some(text_params.font), text_params.font_size, text_params.font_scale);

            draw_text_ex(
                text1,
                screen_width() / 2.0 - text1_size.width / 2.0,
                screen_height() / 2.0 - text2_size.height / 2.0,
                text_params,
            );
            draw_text_ex(
                text2,
                screen_width() / 2.0 - text2_size.width / 2.0,
                screen_height() / 2.0 + 60.0 - text2_size.height / 2.0,
                text_params,
            );
        }
    }
}

#[derive(Clone)]
pub struct LevelData {
    pub move_priority: Vec<TileMove>,
    pub name: String,
    pub speed: f32,
    pub delta_time: f32,
    pub update_duration: Duration,
    pub camera_target: Option<Vec2>,
    pub game_state: GameState,
    pub infotrons_required: i32,
    pub red_utility_disks: u32,
    pub gravity_enabled: bool,
    pub zonks_frozen: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Died,
    Finished,
}
