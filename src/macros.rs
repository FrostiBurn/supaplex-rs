#[macro_export]
macro_rules! get_entity {
    ($grid:expr, $x:expr, $y:expr) => {
        match $grid.get($x) {
            Some(col) => match col.get($y) {
                Some(entity) => *entity,
                None => Entity::from_tile(Tile::None),
            },
            None => Entity::from_tile(Tile::None),
        }
    };
}

#[macro_export]
macro_rules! set_entity {
    ($grid:expr, $x:expr, $y:expr, $entity:expr) => {
        match $grid.get_mut($x) {
            Some(col) => match col.get_mut($y) {
                Some(old_entity) => *old_entity = $entity,
                None => return,
            },
            None => return,
        }
    };
}