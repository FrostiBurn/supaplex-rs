use std::slice::Iter;

use macroquad::texture::Texture2D;

use crate::{
    level::components::{
        move_component::MoveComponent, transitory_component::TransitoryComponent,
        update_component::UpdateComponent,
    },
    tile::Tile,
};

pub struct Grid {
    cols: usize,
    cells: Cells,
    //updated_cells: UpdatingCells,
}

impl Grid {
    pub fn new(size: (usize, usize), value: Tile) -> Self {
        let cols = size.0;
        let cells = vec![value; size.0 * size.1];
        Self {
            cols,
            //updated_cells: UpdatingCells::new(cols, cells.len()),
            cells: Cells::new(cols, cells),
        }
    }

    pub fn new_from(cols: usize, grid: Vec<Tile>) -> Self {
        Self {
            cols,
            //updated_cells: UpdatingCells::new(cols, grid.len()),
            cells: Cells::new(cols, grid),
        }
    }
    pub fn update(&mut self, move_component_vec: &[MoveComponent], special_key: bool) {
        self.cells.iter_mut().for_each(|tile| tile.update_mut());

        let (mut x, mut y) = (0usize, 0usize);
        for _ in 0..self.cells.len() {
            if let Some(updates) =
                self.cells
                    .get(x, y)
                    .update((x, y), &self.cells, move_component_vec, special_key)
            {
                updates.into_iter().for_each(|update| {
                    self.cells.set(update.0, update.1);
                });
            }

            x += 1;
            if x == self.cols {
                y += 1;
                x = 0;
            }
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        let (mut x, mut y) = (0, 0);
        self.cells.iter().for_each(|cell| {
            cell.draw((x, y), texture);
            x += 1;
            if x == self.cols {
                y += 1;
                x = 0;
            }
        });
    }
}

pub struct Cells {
    cols: usize,
    cells: Vec<Tile>,
}

impl Cells {
    pub fn new(cols: usize, cells: Vec<Tile>) -> Self {
        Self { cols, cells }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn iter(&self) -> Iter<Tile> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<Tile> {
        self.cells.iter_mut()
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut Tile> {
        self.cells.get_mut(i)
    }

    pub fn get(&self, x: usize, y: usize) -> &Tile {
        match self.cells.get(x + y * self.cols) {
            Some(tile) => tile,
            None => &Tile::None,
        }
    }

    pub fn get_tuple(&self, src: (usize, usize)) -> &Tile {
        match self.cells.get(src.0 + src.1 * self.cols) {
            Some(tile) => tile,
            None => &Tile::None,
        }
    }

    pub fn get_trans(&self, src: (usize, usize), trans: (i16, i16)) -> &Tile {
        let index =
            (src.0 as i16 + trans.0) as usize + ((src.1 as i16 + trans.1) as usize * self.cols);
        match self.cells.get(index) {
            Some(tile) => {
                //println!("[{}] eatable: {}, kind: {:?}", index, tile.is_eatable, tile.kind);
                tile
            }
            None => &Tile::None,
        }
    }

    pub fn set(&mut self, dst: (usize, usize), value: Tile) -> bool {
        match self.get_mut(dst.0 + dst.1 * self.cols) {
            Some(tile) => *tile = value,
            None => return false,
        };
        true
    }

    pub fn set_trans(&mut self, src: (usize, usize), trans: (i16, i16), tile: Tile) -> bool {
        self.set(
            (
                (src.0 as i16 + trans.0) as usize,
                (src.1 as i16 + trans.1) as usize,
            ),
            tile,
        )
    }

    pub fn mov(&mut self, grid: &Cells, src: (usize, usize), trans: (i16, i16)) {
        let tile = grid.get(src.0, src.1);
        if self.set(
            (
                (src.0 as i16 + trans.0) as usize,
                (src.1 as i16 + trans.1) as usize,
            ),
            *tile,
        ) {
            self.set(src, Tile::Empty);
        }
    }

    pub fn mov_val(&mut self, src: (usize, usize), trans: (i16, i16), value: Tile) {
        self.set(
            (
                (src.0 as i16 + trans.0) as usize,
                (src.1 as i16 + trans.1) as usize,
            ),
            value,
        );
        self.set(
            src,
            Tile::Transitory(UpdateComponent(16, 0), TransitoryComponent::None),
        );
    }
}
