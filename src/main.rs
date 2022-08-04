mod settings;
mod player;

use settings::*;
use player::Player;

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
    request_new_screen_size(500f32, 500f32);
    let mut player = Player{coords: vec![screen_width()*0.5f32-TILE_SIZE*0.5f32, screen_height()*0.5f32-TILE_SIZE*0.5f32], w: TILE_SIZE, h: TILE_SIZE, dir: vec![1f32, 0f32], colour: RED};
    loop {
        
        clear_background(BLACK);
        
        let keys = vec![is_key_down(KeyCode::Up), is_key_down(KeyCode::Down), is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)];
        //TODO: make this 2d vector and finish match statement
        player.dir = match keys {
            
            _ => vec![1f32, 0f32]
        };

        for (i, key) in keys.iter().enumerate() {
            if *key{
                if i == 0 {
                    player.dir = vec![-1f32, 0f32];
                } else if i == 1 {
                    player.dir = vec![1f32, 0f32];
                } else if i == 2 {
                    player.dir = vec![0f32, -1f32];
                } else{
                    player.dir = vec![0f32, 1f32];
                }
            } 
        }

        player.mv();
        player.draw();

        // println!("{}", 1. / 6.);
        let minimum_frame_time = 1. / 7.; // 8 FPS
        let frame_time = get_frame_time();
        // println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            // println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await
    }
}