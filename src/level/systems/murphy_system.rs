use macroquad::{
    prelude::{Rect, Vec2, WHITE},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
    grid::Cells,
    level::components::{
        direction_component::DirectionComponent,
        interaction_component::{InteractionComponent, InteractionTile},
        move_component::MoveComponent,
        transitory_component::TransitoryComponent,
        update_component::UpdateComponent,
    },
    tile::{simple_draw, Tile},
    FTupleExt, TupleExt,
};

pub fn murphy_system(
    src: (usize, usize),
    grid: &Cells,
    update_component: &UpdateComponent,
    move_component_vec: &[MoveComponent],
    direction_component: &DirectionComponent,
    special_key: bool,
) -> Option<Vec<((usize, usize), Tile)>> {
    if update_component.0 != 0 {
        return None;
    }

    let mut updates = Vec::new();

    for move_component in move_component_vec.iter() {
        let src2 = src.trans(move_component.as_tuple());
        let tile2 = grid.get_tuple(src2);

        match tile2 {
            Tile::Bug(uc)
            | Tile::Infotron(uc, _)
            | Tile::RedUtilityDisk(uc)
            | Tile::OrangeUtilityDisk(uc, _)
            | Tile::YellowUtilityDisk(uc, _)
            | Tile::Zonk(uc, _)
                if uc.0 != 0 =>
            {
                // should explode!!!
                break;
            }
            _ => {}
        };

        match tile2 {
            Tile::Base
            | Tile::Bug(_)
            | Tile::Infotron(_, _)
            | Tile::RedUtilityDisk(_)
            | Tile::Empty => {
                if special_key && !matches!(tile2, Tile::Empty) {
                    let murphy_tile = Tile::Murphy(
                        UpdateComponent(16, 0),
                        *move_component,
                        get_direction_component(move_component, direction_component),
                        InteractionComponent::Slurping,
                    );
                    updates.push((src, murphy_tile));
                    updates.push((
                        src2,
                        Tile::Transitory(
                            UpdateComponent(16, 0),
                            TransitoryComponent::from_tile(tile2),
                        ),
                    ));
                    break;
                } else {
                    let murphy_tile = Tile::Murphy(
                        UpdateComponent(16, 0),
                        *move_component,
                        get_direction_component(move_component, direction_component),
                        InteractionComponent::Eating(InteractionTile::from_tile(tile2)),
                    );
                    updates.push((src, Tile::transitory_empty()));
                    updates.push((src2, murphy_tile));
                    break;
                }
            }
            Tile::Zonk(_, _) | Tile::YellowUtilityDisk(_, _) | Tile::OrangeUtilityDisk(_, _) => {
                if matches!(tile2, Tile::Zonk(_, _))
                    && (matches!(move_component, MoveComponent::Up)
                        || matches!(move_component, MoveComponent::Down))
                {
                    break;
                }

                let src3 = src2.trans(move_component.as_tuple());
                let tile3 = grid.get_tuple(src3);
                if matches!(tile3, Tile::Empty) {
                    if let Some(new_tile3) =
                        tile2.change_tile(UpdateComponent(16, 16), *move_component)
                    {
                        let murphy_tile = Tile::Murphy(
                            UpdateComponent(16, 16),
                            *move_component,
                            get_direction_component(move_component, direction_component),
                            InteractionComponent::Pushing,
                        );
                        updates.push((
                            src,
                            Tile::Transitory(UpdateComponent(16, 16), TransitoryComponent::None),
                        ));
                        updates.push((src2, murphy_tile));
                        updates.push((src3, new_tile3));
                        break;
                    }
                }
            }
            Tile::Ports(port_type) => {
                let src3 = src2.trans(move_component.as_tuple());

                if matches!(grid.get_tuple(src3), Tile::Empty) {
                    let murphy_tile = Tile::Murphy(
                        UpdateComponent(16, 0),
                        *move_component,
                        get_direction_component(move_component, direction_component),
                        InteractionComponent::Tunneling(*port_type),
                    );

                    match (port_type, move_component) {
                        (crate::tile::PortsType::All, _)
                        | (crate::tile::PortsType::Horizontal, MoveComponent::Left)
                        | (crate::tile::PortsType::Horizontal, MoveComponent::Right)
                        | (crate::tile::PortsType::Vertical, MoveComponent::Up)
                        | (crate::tile::PortsType::Vertical, MoveComponent::Down)
                        | (crate::tile::PortsType::Left, MoveComponent::Left)
                        | (crate::tile::PortsType::Right, MoveComponent::Right)
                        | (crate::tile::PortsType::Up, MoveComponent::Up)
                        | (crate::tile::PortsType::Down, MoveComponent::Down) => {
                            updates.push((src, Tile::transitory_empty()));
                            updates.push((src3, murphy_tile));
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => (),
        }
    }

    if updates.is_empty() {
        let murphy_tile = Tile::Murphy(
            UpdateComponent(0, 0),
            MoveComponent::Stationary,
            *direction_component,
            InteractionComponent::None,
        );
        updates.push((src, murphy_tile));
        Some(updates)
    } else {
        Some(updates)
    }
}

fn get_direction_component(
    move_component: &MoveComponent,
    direction_component: &DirectionComponent,
) -> DirectionComponent {
    match move_component {
        MoveComponent::Left => DirectionComponent::Left,
        MoveComponent::Right => DirectionComponent::Right,
        _ => *direction_component,
    }
}

pub fn draw_murphy(
    dst: (f32, f32),
    texture: Texture2D,
    uc: &UpdateComponent,
    mc: &MoveComponent,
    ic: &InteractionComponent,
    dc: &DirectionComponent,
) {
    let smooth_dst = dst.get_offset(uc.0, mc);

    match ic {
        InteractionComponent::None => simple_draw(texture, dst, (0, 7)),
        InteractionComponent::Pushing => {
            advanced_draw(texture, smooth_dst, (3.0, 7.0), dc.as_bool(), false)
        }
        InteractionComponent::Eating(tile) => {
            let x = match uc.0 {
                12..=15 => 10.0,
                8..=11 => 11.0,
                4..=7 => 12.0,
                _ => 13.0,
            };

            match tile {
                InteractionTile::None => {}
                InteractionTile::Base => simple_draw(texture, dst, (0, 0)),
                InteractionTile::Infotron => simple_draw(texture, dst, (0, 4)),
                InteractionTile::RedUtilityDisk => simple_draw(texture, dst, (0, 14)),
            }

            advanced_draw(texture, smooth_dst, (x, 7.0), dc.as_bool(), false);
        }
        InteractionComponent::Slurping => match mc {
            MoveComponent::Up => simple_draw(texture, dst, (4, 7)),
            MoveComponent::Down => simple_draw(texture, dst, (5, 7)),
            MoveComponent::Left => advanced_draw(texture, dst, (6.0, 7.0), true, false),
            MoveComponent::Right => simple_draw(texture, dst, (6, 7)),
            _ => {}
        },
        InteractionComponent::Tunneling(port_type) => {
            //println!("tunneling");
            let x = match uc.0 {
                12..=15 => 10.0,
                8..=11 => 11.0,
                4..=7 => 12.0,
                _ => 13.0,
            };

            advanced_draw(texture, smooth_dst, (x, 7.0), dc.as_bool(), false);
            advanced_draw(
                texture,
                smooth_dst.get_offset(16, mc),
                (x, 7.0),
                dc.as_bool(),
                false,
            );
            simple_draw(texture, dst.get_offset(16, mc), (port_type.as_u8(), 10));
        }
    }
}

pub fn advanced_draw(
    texture: Texture2D,
    dst: (f32, f32),
    pos: (f32, f32),
    flip_x: bool,
    flip_y: bool,
) {
    draw_texture_ex(
        texture,
        dst.0,
        dst.1,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 1.0, y: 1.0 }),
            source: Some(Rect {
                x: pos.0 * 17.0,
                y: pos.1 * 17.0,
                w: 16.0,
                h: 16.0,
            }),
            rotation: 0.0,
            flip_x,
            flip_y,
            pivot: None,
        },
    );
}
