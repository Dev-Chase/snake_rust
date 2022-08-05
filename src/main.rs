use macroquad::prelude::*;

mod player;
use player::{Player, GameState};

const TILE_SIZE: f32 = 25f32;
const MOVE_SPEED: f32 = 25f32;


// Creating a Function for the Play Game Status
fn play(player: &mut Player, input: &Vec<f32>, score: &mut f32, frame_rect: &mut Rect, font: &Font) -> GameState {
    // Drawing Background Frame Rect
    draw_rectangle(frame_rect.x, frame_rect.y, frame_rect.w, frame_rect.h, BLACK);

    // Changing the Player Direction to Reflect the Input Variable
    player.dir = input.clone();
    let game_state_clone = player.update(frame_rect);

    if is_key_released(KeyCode::Space) {
        return GameState::Pause;
    }

    *score = (player.body.len() as f32) - 4f32;

    match game_state_clone {
        GameState::Gameover => {
            return game_state_clone;
        }
        _ => {
            // Creating Score Text Variable
            let text = format!("Score: {}", score);
        
            // Drawing Top Border
            draw_rectangle(0f32, 0f32, screen_width(), 24f32, BLACK);
            draw_line(0f32, 24f32, screen_width(), 24f32, 2f32, WHITE);
            draw_text_ex(text.as_str(), 0f32, 20f32, TextParams { font: *font, font_size: 22u16, font_scale: 1f32, font_scale_aspect: 1f32, color: WHITE });
        }
    }
    game_state_clone
}

// Creating a Function for the Setup Game Status
fn setup(input: &mut Vec<f32>, player: &mut Player, score: &mut f32) -> GameState {
    // Creating new Player
    *input = vec![1f32, 0f32];
    *player = Player::new(TILE_SIZE*4f32, (screen_height()/TILE_SIZE/2f32-1f32)*TILE_SIZE, Color{r: 255f32, g: 0f32, b: 0f32, a: 255f32});
    *score = 0f32;

    GameState::Play
}

// Creating a Function for the Start Game Status
fn start(font: &Font) -> GameState {
    // Checking to See if the Space key or Left mouse button is pressed and then Returning Setup Game State
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Setup;
    }

    // Measuring the Start text
    let start_text_dimension = measure_text("Press Space or Click to Start", Some(*font), 25u16, 1f32);

    // Drawing the Start Text
    draw_text_ex("Press Space or Click to Start", screen_width()*0.5f32-(start_text_dimension.width*0.5f32), screen_height()*0.5f32-(start_text_dimension.height*0.5f32), TextParams { font: *font, font_size: 25u16, font_scale: 1f32, font_scale_aspect: 1f32, color: WHITE });

    GameState::Start
}

// Creating a Function for the Gameover Game Status
fn gameover(score: f32, font: &Font) -> GameState {
    // Checking to See if the Space key or Left mouse button is pressed and then Returning Setup Game State
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Setup;
    }

    // Creating Score Text Variable
    let score_text = format!("Score: {}", score);

    // Measuring text Sections
    let start_text_dimension = measure_text("Press Space or Click to Start", Some(*font), 25u16, 1f32);
    let gameover_text_dimension = measure_text("Gameover", Some(*font), 26u16, 1f32);
    let score_text_dimension = measure_text(score_text.as_str(), Some(*font), 26u16, 1f32);

    // Drawing the Text Sections
    draw_text_ex("Gameover", screen_width()*0.5f32-(gameover_text_dimension.width*0.5f32), screen_height()*0.5f32-gameover_text_dimension.height-(start_text_dimension.height*0.5f32)-4.5f32, TextParams { font: *font, font_size: 26u16, font_scale: 1f32, font_scale_aspect: 1f32, color: WHITE });
    draw_text_ex("Press Space or Click to Start", screen_width()*0.5f32-(start_text_dimension.width*0.5f32), screen_height()*0.5f32-(start_text_dimension.height*0.5f32), TextParams { font: *font, font_size: 25u16, font_scale: 1f32, font_scale_aspect: 1f32, color: WHITE });
    draw_text_ex(score_text.as_str(), screen_width()*0.5f32-(score_text_dimension.width*0.5f32), screen_height()*0.5f32+score_text_dimension.height-8f32, TextParams { font: *font, font_size: 26u16, font_scale: 1f32, font_scale_aspect: 1f32, color: WHITE });

    GameState::Gameover
}

