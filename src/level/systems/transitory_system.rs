use macroquad::texture::Texture2D;

use crate::{
    level::components::{
        transitory_component::TransitoryComponent, update_component::UpdateComponent,
    },
    tile::{simple_draw, Tile},
};

pub fn transitory_system(
    src: (usize, usize),
    update_component: &UpdateComponent,
) -> Option<Vec<((usize, usize), Tile)>> {
    if update_component.0 == 0 {
        return Some(vec![(src, Tile::Empty)]);
    }
    None
}

pub fn draw_transitory(
    src: (f32, f32),
    texture: Texture2D,
    update_component: &UpdateComponent,
    transitory_component: &TransitoryComponent,
) {
    let y = match transitory_component {
        TransitoryComponent::None => {
            //simple_draw(texture, src, (0, 20));
            return;
        }
        TransitoryComponent::Base => 0,
        TransitoryComponent::Infotron => 6,
        TransitoryComponent::RedUtilityDisk => 14,
    };

    let x = match update_component.0 {
        16 | 15 => 1,
        14 | 13 => 2,
        12 | 11 => 3,
        10 | 9 => 4,
        8 | 7 => 5,
        6 | 5 => 6,
        4 | 3 => 7,
        2 | 1 => return,
        _ => return,
    };

    simple_draw(texture, src, (x, y));
}
