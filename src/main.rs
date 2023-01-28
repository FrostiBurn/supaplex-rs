#[macro_use]

mod world;
mod grid;
mod level;
mod stupid;
mod tile;

use level::components::move_component::MoveComponent;
use macroquad::prelude::*;
use stupid::LEVEL_EASY_DEAL;
use world::World;

struct Game {
    camera: Camera2D,
    zoom_value_index: usize,
    zoom_values: [f32; 8],
    camera_speed: f32,
    world: World,
    tiles: Texture2D,
    text_params: TextParams,
}

impl Game {
    async fn new() -> Self {
        let world = World::new_from(60, LEVEL_EASY_DEAL.to_vec());

        //let world = World::new_from(vec![vec![13; 26]; 62]);

        let tiles = load_texture("assets/moving2.png").await.unwrap();
        tiles.set_filter(FilterMode::Nearest);

        let font = load_ttf_font("assets/FiraSans-Medium.ttf").await.unwrap();

        let text_params = TextParams {
            font,
            font_size: 16,
            ..Default::default()
        };

        Self {
            camera: Camera2D {
                zoom: vec2(1., screen_width() / screen_height()),
                ..Default::default()
            },
            zoom_value_index: 4,
            zoom_values: [2.0, 1.0, 0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625],
            camera_speed: 1.0,
            world,
            tiles,
            text_params,
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
    if is_key_pressed(KeyCode::W) {
        game.camera.target.y -= game.camera_speed; // * get_frame_time() * 69.0;
    }
    if is_key_pressed(KeyCode::A) {
        game.camera.target.x -= game.camera_speed; // * get_frame_time() * 69.0;
    }
    if is_key_pressed(KeyCode::S) {
        game.camera.target.y += game.camera_speed; // * get_frame_time() * 69.0;
    }
    if is_key_pressed(KeyCode::D) {
        game.camera.target.x += game.camera_speed; // * get_frame_time() * 69.0;
    }

    game.world.ups +=
        4 * (is_key_pressed(KeyCode::Equal) as u8 - is_key_pressed(KeyCode::Minus) as u8);

    if is_key_pressed(KeyCode::R) {
        game.world = World::new_from(60, LEVEL_EASY_DEAL.to_vec());
    }

    if is_key_pressed(KeyCode::F) {}

    if mouse_wheel().1 != 0.0 {
        // fix windows having decimal zoom capabilities
        if mouse_wheel().1 > 0.0 {
            if game.zoom_value_index < 7 {
                game.zoom_value_index += 1;
            }
        } else if game.zoom_value_index != 0 {
            game.zoom_value_index -= 1;
        }

        game.camera.zoom.x = game.zoom_values[game.zoom_value_index];
        game.camera.zoom.y = -game.camera.zoom.x * (screen_width() / screen_height());
        /*game.camera.zoom.x =
            0.5 * (-game.scroll_zoom + (4.0 + (game.scroll_zoom * game.scroll_zoom)).sqrt());
        game.camera.zoom.y = -game.camera.zoom.x * (screen_width() / screen_height())*/
    }

    game.world.update();
}

fn render(game: &Game) {
    game.world.draw(game.tiles);
}

fn render_ui(game: &Game) {
    draw_text_ex(&format!("Fps: {}", get_fps()), 10.0, 20.0, game.text_params);
    draw_text_ex(
        &format!("camera target: {:?}", game.camera.target),
        10.0,
        40.0,
        game.text_params,
    );
    draw_text_ex(
        &format!("camera zoom: {:?}", game.camera.zoom),
        10.0,
        60.0,
        game.text_params,
    );
    draw_text_ex(
        &format!("ups: {}", game.world.ups),
        10.0,
        80.0,
        game.text_params,
    );
    /*draw_text(
        &*format!("update timer: {}", game.time_since_last_update),
        30.0,
        120.0,
        30.0,
        WHITE,
    );*/
}

pub trait TupleExt {
    fn trans(&self, trans: (i16, i16)) -> Self;
}

impl TupleExt for (usize, usize) {
    fn trans(&self, trans: (i16, i16)) -> Self {
        (
            (self.0 as i16 + trans.0) as usize,
            (self.1 as i16 + trans.1) as usize,
        )
    }
}

pub trait FTupleExt {
    fn get_offset(&self, offset: u8, mc: &MoveComponent) -> (f32, f32);
}

impl FTupleExt for (f32, f32) {
    fn get_offset(&self, offset: u8, mc: &MoveComponent) -> (f32, f32) {
        match mc {
            MoveComponent::Stationary => (self.0, self.1),
            MoveComponent::Up => (self.0, self.1 + offset as f32 / 16.0),
            MoveComponent::Down => (self.0, self.1 - offset as f32 / 16.0),
            MoveComponent::Left => (self.0 + offset as f32 / 16.0, self.1),
            MoveComponent::Right => (self.0 - offset as f32 / 16.0, self.1),
        }
    }
}
