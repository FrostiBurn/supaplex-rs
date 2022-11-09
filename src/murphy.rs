use macroquad::prelude::*;

use crate::{
    entity::Entity,
    get_entity,
    tiles::{Moveable, Tile},
    UPDATE_TIME,
};

#[derive(Clone, Copy)]
pub struct Murphy {
    pub x: usize,
    pub y: usize,
    axis: Axis,
    moving: Moveable,
    time_since_last_update: f32,
}

impl Murphy {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            axis: Axis::Horizontal,
            moving: Moveable::Stationary,
            time_since_last_update: 0.0,
        }
    }

    pub fn update(&mut self, grid: &Vec<Vec<Entity>>, updates: &mut Vec<(usize, usize, Entity)>) {
        // check input
        let x_axis = is_key_down(KeyCode::Right) as i8 - is_key_down(KeyCode::Left) as i8;
        let y_axis = is_key_down(KeyCode::Down) as i8 - is_key_down(KeyCode::Up) as i8;

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Up) {
            self.axis = Axis::Vertical;
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Left) {
            self.axis = Axis::Horizontal;
        }

        if (x_axis != 0 || y_axis != 0) && self.time_since_last_update <= 0.0 {
            self.time_since_last_update = UPDATE_TIME;

            let entity = get_entity!(grid, self.x, self.y);

            match (x_axis, y_axis) {
                (1, 0) | (-1, 0) => {
                    // horizontal
                    updates.push((self.x, self.y, Entity::from_tile(Tile::Empty)));
                    self.x = (self.x as i8 + x_axis) as usize;
                    self.moving = if x_axis == 1 {
                        Moveable::Right
                    } else {
                        Moveable::Left
                    };
                    updates.push((self.x, self.y, Entity::from(entity, self.moving)));
                }
                (0, 1) | (0, -1) => {
                    // vertical
                    updates.push((self.x, self.y, Entity::from_tile(Tile::Empty)));
                    self.y = (self.y as i8 + y_axis) as usize;
                    self.moving = if y_axis == 1 {
                        Moveable::Down
                    } else {
                        Moveable::Up
                    };
                    updates.push((self.x, self.y, Entity::from(entity, self.moving)));
                }
                _ => {
                    // both
                }
            }
            if x_axis != 0 && y_axis == 0 {}
            if x_axis == 0 && y_axis != 0 {}
        } else {
            self.time_since_last_update -= get_frame_time();
            // todo: fix small stutter
            // doing this makes sure it doesn't overshoot the tile but also makes an ever so slight stutter at the end of the movement
            if self.time_since_last_update < 0.0 {
                self.time_since_last_update = 0.0;
            }
            // todo: get rid of unneccesary calls
            updates.push((self.x, self.y, Entity::from(Entity::from_tile(Tile::Murphy), Moveable::Stationary)));
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        // todo: draw the tile being eaten
        // todo: add correct animation
        match self.moving {
            Moveable::Stationary => simple_draw(texture, self.x as f32, self.y as f32, 0.0, 7.0, false),
            Moveable::Up => simple_draw(texture, self.x as f32, self.y as f32 + (self.time_since_last_update / UPDATE_TIME), 8.0, 7.0, false), // check last horizontal movement
            Moveable::Down => simple_draw(texture, self.x as f32, self.y as f32 - (self.time_since_last_update / UPDATE_TIME), 8.0, 7.0, true),
            Moveable::Left => simple_draw(texture, self.x as f32 + (self.time_since_last_update / UPDATE_TIME), self.y as f32, 8.0, 7.0, true),
            Moveable::Right => simple_draw(texture, self.x as f32 - (self.time_since_last_update / UPDATE_TIME), self.y as f32, 8.0, 7.0, false),
        }
    }
}

#[derive(Clone, Copy)]
enum Axis {
    Horizontal,
    Vertical,
}

fn simple_draw(texture: Texture2D, x: f32, y: f32, x_pos: f32, y_pos: f32, flip_x: bool) {
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 1.0, y: 1.0 }),
            source: Some(Rect {
                x: x_pos * 16.0,
                y: y_pos * 16.0,
                w: 16.0,
                h: 16.0,
            }),
            rotation: 0.0,
            flip_x: flip_x,
            flip_y: false,
            pivot: None,
        },
    );
}