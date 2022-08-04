pub use macroquad::prelude::*;

pub const TILE_SIZE: f32 = 25f32;
pub const _MOVE_SPEED: f32 = TILE_SIZE;

pub struct Cube {
    pub colour: Color,
    pub rect: Rect,
    pub dir: Vec<f32>,
}

impl Cube {
    pub fn new(x: f32, y: f32, color: Color) -> Cube {
        Cube {
            colour: color,
            dir: vec![1f32, 0f32],
            rect: Rect::new(x, y, TILE_SIZE, TILE_SIZE),
        }
    }
}

pub struct Player {
    pub head: Cube,
    pub body: Vec<Cube>,
    pub dir: Vec<f32>,
}

impl Player {
    pub fn new(x: f32, y: f32, color: Color) -> Player {
        Player {
            head: Cube::new(x, y, color),
            body: vec![
                Cube::new(x - TILE_SIZE, y, color),
                Cube::new(x - TILE_SIZE * 2f32, y, color),
            ],
            dir: vec![1f32, 0f32],
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.head.rect.x,
            self.head.rect.y,
            self.head.rect.w,
            self.head.rect.h,
            self.head.colour,
        );
        for cube in self.body.iter() {
            draw_rectangle(
                cube.rect.x,
                cube.rect.y,
                cube.rect.w,
                cube.rect.h,
                cube.colour,
            );
        }
    }

    // pub fn mv(&mut self) {
    //     self.coords = vec![self.coords[0]+(self.dir[0] * MOVE_SPEED), self.coords[1]+(self.dir[1] * MOVE_SPEED)];
    // }
}
