use macroquad::{prelude::*, texture::Texture2D, time::get_frame_time};

use crate::grid::Grid;
use crate::level::components::move_component::MoveComponent;
use crate::tile::Tile;

pub struct World {
    grid: Grid,
    accumulator: f32,
    preffered_axis: Axis,
    pub ups: u8,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: Grid::new((8, 8), Tile::Base), //vec![vec![Entity, 8], 8],
            accumulator: 0.0,
            preffered_axis: Axis::Horizontal,
            ups: 64,
        }
    }

    // test code
    pub fn new_from(cols: usize, grid: Vec<u8>) -> Self {
        let mut new_grid: Vec<Tile> = vec![Tile::Base; grid.len()];
        grid.iter().enumerate().for_each(|(i, value)| {
            new_grid[i] = Tile::from_u8(*value);
        });

        //println!("{:#?}", new_grid);

        Self {
            grid: Grid::new_from(cols, new_grid),
            accumulator: 0.0,
            preffered_axis: Axis::Horizontal,
            ups: 64,
        }
    }

    pub fn update(&mut self) {
        // input handling

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Right) {
            self.preffered_axis = Axis::Horizontal
        } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Down) {
            self.preffered_axis = Axis::Vertical
        };

        let x_axis = is_key_down(KeyCode::Right) as i8 - is_key_down(KeyCode::Left) as i8;
        let y_axis = is_key_down(KeyCode::Down) as i8 - is_key_down(KeyCode::Up) as i8;

        let movement_vec = if matches!(self.preffered_axis, Axis::Horizontal) {
            match (x_axis, y_axis) {
                (1, 0) => vec![MoveComponent::Right],
                (-1, 0) => vec![MoveComponent::Left],
                (1, 1) => vec![MoveComponent::Right, MoveComponent::Down],
                (-1, 1) => vec![MoveComponent::Left, MoveComponent::Down],
                (-1, -1) => vec![MoveComponent::Left, MoveComponent::Up],
                (1, -1) => vec![MoveComponent::Right, MoveComponent::Up],
                _ => vec![],
            }
        } else {
            match (x_axis, y_axis) {
                (0, 1) => vec![MoveComponent::Down],
                (0, -1) => vec![MoveComponent::Up],
                (1, 1) => vec![MoveComponent::Down, MoveComponent::Right],
                (-1, 1) => vec![MoveComponent::Down, MoveComponent::Left],
                (-1, -1) => vec![MoveComponent::Up, MoveComponent::Left],
                (1, -1) => vec![MoveComponent::Up, MoveComponent::Right],
                _ => vec![],
            }
        };

        /*let movement_tuple = if matches!(self.preffered_axis, Axis::Horizontal) {
            if y_axis != 0 {
                vec![MoveComponent::from_i8(x_axis), MoveComponent::from_i8(y_axis << 1)]
            } else {
                vec![MoveComponent::from_i8(x_axis)]
            }
        } else if x_axis != 0 {
            vec![MoveComponent::from_i8(y_axis << 1), MoveComponent::from_i8(x_axis)]
        } else {
            vec![MoveComponent::from_i8(y_axis << 1)]
        };*/

        let special_key = is_key_down(KeyCode::Space);

        // we want to update independent of the game loop so that
        // game logic stays correct, even if that means sacrificing time accuracy,
        // it doesn't matter if we skip a update as a result of it being just short of UPDATE_TIME,
        // because we can just interpolate it with delta time anyway. Or else it will be to fast to see i guess.
        let delta_time = get_frame_time();
        //println!("delta time: {}", delta_time);

        let fixed_delta_time = if (delta_time - (1.0 / 120.0)).abs() < 0.001 {
            //succeeder += 1;
            1.0 / 120.0
        } else if (delta_time - (1.0 / 60.0)).abs() < 0.001 {
            //succeeder += 1;
            1.0 / 60.0
        } else if (delta_time - (1.0 / 30.0)).abs() < 0.001 {
            //succeeder += 1;
            1.0 / 30.0
        } else {
            //failer += 1;
            delta_time
        };

        self.accumulator += fixed_delta_time;
        while self.accumulator >= 1.0 / self.ups as f32 {
            self.grid.update(&movement_vec, special_key);
            self.accumulator -= 1.0 / self.ups as f32;
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        self.grid.draw(texture);
    }
}

#[derive(Clone, Copy)]
enum Axis {
    Horizontal,
    Vertical,
}
