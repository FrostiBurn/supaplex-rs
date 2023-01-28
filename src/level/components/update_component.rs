#[derive(Clone, Copy, Debug, Default)]
pub struct UpdateComponent(pub u8, pub u8);

impl UpdateComponent {
    pub fn next_cycle(&mut self) {
        if self.1 != 0 {
            self.1 -= 1;
        } else if self.0 != 0 {
            self.0 -= 1;
        }
    }
}
