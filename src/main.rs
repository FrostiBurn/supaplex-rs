mod grid;
mod tile_data;
mod level;
mod pixel_camera;
mod level_loader;

use level::Level;
use level_loader::load_level_dat;
use macroquad::{prelude::*, miniquad::gl::GL_MULTISAMPLE};
use pixel_camera::PixelCamera;


struct Game {
    camera: PixelCamera,
    freecam: Option<Vec2>,
    current_level: Level,
    level_set: Vec<Level>,
    selected_level: usize,
    tiles: Texture2D,
    text_params: TextParams,
    debug_info: bool,
    fullscreen: bool,
}

impl Game {
    async fn new() -> Self {
        //let level = Level::new(8.0, 60, 24, LEVEL_EASY_DEAL.to_vec());

        let levels = load_level_dat("LEVELS.DAT").unwrap();

        let tiles = load_texture("assets/moving2.png").await.unwrap();
        tiles.set_filter(FilterMode::Nearest);

        let font = load_ttf_font("assets/FiraSans-Medium.ttf").await.unwrap();

        let text_params = TextParams {
            font,
            font_size: 16,
            ..Default::default()
        };

        let camera = PixelCamera::new(Vec2::ZERO, 1.0, 256.0);

        Self {
            camera,
            freecam: None,
            current_level: levels[0].clone(),
            level_set: levels,
            selected_level: 0,
            tiles,
            text_params,
            debug_info: false,
            fullscreen: false
        }
    }
}

#[macroquad::main("Supaplex")]
async fn main() {
    unsafe {
        miniquad::gl::glDisable(GL_MULTISAMPLE);
    }
    let mut game = Game::new().await;

    loop {
        update(&mut game);

        clear_background(BLACK);

        render(&mut game);
        set_default_camera();

        game.current_level.draw_info(game.text_params);
        if game.debug_info {
            render_ui(&game);
        }

        next_frame().await
    }
}

fn update(game: &mut Game) {
    if let Some(pos) = &mut game.freecam {
        game.camera.move_camera_with_keys(KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D);
        *pos = game.camera.position;
    }
    game.camera.handle_zoom();

    let mut speed =
        (is_key_pressed(KeyCode::Equal) as i8 - is_key_pressed(KeyCode::Minus) as i8) as f32;
    if is_key_down(KeyCode::LeftShift) {
        speed *= 0.1;
    }

    game.current_level.data.speed += speed;

    if is_key_pressed(KeyCode::R) {
        game.current_level = game.level_set[game.selected_level].clone();
    }
    if is_key_pressed(KeyCode::P) {
        if game.freecam.is_none() {
            game.freecam = Some(game.camera.position);
        } else {
            game.freecam = None;
        }
    }

    if is_key_pressed(KeyCode::Comma) {
        game.selected_level -= 1;
        game.current_level = game.level_set[game.selected_level].clone();
    }
    if is_key_pressed(KeyCode::Period) {
        if game.selected_level < game.level_set.len() {
            game.selected_level += 1;
        } else {
            game.selected_level = 0;
        }
        game.current_level = game.level_set[game.selected_level].clone();
    }

    if is_key_pressed(KeyCode::T) {
        game.debug_info = !game.debug_info;
    }

    if is_key_pressed(KeyCode::F11) {
        game.fullscreen = !game.fullscreen;
        macroquad::window::set_fullscreen(game.fullscreen);
    }

    game.current_level.update();
}

fn render(game: &mut Game) {
    if game.freecam.is_none() {
        if let Some(camera_target) = game.current_level.data.camera_target {
            game.camera.position = camera_target;
        }
    }
    //game.camera.begin_pp();
    set_camera(&game.camera);
    game.current_level.draw(game.tiles);
    //game.camera.end_pp();
    if game.debug_info {
        game.current_level.draw_text();
    }
}

fn render_ui(game: &Game) {
    draw_text_ex(&format!("Fps: {}", get_fps()), 10.0, 60.0, game.text_params);
    draw_text_ex(
        &format!("position: {:?}", game.camera.position),
        10.0,
        80.0,
        game.text_params,
    );
    draw_text_ex(
        &format!("zoom: {:?}", game.camera.zoom),
        10.0,
        100.0,
        game.text_params,
    );
    draw_text_ex(
        &format!("speed: {}", game.current_level.data.speed),
        10.0,
        120.0,
        game.text_params,
    );
    draw_text_ex(
        &format!("update duration: {} ns", game.current_level.data.update_duration.as_nanos()),
        10.0,
        140.0,
        game.text_params,
    );
}