use macroquad::prelude::*;

use crate::{entity::Entity, tiles::{Tile, Moveable}, get_entity};

#[derive(Clone, Copy)]
pub struct Murphy;

impl Murphy {
    pub fn update(
        &self,
        x: usize,
        y: usize,
        grid: &Vec<Vec<Entity>>,
        updates: &mut Vec<(usize, usize, Entity)>,
        direction: &(bool, i8, i8),
    ) {
        let entity = get_entity!(grid, x, y);
        //println!("entity: [{}, {}] {:?}", x, y, entity.tile);

        if direction.1 != 0 && direction.2 == 0 {
            // horizontal
            
            // check if you can go there
            // todo
            let new_x = (x as i8 + direction.1) as usize;
            let moving = match direction.1 {
                -1 => Moveable::Left,
                1 => Moveable::Right,
                _ => Moveable::Stationary
            };

            updates.push((x, y, Entity::from_tile(Tile::Empty)));
            updates.push((new_x, y, Entity::from(entity, moving)));
        } else {
            // vertical
            let moving = match direction.2 {
                -1 => Moveable::Up,
                1 => Moveable::Down,
                _ => Moveable::Stationary
            };

            updates.push((x, y, Entity::from_tile(Tile::Empty)));
            updates.push((x, (y as i8 + direction.2) as usize, Entity::from(entity, moving)));
        }
        if direction.1 != 0 && direction.2 != 0 {
            if direction.0 {
                // horizontal

            } else {
                // vertical

            }
        }

    }
}
