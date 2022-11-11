use macroquad::prelude::{*, animation::AnimatedSprite};

use crate::{
    entity::Entity,
    get_entity,
    tiles::{Moveable, Tile},
    UPDATE_TIME,
};

#[derive(Clone, Copy)]
pub struct Murphy {
    pub position: (usize, usize),
    pub prev_position: (usize, usize),
    eating_tile: Tile,
    preffered_axis: Axis,
    animation_direction: AnimationDirection,
    //moving: Moveable,
    time_since_last_update: f32,
}

impl Murphy {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            position: (x, y),
            prev_position: (x, y),
            eating_tile: Tile::Empty,
            preffered_axis: Axis::Horizontal,
            animation_direction: AnimationDirection::Right,
            //moving: Moveable::Stationary,
            time_since_last_update: 0.0,
        }
    }

    pub fn update(&mut self, grid: &Vec<Vec<Entity>>, updates: &mut Vec<(usize, usize, Entity)>) {
        // check input
        let x_axis = is_key_down(KeyCode::Right) as i8 - is_key_down(KeyCode::Left) as i8;
        let y_axis = is_key_down(KeyCode::Down) as i8 - is_key_down(KeyCode::Up) as i8;

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Up) {
            self.preffered_axis = Axis::Vertical;
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Left) {
            self.preffered_axis = Axis::Horizontal;
        }

        if self.time_since_last_update <= 0.0 {
            self.prev_position = self.position;
            if x_axis != 0 || y_axis != 0 {
                self.time_since_last_update = UPDATE_TIME;

                println!("x, y: {}, {}", x_axis, y_axis);

                if matches!(self.preffered_axis, Axis::Horizontal) {
                    if x_axis != 0
                        && get_entity!(
                            grid,
                            (self.position.0 as i8 + x_axis) as usize,
                            self.position.1
                        )
                        .is_eatable
                    {
                        self.animation_direction = if x_axis == 1 {
                            AnimationDirection::Right
                        } else {
                            AnimationDirection::Left
                        };
                        self.position =
                            ((self.position.0 as i8 + x_axis) as usize, self.position.1);
                        self.eating_tile = get_entity!(grid, self.position.0, self.position.1).tile;
                    } else if y_axis != 0
                        && get_entity!(
                            grid,
                            self.position.0,
                            (self.position.1 as i8 + y_axis) as usize
                        )
                        .is_eatable
                    {
                        self.position =
                            (self.position.0, (self.position.1 as i8 + y_axis) as usize);
                        self.eating_tile = get_entity!(grid, self.position.0, self.position.1).tile;
                    }
                } else {
                    if y_axis != 0
                        && get_entity!(
                            grid,
                            self.position.0,
                            (self.position.1 as i8 + y_axis) as usize
                        )
                        .is_eatable
                    {
                        self.position =
                            (self.position.0, (self.position.1 as i8 + y_axis) as usize);
                        self.eating_tile = get_entity!(grid, self.position.0, self.position.1).tile;
                    } else if x_axis != 0
                        && get_entity!(
                            grid,
                            (self.position.0 as i8 + x_axis) as usize,
                            self.position.1
                        )
                        .is_eatable
                    {
                        self.animation_direction = if x_axis == 1 {
                            AnimationDirection::Right
                        } else {
                            AnimationDirection::Left
                        };
                        self.position =
                            ((self.position.0 as i8 + x_axis) as usize, self.position.1);
                        self.eating_tile = get_entity!(grid, self.position.0, self.position.1).tile;
                    }
                }
                if self.position != self.prev_position {
                    updates.push((
                        self.position.0,
                        self.position.1,
                        Entity::from_tile(Tile::Murphy),
                    ));
                    updates.push((
                        self.prev_position.0,
                        self.prev_position.1,
                        Entity::from_tile(Tile::Empty),
                    ));
                }
            }
        } else {
            self.time_since_last_update -= get_frame_time();
            // todo: fix small stutter
            // doing this makes sure it doesn't overshoot the tile but also makes an ever so slight stutter at the end of the movement
            if self.time_since_last_update < 0.0 {
                self.time_since_last_update = 0.0;
            }
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        // todo: draw the tile being eaten
        // todo: add correct animation

        if self.position == self.prev_position {
            simple_draw(
                texture,
                self.position.0 as f32,
                self.position.1 as f32,
                0.0,
                7.0,
                false,
            );
        } else {
            let delta = self.time_since_last_update / UPDATE_TIME;
            let new_x = self.position.0 as f32
                - ((self.position.0 as f32 - self.prev_position.0 as f32) * delta);
            let new_y = self.position.1 as f32
                - ((self.position.1 as f32 - self.prev_position.1 as f32) * delta);

            self.eating_tile
                .draw(texture, self.position.0 as f32, self.position.1 as f32);

            match self.animation_direction {
                AnimationDirection::Left => simple_draw(texture, new_x, new_y, 8.0, 7.0, true),
                AnimationDirection::Right => simple_draw(texture, new_x, new_y, 8.0, 7.0, false),
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
enum AnimationDirection {
    Left,
    Right,
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
                x: x_pos * 17.0,
                y: y_pos * 17.0,
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
