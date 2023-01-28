use crate::{
    grid::Cells,
    level::components::{move_component::MoveComponent, update_component::UpdateComponent},
    tile::Tile,
    TupleExt,
};

pub fn gravity_system(
    src: (usize, usize),
    grid: &Cells,
    update_component: &UpdateComponent,
    move_component: &MoveComponent,
) -> Option<Vec<((usize, usize), Tile)>> {
    if update_component.0 != 0 {
        return None;
    }

    match grid.get_trans(src, (0, 1)) {
        Tile::Empty => {
            if let Some(tile) = grid
                .get_tuple(src)
                .change_tile(UpdateComponent(16, 0), MoveComponent::Down)
            {
                return Some(vec![
                    (src.trans((0, 1)), tile),
                    (src, Tile::transitory_empty()),
                ]);
            }
        }
        Tile::Infotron(_, _) | Tile::RAMChips(_) | Tile::Zonk(_, _) => {
            if matches!(grid.get_trans(src, (-1, 0)), &Tile::Empty)
                && matches!(grid.get_trans(src, (-1, 1)), &Tile::Empty)
            {
                if let Some(tile) = grid
                    .get_tuple(src)
                    .change_tile(UpdateComponent(16, 0), MoveComponent::Left)
                {
                    return Some(vec![
                        (src.trans((-1, 0)), tile),
                        (src, Tile::transitory_empty()),
                    ]);
                }
            } else if matches!(grid.get_trans(src, (1, 0)), &Tile::Empty)
                && matches!(grid.get_trans(src, (1, 1)), &Tile::Empty)
            {
                if let Some(tile) = grid
                    .get_tuple(src)
                    .change_tile(UpdateComponent(16, 0), MoveComponent::Right)
                {
                    return Some(vec![
                        (src.trans((1, 0)), tile),
                        (src, Tile::transitory_empty()),
                    ]);
                }
            } else if !matches!(move_component, MoveComponent::Stationary) {
                if let Some(tile) = grid
                    .get_tuple(src)
                    .change_tile(UpdateComponent(0, 0), MoveComponent::Stationary)
                {
                    return Some(vec![(src, tile)]);
                }
            }
        }
        _ => {
            if !matches!(move_component, MoveComponent::Stationary) {
                if let Some(tile) = grid
                    .get_tuple(src)
                    .change_tile(UpdateComponent(0, 0), MoveComponent::Stationary)
                {
                    return Some(vec![(src, tile)]);
                }
            }
        }
    }
    None
}
