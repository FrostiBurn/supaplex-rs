use crate::tile_data::{
    tile::Tile, tile_interaction::TileInteraction, tile_move::TileMove,
    tile_state::TileState, tile_type::TileType,
};

const BORDER_TILE: Tile = Tile {
    upd: 0.0,
    typ: TileType::None,
    state: TileState::Indestructible,
    int: TileInteraction::None,
    mov: TileMove::None,
    mov2: TileMove::Left,
};

#[derive(Clone)]
pub struct Grid {
    pub width: i16,
    pub height: i16,
    pub array: Vec<Tile>,
}

impl Grid {
    pub fn new(width: i16, height: i16, array: Vec<u8>) -> Self {
        let array: Vec<Tile> = array
            .into_iter()
            .map(|i| TileType::to_tile(TileType::from_u8(i)))
            .collect();

        Self {
            width,
            height,
            array,
        }
    }

    pub fn get(&self, coord: &Coord) -> &Tile {
        match self
            .array
            .get((coord.y * self.width + coord.x) as usize)
        {
            Some(tile) => tile,
            None => &BORDER_TILE,
        }
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut Tile> {
        self.array
            .get_mut((coord.y * self.width + coord.x) as usize)
    }

    pub fn get_mut_unchecked(&mut self, coord: &Coord) -> &mut Tile {
        unsafe {
            self.array
                .get_unchecked_mut((coord.y * self.width + coord.x) as usize)
        }
    }

    pub fn set(&mut self, coord: &Coord, tile: Tile) {
        if let Some(current_tile) = self
            .array
            .get_mut((coord.y * self.width + coord.x) as usize)
        {
            *current_tile = tile;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl Coord {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn offset(&self, rhs: &impl ToCoord) -> Self {
        let offset = rhs.to_coord();

        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    pub fn to_fcoord(self, tile: &Tile) -> FCoord {
        let upd = tile.upd.max(0.0);

        match &tile.mov {
            TileMove::Up => FCoord::new(self.x as f32, self.y as f32 + upd),
            TileMove::Right => FCoord::new(self.x as f32 - upd, self.y as f32),
            TileMove::Down => FCoord::new(self.x as f32, self.y as f32 - upd),
            TileMove::Left => FCoord::new(self.x as f32 + upd, self.y as f32),
            TileMove::None => FCoord::new(self.x as f32, self.y as f32),
        }
    }

    pub fn as_fcoord(&self) -> FCoord {
        FCoord::new(self.x as f32, self.y as f32)
    }
}

pub trait ToCoord {
    fn to_coord(&self) -> Coord;
}

#[derive(Clone, Copy)]
pub struct FCoord {
    pub x: f32,
    pub y: f32,
}

impl FCoord {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn offset_time(&self, tile: &Tile) -> FCoord {
        let upd = tile.upd.max(0.0);

        match &tile.mov {
            TileMove::Up => FCoord::new(self.x, self.y + upd),
            TileMove::Right => FCoord::new(self.x - upd, self.y),
            TileMove::Down => FCoord::new(self.x, self.y - upd),
            TileMove::Left => FCoord::new(self.x + upd, self.y),
            TileMove::None => FCoord::new(self.x, self.y),
        }
    }

    pub fn offset(&self, rhs: &impl ToCoord) -> Self {
        let offset = rhs.to_coord();

        Self {
            x: self.x + offset.x as f32,
            y: self.y + offset.y as f32,
        }
    }
}

impl ToCoord for (i16, i16) {
    fn to_coord(&self) -> Coord {
        Coord { x: self.0, y: self.1 }
    }
}
