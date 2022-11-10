#[macro_use]

mod tiles;
mod bug_state;
mod entity;
mod gravity;
mod macros;
mod murphy;
mod world;

use macroquad::prelude::*;
use world::World;

const UPDATE_TIME: f32 = 0.5;

struct Game {
    camera: Camera2D,
    scroll_zoom: f32,
    camera_speed: f32,
    level: World,
    tiles: Texture2D,
}

impl Game {
    async fn new() -> Self {
        let world = World::new_from(vec![
            vec![5, 5, 5, 5, 5, 5],
            vec![5, 7, 6, 3, 13, 5],
            vec![5, 3, 3, 3, 3, 5],
            vec![5, 13, 3, 3, 3, 5],
            vec![5, 4, 0, 6, 0, 5],
            vec![5, 5, 5, 5, 5, 5],
        ]);

        let tiles = load_texture("assets/moving3.png").await.unwrap();
        tiles.set_filter(FilterMode::Nearest);

        //build_textures_atlas();

        Self {
            camera: Camera2D {
                zoom: vec2(1., screen_width() / screen_height()),
                ..Default::default()
            },
            scroll_zoom: 1.0,
            camera_speed: 0.5,
            level: world,
            tiles,
        }
    }
}

#[macroquad::main("Supaplex")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        update(&mut game);

        clear_background(BLACK);

        set_camera(&game.camera);
        render(&game);

        set_default_camera();
        render_ui(&game);

        next_frame().await
    }
}

fn update(game: &mut Game) {
    if is_key_down(KeyCode::W) {
        game.camera.target.y -= game.camera_speed * get_frame_time() * 69.0;
    }
    if is_key_down(KeyCode::A) {
        game.camera.target.x -= game.camera_speed * get_frame_time() * 69.0;
    }
    if is_key_down(KeyCode::S) {
        game.camera.target.y += game.camera_speed * get_frame_time() * 69.0;
    }
    if is_key_down(KeyCode::D) {
        game.camera.target.x += game.camera_speed * get_frame_time() * 69.0;
    }

    if mouse_wheel().1 != 0.0 {
    	let zoom_value = if mouse_wheel().1 > 0.0 {
    	    1.0
    	} else {
    	    -1.0
    	};
    	
        game.scroll_zoom += zoom_value;
        game.camera.zoom.x =
            0.5 * (-game.scroll_zoom + (4.0 + (game.scroll_zoom * game.scroll_zoom)).sqrt());
        game.camera.zoom.y = -game.camera.zoom.x * (screen_width() / screen_height())
    }

    game.level.update();
}

fn render(game: &Game) {
    game.level.draw(game.tiles);
}

fn render_ui(game: &Game) {
    draw_text(&*format!("Fps: {}", get_fps()), 30.0, 30.0, 30.0, WHITE);
    draw_text(
        &*format!("mouse pos: {:?}", mouse_position()),
        30.0,
        60.0,
        30.0,
        WHITE,
    );
    draw_text(
        &*format!("touches: {:?}", touches()),
        30.0,
        90.0,
        30.0,
        WHITE,
    );
    /*draw_text(
        &*format!("cam zoom: {}", game.camera.zoom),
        30.0,
        60.0,
        30.0,
        WHITE,
    );
    draw_text(
        &*format!("cam pos: {}", game.camera.target),
        30.0,
        90.0,
        30.0,
        WHITE,
    );*/
    /*draw_text(
        &*format!("update timer: {}", game.time_since_last_update),
        30.0,
        120.0,
        30.0,
        WHITE,
    );*/
}
