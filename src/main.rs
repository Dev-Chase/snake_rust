use macroquad::prelude::*;

mod player;

use player::Player;

pub const TILE_SIZE: f32 = 25f32;
pub const MOVE_SPEED: f32 = 25f32;

fn create_window_conf() -> Conf {
    Conf {
        window_title: String::from("Snake"),
        fullscreen: false,
        window_width: 500,
        window_height: 500,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(create_window_conf())]
async fn main() {
    // Creating Player "Class"
    let mut player = Player::new(TILE_SIZE*4f32, (screen_height()/TILE_SIZE/2f32-1f32)*TILE_SIZE, RED);
    // Creating Input Vector
    let mut input = vec![1f32, 0f32];
    // Creating Frame Variables
    let mut minimum_frame_time: f32;
    let mut frame_time: f32;
    let mut time_to_sleep = 0f32;

    loop {
        
        clear_background(BLACK);
        
        if is_key_down(KeyCode::Right) && (input[0] != -1f32 && input[0] != 1f32){
            input = vec![1f32, 0f32];
        } else if is_key_down(KeyCode::Down) && (input[1] != -1f32 && input[1] != 1f32){
            input = vec![0f32, 1f32];
        } else if is_key_down(KeyCode::Left) && (input[0] != 1f32 && input[0] != -1f32){
            input = vec![-1f32, 0f32];
        }else if is_key_down(KeyCode::Up) && (input[1] != 1f32 && input[1] != -1f32){
            input = vec![0f32, -1f32];
        }
        
        
        player.dir = input.clone();
        player.update();

        minimum_frame_time = 1. / 8.; // 8 FPS
        frame_time = get_frame_time()-time_to_sleep*0.001f32;
        time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));

        
        next_frame().await
    }
}