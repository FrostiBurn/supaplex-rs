use macroquad::{
    prelude::{is_key_down, is_key_pressed, KeyCode, Rect, Vec2, WHITE},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    time::{get_frame_time},
};

use crate::{
    entity::Entity,
    murphy::Murphy,
    tiles::{Moveable, Tile},
};
use crate::{get_entity, set_entity};

pub struct World {
    grid: Vec<Vec<Entity>>,
    murphy: Murphy,
    updated_entities: Vec<(usize, usize, Entity)>,
    time_since_last_update: f32,
    direction: (bool, i8, i8),
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(), //vec![vec![Entity, 8], 8],
            updated_entities: Vec::new(),
            murphy: Murphy,
            time_since_last_update: 0.0,
            direction: (true, 0, 0),
        }
    }

    // test code
    pub fn new_from(grid: Vec<Vec<u8>>) -> Self {
        let mut new_grid: Vec<Vec<Entity>> = Vec::new();
        grid.iter().for_each(|col| {
            let mut new_col: Vec<Entity> = Vec::new();
            col.iter().for_each(|i| {
                new_col.push(Entity::from_tile(Tile::from_u8(*i)));
            });
            new_grid.push(new_col);
        });

        //println!("{:#?}", new_grid);

        Self {
            grid: new_grid,
            updated_entities: Vec::new(),
            murphy: Murphy,
            time_since_last_update: 0.0,
            direction: (true, 0, 0),
        }
    }

    pub fn update(&mut self) {
        // handle murphy input handling
        match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, true) => self.direction.2 = 0,
            (true, false) => self.direction.2 = -1,
            (false, true) => self.direction.2 = 1,
            _ => {}
        }

        match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, true) => self.direction.1 = 0,
            (true, false) => self.direction.1 = -1,
            (false, true) => self.direction.1 = 1,
            _ => {}
        }

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Up) {
            self.direction.0 = false;
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Left) {
            self.direction.0 = true;
        }

        self.time_since_last_update += get_frame_time();
        if self.time_since_last_update >= 0.5 {
            // do logic and push to updated_entities
            self.grid.iter().enumerate().for_each(|(x, col)| {
                col.iter().enumerate().for_each(|(y, entity)| {
                    if let Some(murphy) = entity.murphy {
                    murphy.update(x, y, &self.grid, &mut self.updated_entities, &self.direction);
                    }
                    if let Some(gravity) = entity.gravity {
                        gravity.update(x, y, &self.grid, &mut self.updated_entities);
                    }
                    if let Some(mut bug_state) = entity.bug_state {
                        bug_state.update();
                    }
                });
            });

            // reset x, y direction
            self.direction = (self.direction.0, 0, 0);
            self.time_since_last_update = 0.0;
        }


        // push all grid updates at once
        self.updated_entities.iter().for_each(|(x, y, entity)| {
            set_entity!(self.grid, *x, *y, *entity);
        });
        self.updated_entities.clear();
    }

    pub fn draw(&self, texture: Texture2D) {
        self.grid.iter().enumerate().for_each(|(x, col)| {
            col.iter().enumerate().for_each(|(y, entity)| {
                if !matches!(entity.tile, Tile::Empty) {
                    match entity.moveable {
                        Moveable::Stationary => {
                            simple_draw(texture, x as f32, y as f32, entity.tile.as_u8(), 0)
                        }
                        Moveable::Up => simple_draw(
                            texture,
                            x as f32,
                            y as f32 + 1.0 - self.time_since_last_update * 2.0,
                            entity.tile.as_u8(),
                            0,
                        ),
                        Moveable::Down => simple_draw(
                            texture,
                            x as f32,
                            y as f32 - 1.0 + self.time_since_last_update * 2.0,
                            entity.tile.as_u8(),
                            0,
                        ),
                        Moveable::Left => simple_draw(
                            texture,
                            x as f32 + 1.0 - self.time_since_last_update * 2.0,
                            y as f32,
                            entity.tile.as_u8(),
                            0,
                        ),
                        Moveable::Right => simple_draw(
                            texture,
                            x as f32 - 1.0 + self.time_since_last_update * 2.0,
                            y as f32,
                            entity.tile.as_u8(),
                            0,
                        ),
                    }
                }
            });
        });
    }
}

fn simple_draw(texture: Texture2D, x: f32, y: f32, x_pos: u8, y_pos: u8) {
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 1.0, y: 1.0 }),
            source: Some(Rect {
                x: x_pos as f32 * 16.0,
                y: y_pos as f32 * 16.0,
                w: 16.0,
                h: 16.0,
            }),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}
