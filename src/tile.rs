use macroquad::{
    prelude::{Rect, Vec2, WHITE},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::{
    grid::Cells,
    level::{
        components::{
            direction_component::DirectionComponent, interaction_component::InteractionComponent,
            move_component::MoveComponent, transitory_component::TransitoryComponent,
            update_component::UpdateComponent,
        },
        systems::{
            ai_system::{ai_system, draw_ai},
            gravity_system::gravity_system,
            murphy_system::{draw_murphy, murphy_system},
            simple_gravity_system,
            transitory_system::{draw_transitory, transitory_system},
        },
    },
    FTupleExt,
};

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PortsType {
    All = 0,
    Horizontal = 1,
    Vertical = 2,
    Left = 3,
    Right = 4,
    Up = 5,
    Down = 6,
    AllBlue = 7,
    HorizontalBlue = 8,
    VerticalBlue = 9,
    LeftBlue = 10,
    RightBlue = 11,
    UpBlue = 12,
    DownBlue = 13,
}

impl PortsType {
    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::All => 0,
            Self::Horizontal => 1,
            Self::Vertical => 2,
            Self::Left => 3,
            Self::Right => 4,
            Self::Up => 5,
            Self::Down => 6,
            Self::AllBlue => 7,
            Self::HorizontalBlue => 8,
            Self::VerticalBlue => 9,
            Self::LeftBlue => 10,
            Self::RightBlue => 11,
            Self::UpBlue => 12,
            Self::DownBlue => 13,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum RAMChipsType {
    Base = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum HardwareType {
    Wall = 0,
    RedLight = 1,
    GreenLight = 2,
    BlueLight = 3,
    YellowBlack = 4,
    ResistorsYellow = 5,
    ResistorsRed = 6,
    ResistorsColored = 7,
    ResistorsSpecial1 = 8,
    ResistorsSpecial2 = 9,
    Capacitor = 10,
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Base,
    Bug(UpdateComponent),
    Transitory(UpdateComponent, TransitoryComponent),
    Electron(UpdateComponent, MoveComponent),
    Empty,
    Exit,
    Hardware(HardwareType),
    Infotron(UpdateComponent, MoveComponent),
    Murphy(
        UpdateComponent,
        MoveComponent,
        DirectionComponent,
        InteractionComponent,
    ),
    None,
    Ports(PortsType),
    RAMChips(RAMChipsType),
    SnikSnak(UpdateComponent, MoveComponent, MoveComponent),
    Terminal(UpdateComponent),
    RedUtilityDisk(UpdateComponent),
    OrangeUtilityDisk(UpdateComponent, MoveComponent),
    YellowUtilityDisk(UpdateComponent, MoveComponent),
    Zonk(UpdateComponent, MoveComponent),
}

impl Tile {
    pub fn change_tile(&self, uc: UpdateComponent, mc: MoveComponent) -> Option<Tile> {
        match self {
            Self::Electron(_, _) => Some(Self::Electron(uc, mc)),
            Self::Infotron(_, _) => Some(Self::Infotron(uc, mc)),
            //Self::SnikSnak(_, _, _) => Some(Self::SnikSnak(uc, mc, )),
            Self::OrangeUtilityDisk(_, _) => Some(Self::OrangeUtilityDisk(uc, mc)),
            Self::YellowUtilityDisk(_, _) => Some(Self::YellowUtilityDisk(uc, mc)),
            Self::Zonk(_, _) => Some(Self::Zonk(uc, mc)),
            _ => None,
        }
    }

    pub fn draw(&self, dst: (usize, usize), texture: Texture2D) {
        let dst = (dst.0 as f32, dst.1 as f32);

        match self {
            Tile::Base => simple_draw(texture, dst, (0, 0)),
            Tile::Bug(_) => simple_draw(texture, dst, (0, 0)),
            Tile::Transitory(uc, tc) => draw_transitory(dst, texture, uc, tc),
            Tile::Electron(uc, mc) => simple_draw(texture, dst.get_offset(uc.0, mc), (0, 1)),
            Tile::Exit => simple_draw(texture, dst, (0, 2)),
            Tile::Hardware(hardware_type) => simple_draw(texture, dst, (*hardware_type as u8, 3)),
            Tile::Infotron(uc, mc) => simple_draw(
                texture,
                dst.get_offset(uc.0, mc),
                (get_zonk_infotron_pos(uc, mc), 4),
            ),
            Tile::Murphy(uc, mc, dc, ic) => draw_murphy(dst, texture, uc, mc, ic, dc),
            Tile::Ports(ports_type) => simple_draw(texture, dst, (ports_type.as_u8(), 10)),
            Tile::RAMChips(ramchips_type) => simple_draw(texture, dst, (*ramchips_type as u8, 11)),
            Tile::SnikSnak(uc, mc, mc_pre) => draw_ai(dst, texture, uc, mc, mc_pre),
            Tile::Terminal(_) => simple_draw(texture, dst, (0, 13)),
            Tile::RedUtilityDisk(_) => simple_draw(texture, dst, (0, 14)),
            Tile::OrangeUtilityDisk(uc, mc) => {
                simple_draw(texture, dst.get_offset(uc.0, mc), (0, 15))
            }
            Tile::YellowUtilityDisk(uc, mc) => {
                simple_draw(texture, dst.get_offset(uc.0, mc), (0, 16))
            }
            Tile::Zonk(uc, mc) => simple_draw(
                texture,
                dst.get_offset(uc.0, mc),
                (get_zonk_infotron_pos(uc, mc), 17),
            ),
            _ => {}
        }
    }

    pub fn from_u8(i: u8) -> Self {
        match i {
            1 => Tile::Bug(UpdateComponent::default()),
            2 => Tile::Electron(UpdateComponent::default(), MoveComponent::Stationary),
            3 => Tile::Empty,
            4 => Tile::Exit,
            5 => Tile::Hardware(HardwareType::Wall),
            6 => Tile::Hardware(HardwareType::RedLight),
            7 => Tile::Hardware(HardwareType::GreenLight),
            8 => Tile::Hardware(HardwareType::BlueLight),
            9 => Tile::Hardware(HardwareType::YellowBlack),
            10 => Tile::Hardware(HardwareType::ResistorsYellow),
            11 => Tile::Hardware(HardwareType::ResistorsRed),
            12 => Tile::Hardware(HardwareType::ResistorsColored),
            13 => Tile::Hardware(HardwareType::ResistorsSpecial1),
            14 => Tile::Hardware(HardwareType::ResistorsSpecial2),
            15 => Tile::Hardware(HardwareType::Capacitor),
            16 => Tile::Infotron(UpdateComponent::default(), MoveComponent::Stationary),
            17 => Tile::Murphy(
                UpdateComponent::default(),
                MoveComponent::Stationary,
                DirectionComponent::Left,
                InteractionComponent::None,
            ),
            18 => Tile::Ports(PortsType::All),
            19 => Tile::Ports(PortsType::Horizontal),
            20 => Tile::Ports(PortsType::Vertical),
            21 => Tile::Ports(PortsType::Left),
            22 => Tile::Ports(PortsType::Right),
            23 => Tile::Ports(PortsType::Up),
            24 => Tile::Ports(PortsType::Down),
            25 => Tile::Ports(PortsType::AllBlue),
            26 => Tile::Ports(PortsType::HorizontalBlue),
            27 => Tile::Ports(PortsType::VerticalBlue),
            28 => Tile::Ports(PortsType::LeftBlue),
            29 => Tile::Ports(PortsType::RightBlue),
            30 => Tile::Ports(PortsType::UpBlue),
            31 => Tile::Ports(PortsType::DownBlue),
            32 => Tile::RAMChips(RAMChipsType::Base),
            33 => Tile::RAMChips(RAMChipsType::Left),
            34 => Tile::RAMChips(RAMChipsType::Right),
            35 => Tile::RAMChips(RAMChipsType::Up),
            36 => Tile::RAMChips(RAMChipsType::Down),
            37 => Tile::SnikSnak(
                UpdateComponent::default(),
                MoveComponent::Left,
                MoveComponent::Stationary,
            ),
            38 => Tile::Terminal(UpdateComponent::default()),
            39 => Tile::RedUtilityDisk(UpdateComponent::default()),
            40 => Tile::OrangeUtilityDisk(UpdateComponent::default(), MoveComponent::Stationary),
            41 => Tile::YellowUtilityDisk(UpdateComponent::default(), MoveComponent::Stationary),
            42 => Tile::Zonk(UpdateComponent::default(), MoveComponent::Stationary),
            _ => Tile::Base,
        }
    }

    pub fn simple_draw(&self, dst: (usize, usize), texture: Texture2D) {
        let dst = (dst.0 as f32, dst.1 as f32);

        match self {
            Tile::Base => simple_draw(texture, dst, (0, 0)),
            Tile::Bug(_) => simple_draw(texture, dst, (12, 0)),
            //Tile::Transitory(_, _) => simple_draw(texture, src, (0, 0)),
            Tile::Electron(_, _) => simple_draw(texture, dst, (0, 1)),
            //Tile::Empty => simple_draw(texture, src, (0, 0)),
            Tile::Exit => simple_draw(texture, dst, (0, 2)),
            Tile::Hardware(hardware_type) => simple_draw(texture, dst, (*hardware_type as u8, 3)),
            Tile::Infotron(_, _) => simple_draw(texture, dst, (0, 4)),
            Tile::Murphy(_, _, _, _) => simple_draw(texture, dst, (0, 7)),
            Tile::None => simple_draw(texture, dst, (0, 18)),
            Tile::Ports(ports_type) => simple_draw(texture, dst, (*ports_type as u8, 10)),
            Tile::RAMChips(ramchips_type) => simple_draw(texture, dst, (*ramchips_type as u8, 11)),
            Tile::SnikSnak(_, _, _) => simple_draw(texture, dst, (4, 12)),
            Tile::Terminal(_) => simple_draw(texture, dst, (0, 13)),
            Tile::RedUtilityDisk(_) => simple_draw(texture, dst, (0, 14)),
            Tile::OrangeUtilityDisk(_, _) => simple_draw(texture, dst, (0, 15)),
            Tile::YellowUtilityDisk(_, _) => simple_draw(texture, dst, (0, 16)),
            Tile::Zonk(_, _) => simple_draw(texture, dst, (0, 17)),
            _ => {}
        }
    }

    pub fn transitory_empty() -> Self {
        Self::Transitory(UpdateComponent(16, 0), TransitoryComponent::None)
    }

    pub fn update(
        &self,
        src: (usize, usize),
        grid: &Cells,
        move_component_vec: &[MoveComponent],
        special_key: bool,
    ) -> Option<Vec<((usize, usize), Tile)>> {
        match self {
            Tile::Transitory(update_component, _) => transitory_system(src, update_component),
            Tile::Electron(_, _) => None,
            Tile::Infotron(update_component, move_component) => {
                gravity_system(src, grid, update_component, move_component)
            }
            Tile::Murphy(update_component, _, direction_component, _) => murphy_system(
                src,
                grid,
                update_component,
                move_component_vec,
                direction_component,
                special_key,
            ),
            Tile::SnikSnak(uc, mc, mc_pre) => ai_system(src, grid, uc, mc, mc_pre),
            Tile::OrangeUtilityDisk(update_component, move_component) => {
                simple_gravity_system::simple_gravity_system(
                    src,
                    grid,
                    update_component,
                    move_component,
                )
            }
            //Tile::YellowUtilityDisk(_, _) => None,
            Tile::Zonk(update_component, move_component) => {
                gravity_system(src, grid, update_component, move_component)
            }
            _ => None,
        }
    }

    pub fn update_mut(&mut self) {
        match self {
            Tile::Bug(ref mut update_component)
            | Tile::Electron(ref mut update_component, _)
            | Tile::Infotron(ref mut update_component, _)
            | Tile::Murphy(ref mut update_component, _, _, _)
            | Tile::SnikSnak(ref mut update_component, _, _)
            | Tile::Terminal(ref mut update_component)
            | Tile::Transitory(ref mut update_component, _)
            | Tile::RedUtilityDisk(ref mut update_component)
            | Tile::OrangeUtilityDisk(ref mut update_component, _)
            | Tile::YellowUtilityDisk(ref mut update_component, _)
            | Tile::Zonk(ref mut update_component, _) => update_component.next_cycle(),
            _ => {}
        }
    }
}

fn get_zonk_infotron_pos(update_component: &UpdateComponent, move_component: &MoveComponent) -> u8 {
    match move_component {
        MoveComponent::Left => match update_component.0 {
            15 | 14 => 7,
            13 | 12 => 6,
            11 | 10 => 5,
            9 | 8 => 4,
            7 | 6 => 3,
            5 | 4 => 2,
            3 | 2 => 1,
            _ => 0,
        },
        MoveComponent::Right => match update_component.0 {
            15 | 14 => 1,
            13 | 12 => 2,
            11 | 10 => 3,
            9 | 8 => 4,
            7 | 6 => 5,
            5 | 4 => 6,
            3 | 2 => 7,
            _ => 0,
        },
        _ => 0,
    }
}

pub fn simple_draw(texture: Texture2D, dst: (f32, f32), pos: (u8, u8)) {
    draw_texture_ex(
        texture,
        dst.0,
        dst.1,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 1.0, y: 1.0 }),
            source: Some(Rect {
                x: pos.0 as f32 * 17.0,
                y: pos.1 as f32 * 17.0,
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