// Creating a Function for the Pause Game Status
fn pause(score: f32, font: &Font) -> GameState{
    // Checking to See if the Space key or Left mouse button is pressed and then Returning Play Game State
    if is_key_released(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
        return GameState::Play;
    }

    // Creating Score text Variable
    let score_text = format!("Score: {}", score);

    // Finding Text Measurements and Settings them To Variables
    let unpause_text_dimension = measure_text("Press Space or Click to Unpause", Some(*font), 25u16, 1f32);
    let score_text_dimension = measure_text(score_text.as_str(), Some(*font), 26u16, 1f32);

    // Drawing Text Sections
    draw_text_ex(score_text.as_str(), screen_width()*0.5f32-(score_text_dimension.width*0.5f32), screen_height()*0.5f32-score_text_dimension.height-17f32, TextParams { font: *font, font_size: 26u16, font_scale: 1f32, font_scale_aspect: 1f32, color: WHITE });
    draw_text_ex("Press Space or Click to Unpause", screen_width()*0.5f32-(unpause_text_dimension.width*0.5f32), screen_height()*0.5f32-(unpause_text_dimension.height*0.5f32), TextParams { font: *font, font_size: 25u16, font_scale: 1f32, font_scale_aspect: 1f32, color: WHITE });

    GameState::Pause
}

// Creating a Function to Configurate the Window
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
    // Creating Game State Variable
    let mut game_state = GameState::Start;

    // Creating Player "Class" and Associated Variables
    let mut player: Player = Player::new(TILE_SIZE*4f32, (screen_height()/TILE_SIZE/2f32-1f32)*TILE_SIZE, Color{r: 255f32, g: 0f32, b: 0f32, a: 255f32});
    let mut score: f32 = 0f32;
    
    // Creating Input Vector
    let mut input = vec![1f32, 0f32];
    
    // Creating Frame Variables
    let mut frame_bg_rect = Rect{x: 0f32, y: 25f32, w: screen_width(), h: screen_height()-25f32};
    let mut minimum_frame_time: f32;
    let mut frame_time: f32;
    let mut time_to_sleep = 0f32;

    let font = load_ttf_font("./font/Roboto-Regular.ttf").await.unwrap();

    loop {
        // Making the Window all Black
        clear_background(BLACK);
        
        // Checking for Key Inputs and Changing the Associated Variable
        if (is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D)) && (input[0] != -1f32 && input[0] != 1f32){
            input = vec![1f32, 0f32];
        } else if (is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S)) && (input[1] != -1f32 && input[1] != 1f32){
            input = vec![0f32, 1f32];
        } else if (is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A)) && (input[0] != 1f32 && input[0] != -1f32){
            input = vec![-1f32, 0f32];
        }else if (is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W)) && (input[1] != 1f32 && input[1] != -1f32){
            input = vec![0f32, -1f32];
        }

        // Calling the Associated function with the Game State
        game_state = match game_state {
            GameState::Setup => setup(&mut input, &mut player, &mut score),
            GameState::Play => play(&mut player, &input, &mut score, &mut frame_bg_rect, &font),
            GameState::Start => start(&font),
            GameState::Gameover => gameover(score, &font),
            GameState::Pause => pause(score, &font),
        };

        // Capping the Frame Rate
        minimum_frame_time = 1. / 9.; // 9 FPS
        frame_time = get_frame_time()-time_to_sleep*0.001f32;
        time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));

        // Awaiting the Next Frame
        next_frame().await
    }
}