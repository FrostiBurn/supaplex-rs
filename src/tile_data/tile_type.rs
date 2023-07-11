use macroquad::{
    prelude::{Rect, Vec2, PURPLE, WHITE},
    text::{draw_text_ex, TextParams},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::grid::{Coord, FCoord};

use super::{
    tile::Tile,
    tile_state::TileState,
    tile_update::TileUpdate, tile_move::TileMove,
};

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum TileType {
    Base,
    Bug,
    Electron,
    Empty,
    Exit,
    HardwareBlueLight,
    HardwareCapacitor,
    HardwareGreenLight,
    HardwareRedLight,
    HardwareResistorsColored,
    HardwareResistorsRed,
    HardwareResistorsSpecial1,
    HardwareResistorsSpecial2,
    HardwareResistorsYellow,
    HardwareWall,
    HardwareYellowBlack,
    Infotron,
    Murphy,
    #[default]
    None,
    OrangeUtilityDisk,
    PortsAll,
    PortsAllBlue,
    PortsDown,
    PortsDownBlue,
    PortsHorizontal,
    PortsHorizontalBlue,
    PortsLeft,
    PortsLeftBlue,
    PortsRight,
    PortsRightBlue,
    PortsUp,
    PortsUpBlue,
    PortsVertical,
    PortsVerticalBlue,
    RAMChipsBase,
    RAMChipsDown,
    RAMChipsLeft,
    RAMChipsRight,
    RAMChipsUp,
    RedUtilityDisk,
    SnikSnak,
    Terminal,
    Transitory,
    YellowUtilityDisk,
    Zonk,
    Explosion,
    Explosion2,
}

impl TileType {
    pub const fn def_tex_pos(&self) -> Coord {
        match self {
            TileType::Base => Coord::new(0, 0),
            TileType::Bug => Coord::new(8, 0),
            TileType::Transitory => Coord::new(0, 20),
            TileType::Electron => Coord::new(0, 1),
            TileType::Empty => Coord::new(1, 2),
            TileType::Exit => Coord::new(0, 2),
            TileType::HardwareWall => Coord::new(0, 3),
            TileType::HardwareRedLight => Coord::new(1, 3),
            TileType::HardwareGreenLight => Coord::new(2, 3),
            TileType::HardwareBlueLight => Coord::new(3, 3),
            TileType::HardwareYellowBlack => Coord::new(4, 3),
            TileType::HardwareResistorsYellow => Coord::new(5, 3),
            TileType::HardwareResistorsRed => Coord::new(6, 3),
            TileType::HardwareResistorsColored => Coord::new(7, 3),
            TileType::HardwareResistorsSpecial1 => Coord::new(8, 3),
            TileType::HardwareResistorsSpecial2 => Coord::new(9, 3),
            TileType::HardwareCapacitor => Coord::new(10, 3),
            TileType::Infotron => Coord::new(0, 4),
            TileType::Murphy => Coord::new(0, 7),
            TileType::None => Coord::new(0, 20),
            TileType::PortsAll => Coord::new(0, 10),
            TileType::PortsHorizontal => Coord::new(1, 10),
            TileType::PortsVertical => Coord::new(2, 10),
            TileType::PortsLeft => Coord::new(3, 10),
            TileType::PortsRight => Coord::new(4, 10),
            TileType::PortsUp => Coord::new(5, 10),
            TileType::PortsDown => Coord::new(6, 10),
            TileType::PortsAllBlue => Coord::new(7, 10),
            TileType::PortsHorizontalBlue => Coord::new(8, 10),
            TileType::PortsVerticalBlue => Coord::new(9, 10),
            TileType::PortsLeftBlue => Coord::new(10, 10),
            TileType::PortsRightBlue => Coord::new(11, 10),
            TileType::PortsUpBlue => Coord::new(12, 10),
            TileType::PortsDownBlue => Coord::new(13, 10),
            TileType::RAMChipsBase => Coord::new(0, 11),
            TileType::RAMChipsLeft => Coord::new(1, 11),
            TileType::RAMChipsRight => Coord::new(2, 11),
            TileType::RAMChipsUp => Coord::new(3, 11),
            TileType::RAMChipsDown => Coord::new(4, 11),
            TileType::SnikSnak => Coord::new(4, 12),
            TileType::Terminal => Coord::new(0, 13),
            TileType::RedUtilityDisk => Coord::new(0, 14),
            TileType::OrangeUtilityDisk => Coord::new(0, 15),
            TileType::YellowUtilityDisk => Coord::new(0, 16),
            TileType::Zonk => Coord::new(0, 17),
            TileType::Explosion | TileType::Explosion2 => Coord::new(0, 18),
        }
    }

    pub fn from_u8(i: u8) -> Self {
        match i {
            1 => TileType::Zonk,
            2 => TileType::Base,
            3 => TileType::Murphy,
            4 => TileType::Infotron,
            5 => TileType::RAMChipsBase,
            6 => TileType::HardwareWall,
            7 => TileType::Exit,
            8 => TileType::OrangeUtilityDisk,
            9 => TileType::PortsRight,
            10 => TileType::PortsDown,
            11 => TileType::PortsLeft,
            12 => TileType::PortsUp,
            13 => TileType::Terminal,
            14 => TileType::RedUtilityDisk,
            15 => TileType::HardwareCapacitor,
            16 => TileType::Infotron,
            17 => TileType::SnikSnak,
            18 => TileType::YellowUtilityDisk,
            19 => TileType::Terminal,
            20 => TileType::RedUtilityDisk,
            21 => TileType::PortsVertical,
            22 => TileType::PortsHorizontal,
            23 => TileType::PortsAll,
            24 => TileType::Electron,
            25 => TileType::Bug,
            26 => TileType::RAMChipsLeft,
            27 => TileType::RAMChipsRight,
            28 => TileType::HardwareResistorsSpecial2,
            29 => TileType::HardwareGreenLight,
            30 => TileType::HardwareBlueLight,
            31 => TileType::HardwareRedLight,
            32 => TileType::HardwareYellowBlack,
            33 => TileType::HardwareResistorsSpecial1,
            34 => TileType::HardwareCapacitor,
            35 => TileType::HardwareResistorsColored,
            36 => TileType::HardwareResistorsRed,
            37 => TileType::HardwareResistorsYellow,
            38 => TileType::RAMChipsUp,
            39 => TileType::RAMChipsDown,
            40 => TileType::None,
            _ => TileType::Empty,
        }
    }

    pub fn to_tile(self) -> Tile {
        match self {
            TileType::Base
            | TileType::Bug
            | TileType::Infotron
            | TileType::RedUtilityDisk
            | TileType::Empty => Tile {
                typ: self,
                state: TileState::Eatable,
                ..Default::default()
            },
            TileType::Electron | TileType::SnikSnak => Tile {
                typ: self,
                state: TileState::Dangerous,
                mov: TileMove::Up,
                mov2: TileMove::Up,
                ..Default::default()
            },
            TileType::HardwareWall
            | TileType::HardwareRedLight
            | TileType::HardwareGreenLight
            | TileType::HardwareBlueLight
            | TileType::HardwareYellowBlack
            | TileType::HardwareResistorsYellow
            | TileType::HardwareResistorsRed
            | TileType::HardwareResistorsColored
            | TileType::HardwareResistorsSpecial1
            | TileType::HardwareResistorsSpecial2
            | TileType::HardwareCapacitor
            | TileType::None => Tile {
                typ: self,
                state: TileState::Indestructible,
                ..Default::default()
            },
            TileType::Murphy => Tile {
                typ: self,
                state: TileState::Destructible,
                ..Default::default()
            },
            TileType::PortsAll
            | TileType::PortsHorizontal
            | TileType::PortsVertical
            | TileType::PortsLeft
            | TileType::PortsRight
            | TileType::PortsUp
            | TileType::PortsDown
            | TileType::PortsAllBlue
            | TileType::PortsHorizontalBlue
            | TileType::PortsVerticalBlue
            | TileType::PortsLeftBlue
            | TileType::PortsRightBlue
            | TileType::PortsUpBlue
            | TileType::PortsDownBlue => Tile {
                typ: self,
                state: TileState::Tunnelable,
                ..Default::default()
            },
            TileType::RAMChipsBase
            | TileType::RAMChipsLeft
            | TileType::RAMChipsRight
            | TileType::RAMChipsUp
            | TileType::RAMChipsDown
            | TileType::Terminal
            | TileType::Exit => Tile {
                typ: self,
                state: TileState::Destructible,
                ..Default::default()
            },
            TileType::Transitory => Tile {
                typ: self,
                state: TileState::Destructible,
                ..Default::default()
            },
            TileType::OrangeUtilityDisk | TileType::YellowUtilityDisk | TileType::Zonk => Tile {
                typ: self,
                state: TileState::Moveable,
                ..Default::default()
            },
            TileType::Explosion | TileType::Explosion2 => Tile {
                typ: self,
                state: TileState::Dangerous,
                ..Default::default()
            },
        }
    }
}

pub fn simple_draw(texture: Texture2D, fcoord: &FCoord, pos: &Coord, flip_x: bool) {
    draw_texture_ex(
        texture,
        fcoord.x * 16.0,
        fcoord.y * 16.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 16.0, y: 16.0 }),
            source: Some(Rect {
                x: pos.x as f32 * 17.0,
                y: pos.y as f32 * 17.0,
                w: 16.0,
                h: 16.0,
            }),
            rotation: 0.0,
            flip_x,
            flip_y: false,
            pivot: None,
        },
    );
}

pub fn draw_time(coord: &FCoord, time: TileUpdate) {
    let time = (time * 100.0).round() / 100.0;
    draw_text_ex(
        &format!("{time}"),
        coord.x,
        coord.y + 16.0,
        TextParams {
            font_size: 128,
            font_scale: 0.05,
            color: PURPLE,
            ..Default::default()
        },
    );
}
