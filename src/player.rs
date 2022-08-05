use macroquad::prelude::*;
extern crate rand;
use crate::{MOVE_SPEED, TILE_SIZE};
use rand::Rng;

// pub const TILE_SIZE: f32 = 25f32;
// pub const MOVE_SPEED: f32 = 25f32;

pub enum GameState {
    Play,
    Start,
    Setup,
    Gameover,
    Pause,
}

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

    pub fn go_to(&mut self, x: f32, y: f32) {
        self.rect.x = x;
        self.rect.y = y;
    }
}

pub struct Player {
    pub body: Vec<Cube>,
    pub food: Cube,
    pub dir: Vec<f32>,
    pub colour: Color,
}

impl Player {
    pub fn new(x: f32, y: f32, color: Color) -> Player {
        Player {
            body: vec![
                Cube::new(
                    x,
                    y,
                    Color {
                        r: 255f32,
                        g: 0f32,
                        b: 0f32,
                        a: 255f32,
                    },
                ),
                Cube::new(x - TILE_SIZE, y, color),
                Cube::new(x - TILE_SIZE * 2f32, y, color),
                Cube::new(x - TILE_SIZE * 3f32, y, color),
            ],
            food: Cube::new(
                screen_width() - 3f32 * TILE_SIZE,
                (screen_height() / TILE_SIZE / 2f32 - 1f32) * TILE_SIZE,
                GREEN,
            ),
            dir: vec![1f32, 0f32],
            colour: color,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.food.rect.x,
            self.food.rect.y,
            self.food.rect.w,
            self.food.rect.h,
            self.food.colour,
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

    pub fn touching_food(&self) -> bool {
        for cube in self.body.iter() {
            if self.food.rect.overlaps(&cube.rect) {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, frame_rect: &mut Rect) -> GameState {
        let mut x: i32;
        for i in (1..self.body.len()).rev() {
            x = (i as i32) - 1;
            self.body[i].dir = self.body[x as usize].dir.clone();
        }

        self.body[0].dir = self.dir.clone();

        for i in 0..self.body.len() {
            let old_x = self.body[i].rect.x.clone();
            let old_y = self.body[i].rect.y.clone();

            let cur_dir = vec![self.body[i].dir[0].clone(), self.body[i].dir[1].clone()];

            self.body[i].go_to(
                old_x + (cur_dir[0] * MOVE_SPEED),
                old_y + (cur_dir[1] * MOVE_SPEED),
            );
        }

        if !frame_rect.contains(self.get_head_center()) || self.collided_self() {
            return GameState::Gameover;
        }

        self.draw();

        if self.touching_food() {
            self.append();

            self.food.go_to(
                rand::thread_rng().gen_range(0f32..screen_width() - self.body[0].rect.w),
                rand::thread_rng().gen_range(25f32..screen_height() - self.body[0].rect.h),
            );
            while self.touching_food() {
                self.food.go_to(
                    rand::thread_rng().gen_range(0f32..screen_width() - self.body[0].rect.w),
                    rand::thread_rng().gen_range(25f32..screen_height() - self.body[0].rect.h),
                );
            }
        }
        GameState::Play
    }

    pub fn get_head_center(&self) -> Vec2 {
        vec2(self.body[0].rect.x+(self.body[0].rect.w*0.5f32), self.body[0].rect.y+(self.body[0].rect.h*0.5f32))
    }

    pub fn collided_self(&self) -> bool {
        let head_center = self.get_head_center();
        for i in 1..self.body.len() {
            if self.body[i].rect.contains(head_center) {
                return true;
            }
        }
        false
    }

    pub fn append(&mut self) {
        let prev_cube_dir = self.body[self.body.len() - 1].dir.clone();
        let prev_cube_pos = vec![
            self.body[self.body.len() - 1].rect.x.clone(),
            self.body[self.body.len() - 1].rect.y.clone(),
        ];
        self.body.push(Cube::new(
            prev_cube_pos[0] + (-1f32 * prev_cube_dir[0] * TILE_SIZE),
            prev_cube_pos[1] + (-1f32 * prev_cube_dir[1] * TILE_SIZE),
            self.colour.clone(),
        ));
    }
}
