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
    pub fn draw(&self) {
        match self {
            Tile::Base => todo!(),
            Tile::Bug => todo!(),
            Tile::Electron => todo!(),
            Tile::Empty => todo!(),
            Tile::Exit => todo!(),
            Tile::Hardware(_) => todo!(),
            Tile::Infotron => todo!(),
            Tile::Murphy => todo!(),
            Tile::None => todo!(),
            Tile::Ports(_) => todo!(),
            Tile::RAMChips(_) => todo!(),
            Tile::SnikSnak => todo!(),
            Tile::Terminal => todo!(),
            Tile::UtilityDisks(_) => todo!(),
            Tile::Zonk => todo!(),
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Tile::Base => 0,
            Tile::Bug => 1,
            Tile::Electron => 2,
            Tile::Empty => 3,
            Tile::Exit => 4,
            Tile::Hardware(_) => 5,
            Tile::Infotron => 6,
            Tile::Murphy => 7,
            Tile::Ports(_) => 9,
            Tile::RAMChips(_) => 9,
            Tile::SnikSnak => 10,
            Tile::Terminal => 11,
            Tile::UtilityDisks(_) => 12,
            Tile::Zonk => 13,
            Tile::None => 14,
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