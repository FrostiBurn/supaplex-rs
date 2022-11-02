#[derive(Clone, Copy)]
pub struct BugState {
    pub update_cycle: u8,
    pub animation_cycle: u8,
}

impl BugState {
    pub fn new() -> Self {
        Self {
            update_cycle: 0,
            animation_cycle: 0,
        }
    }

    pub fn update(&mut self) {

    }
}