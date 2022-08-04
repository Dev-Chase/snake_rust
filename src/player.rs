pub use macroquad::prelude::*;

// pub const TILE_SIZE: f32 = 25f32;
pub const MOVE_SPEED: f32 = 25f32;
pub struct Player {
    pub coords: Vec<f32>,
    pub dir: Vec<f32>,
    pub w: f32,
    pub h: f32,
    pub colour: Color,
}

impl Player {
    pub fn draw(&self) {
        draw_rectangle(self.coords[0], self.coords[1], self.w, self.h, self.colour);
    }

    pub fn mv(&mut self) {
        self.coords[0] += self.dir[0] * MOVE_SPEED;
        self.coords[1] += self.dir[1] * MOVE_SPEED;
    }
}
