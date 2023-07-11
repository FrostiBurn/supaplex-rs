pub type TileUpdate = f32;

pub trait Updateable {
    fn normal_update(&mut self, time: f32);
    fn dont_update(&mut self);
    fn move_update(&self) -> Self;
}

impl Updateable for TileUpdate {
    fn normal_update(&mut self, time: f32) {
        *self -= time;
    }

    fn dont_update(&mut self) {
        *self = 0.0;
    }

    fn move_update(&self) -> Self {
        self + 1.0
    }
}