use macroquad::texture::Texture2D;

use crate::{
    grid::Cells,
    level::components::{move_component::MoveComponent, update_component::UpdateComponent},
    tile::{simple_draw, Tile},
    FTupleExt, TupleExt,
};

use super::murphy_system::advanced_draw;

pub fn ai_system(
    src: (usize, usize),
    grid: &Cells,
    uc: &UpdateComponent,
    mc: &MoveComponent,
    mc_pre: &MoveComponent,
) -> Option<Vec<((usize, usize), Tile)>> {
    if uc.0 != 0 || uc.1 != 0 {
        return None;
    }

    let mc_left = mc.rotate_left();
    let src2 = src.trans(mc_left.as_tuple());

    return match grid.get_tuple(src2) {
        Tile::Empty => {
            let src_front = src.trans(mc.as_tuple());
            if !matches!(grid.get_tuple(src2.trans(mc.as_tuple())), Tile::Empty)
                && matches!(grid.get_tuple(src_front), Tile::Empty)
            {
                let ai_tile = Tile::SnikSnak(UpdateComponent(16, 0), *mc, *mc);
                return Some(vec![(src_front, ai_tile), (src, Tile::transitory_empty())]);
            } else {
                let ai_tile = Tile::SnikSnak(UpdateComponent(0, 8), mc_left, *mc);
                return Some(vec![(src, ai_tile)]);
            }
        }
        Tile::Murphy(_, _, _, _) => None, // BOOM!!!
        _ => {
            let src_front = src.trans(mc.as_tuple());
            match grid.get_tuple(src_front) {
                Tile::Empty => {
                    let ai_tile = Tile::SnikSnak(UpdateComponent(16, 0), *mc, *mc);
                    return Some(vec![(src_front, ai_tile), (src, Tile::transitory_empty())]);
                }
                _ => {
                    let mc3 = mc.rotate_right();
                    let ai_tile = Tile::SnikSnak(UpdateComponent(0, 8), mc3, *mc);
                    return Some(vec![(src, ai_tile)]);
                }
            };
        }
    };
}

pub fn draw_ai(
    dst: (f32, f32),
    texture: Texture2D,
    uc: &UpdateComponent,
    mc: &MoveComponent,
    mc_pre: &MoveComponent,
) {
    if uc.1 != 0 {
        let x = match (mc, mc_pre) {
            (MoveComponent::Up, MoveComponent::Left) => 0,
            (MoveComponent::Up, MoveComponent::Right) => 3,
            (MoveComponent::Down, MoveComponent::Left) => 1,
            (MoveComponent::Down, MoveComponent::Right) => 2,
            (MoveComponent::Left, MoveComponent::Up) => 0,
            (MoveComponent::Left, MoveComponent::Down) => 1,
            (MoveComponent::Right, MoveComponent::Up) => 3,
            (MoveComponent::Right, MoveComponent::Down) => 2,
            (_, _) => 13,
        };

        //println!("rotating!!!!!!!!!!!!!!!! {}", x);

        simple_draw(texture, dst, (x, 12));
    } else {
        let x = match uc.0 {
            0 | 1 | 14 | 15 | 16 => 0.0,
            2 | 3 | 12 | 13 => 1.0,
            4 | 5 | 10 | 11 => 2.0,
            _ => 3.0,
        };

        match mc {
            MoveComponent::Up => advanced_draw(
                texture,
                dst.get_offset(uc.0, mc),
                (x + 4.0, 12.0),
                false,
                false,
            ),
            MoveComponent::Down => advanced_draw(
                texture,
                dst.get_offset(uc.0, mc),
                (x + 4.0, 12.0),
                false,
                true,
            ),
            MoveComponent::Left => advanced_draw(
                texture,
                dst.get_offset(uc.0, mc),
                (x + 8.0, 12.0),
                true,
                false,
            ),
            MoveComponent::Right => advanced_draw(
                texture,
                dst.get_offset(uc.0, mc),
                (x + 8.0, 12.0),
                false,
                false,
            ),
            _ => simple_draw(texture, dst, (4, 12)),
        };
    }
}
