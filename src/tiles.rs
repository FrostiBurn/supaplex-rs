use macroquad::{texture::{Texture2D, draw_texture_ex, DrawTextureParams}, prelude::{WHITE, Vec2, Rect}};

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Moveable {
    Stationary = 0,
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

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
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum UtilityDisksType {
    Red = 0,
    Orange = 1,
    Yellow = 2,
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
    Bug,
    Electron,
    Empty,
    Exit,
    Hardware(HardwareType),
    Infotron,
    Murphy,
    None,
    Ports(PortsType),
    RAMChips(RAMChipsType),
    SnikSnak,
    Terminal,
    UtilityDisks(UtilityDisksType),
    Zonk,
}

impl Tile {
    pub fn draw(&self, texture: Texture2D, x: f32, y: f32) {
        match self {
            Tile::Base =>           simple_draw(texture, x, y, 0.0, 0.0),
            Tile::Bug =>            simple_draw(texture, x, y, 10.0, 0.0),
            Tile::Electron =>       simple_draw(texture, x, y, 0.0, 1.0),
            Tile::Empty =>          {},
            Tile::Exit =>           simple_draw(texture, x, y, 0.0, 2.0),
            Tile::Hardware(_) =>    simple_draw(texture, x, y, 0.0, 3.0),
            Tile::Infotron =>       simple_draw(texture, x, y, 0.0, 4.0),
            Tile::Murphy =>         {},
            Tile::None =>           {},
            Tile::Ports(_) =>       simple_draw(texture, x, y, 0.0, 10.0),
            Tile::RAMChips(_) =>    simple_draw(texture, x, y, 0.0, 11.0),
            Tile::SnikSnak =>       simple_draw(texture, x, y, 0.0, 12.0),
            Tile::Terminal =>       simple_draw(texture, x, y, 0.0, 13.0),
            Tile::UtilityDisks(_) =>simple_draw(texture, x, y, 0.0, 14.0),
            Tile::Zonk =>           simple_draw(texture, x, y, 0.0, 15.0),
        }
    }

    pub fn from_u8(i: u8) -> Self {
        match i {
            1 => Tile::Bug,
            2 => Tile::Electron,
            3 => Tile::Empty,
            4 => Tile::Exit,
            5 => Tile::Hardware(HardwareType::Wall),
            6 => Tile::Infotron,
            7 => Tile::Murphy,
            8 => Tile::Ports(PortsType::All),
            9 => Tile::RAMChips(RAMChipsType::Base),
            10 => Tile::SnikSnak,
            11 => Tile::Terminal,
            12 => Tile::UtilityDisks(UtilityDisksType::Red),
            13 => Tile::Zonk,
            _ => Tile::Base,
        }
    }
}

fn simple_draw(texture: Texture2D, x: f32, y: f32, x_pos: f32, y_pos: f32) {
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 1.0, y: 1.0 }),
            source: Some(Rect {
                x: x_pos * 17.0,
                y: y_pos * 17.0,
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