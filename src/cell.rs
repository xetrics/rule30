use crate::color::Color;

#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    alive: bool,
}

impl Cell {
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn color(&self) -> Color {
        if self.alive {
            Color::ACCENT
        } else {
            Color::BACKGROUND
        }
    }
}
