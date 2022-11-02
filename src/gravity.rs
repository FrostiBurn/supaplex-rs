use crate::{
    get_entity,
    tiles::{Moveable, Tile}, entity::Entity,
};

#[derive(Clone, Copy)]
pub struct Gravity {
    pub slideable: bool,
}

impl Gravity {
    pub fn update(
        &self,
        x: usize,
        y: usize,
        grid: &Vec<Vec<Entity>>,
        updates: &mut Vec<(usize, usize, Entity)>
    ) {
        let entity = get_entity!(grid, x, y);
        let foundation_entity = get_entity!(grid, x, y + 1);

        match foundation_entity.tile {
            Tile::Empty => {
                updates.push((x, y, Entity::from_tile(Tile::Empty)));
                updates.push((x, y + 1, Entity::from(entity, Moveable::Down)));
                //println!("[{:?}]: empty", entity.tile);
            }
            Tile::Infotron | Tile::RAMChips(_) | Tile::Zonk => {
                //println!("[{:?}]: slip", entity.tile);
                if get_entity!(grid, x - 1, y - 1).gravity.is_none()
                    && matches!(get_entity!(grid, x - 1, y).tile, Tile::Empty)
                    && matches!(get_entity!(grid, x - 1, y + 1).tile, Tile::Empty)
                {
                    updates.push((x, y, Entity::from_tile(Tile::Empty)));
                    updates.push((x - 1, y, Entity::from(entity, Moveable::Left)));
                }

                if get_entity!(grid, x + 1, y - 1).gravity.is_none()
                    && matches!(get_entity!(grid, x + 1, y).tile, Tile::Empty)
                    && matches!(get_entity!(grid, x + 1, y + 1).tile, Tile::Empty)
                {
                    updates.push((x, y, Entity::from_tile(Tile::Empty)));
                    updates.push((x + 1, y, Entity::from(entity, Moveable::Left)));
                }

                if !matches!(entity.moveable, Moveable::Stationary) {
                    updates.push((x, y, Entity::from(entity, Moveable::Stationary)));
                }
            }
            _ => {
                if !matches!(entity.moveable, Moveable::Stationary) {
                    updates.push((x, y, Entity::from(entity, Moveable::Stationary)));
                }
            }
        }
    }
}
