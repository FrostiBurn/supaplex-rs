use macroquad::{
    prelude::{Rect, Vec2, WHITE},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
    time::get_frame_time,
};

use crate::set_entity;
use crate::{
    entity::Entity,
    murphy::Murphy,
    tiles::{Moveable, Tile},
    UPDATE_TIME,
};

pub struct World {
    grid: Vec<Vec<Entity>>,
    pub murphy: Murphy,
    updated_entities: Vec<(usize, usize, Entity)>,
    time_since_last_update: f32,
    direction: (bool, i8, i8),
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(), //vec![vec![Entity, 8], 8],
            updated_entities: Vec::new(),
            murphy: Murphy::new(0, 0),
            time_since_last_update: 0.0,
            direction: (true, 0, 0),
        }
    }

    // test code
    pub fn new_from(grid: Vec<Vec<u8>>) -> Self {
        let mut new_grid: Vec<Vec<Entity>> = Vec::new();
        let mut murphy_pos = (0, 0);
        grid.iter().enumerate().for_each(|(x, col)| {
            let mut new_col: Vec<Entity> = Vec::new();
            col.iter().enumerate().for_each(|(y, i)| {
                if matches!(Tile::from_u8(*i), Tile::Murphy) {
                    murphy_pos = (x, y);
                } 
                new_col.push(Entity::from_tile(Tile::from_u8(*i)));
            });
            new_grid.push(new_col);
        });

        //println!("{:#?}", new_grid);

        Self {
            grid: new_grid,
            updated_entities: Vec::new(),
            murphy: Murphy::new(murphy_pos.0, murphy_pos.1),
            time_since_last_update: 0.0,
            direction: (true, 0, 0),
        }
    }

    pub fn update(&mut self) {
        // update murphy before anything else
        self.murphy.update(&self.grid, &mut self.updated_entities);

        self.time_since_last_update += get_frame_time();
        if self.time_since_last_update >= UPDATE_TIME {
            // do logic and push to updated_entities
            self.grid.iter().enumerate().for_each(|(x, col)| {
                col.iter().enumerate().for_each(|(y, entity)| {
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
        self.murphy.draw(texture);

        self.grid.iter().enumerate().for_each(|(x, col)| {
            col.iter().enumerate().for_each(|(y, entity)| {
                if !matches!(entity.tile, Tile::Empty) && !matches!(entity.tile, Tile::Murphy) {
                    match entity.moveable {
                        Moveable::Stationary => entity.tile.draw(texture, x as f32, y as f32),
                        Moveable::Up => entity.tile.draw(
                            texture,
                            x as f32,
                            y as f32 + 1.0 - self.time_since_last_update * UPDATE_TIME,
                        ),
                        Moveable::Down => entity.tile.draw(
                            texture,
                            x as f32,
                            y as f32 - 1.0 + self.time_since_last_update * 2.0,
                        ),
                        Moveable::Left => entity.tile.draw(
                            texture,
                            x as f32 + 1.0 - self.time_since_last_update * 2.0,
                            y as f32,
                        ),
                        Moveable::Right => entity.tile.draw(
                            texture,
                            x as f32 - 1.0 + self.time_since_last_update * 2.0,
                            y as f32,
                        ),
                    }
                }
            });
        });
    }
}
