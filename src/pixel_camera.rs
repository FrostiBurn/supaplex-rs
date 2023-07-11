use macroquad::prelude::*;
use macroquad::{
    miniquad::gl::GL_MULTISAMPLE,
    prelude::{Camera, Mat4},
    window::{screen_height, screen_width},
};

pub struct PixelCamera {
    pub position: Vec2,
    pub zoom: f32,
    pub speed: f32,
}

impl PixelCamera {
    pub fn new(position: Vec2, zoom: f32, speed: f32) -> Self {
        Self { position, zoom, speed }
    }

    pub fn move_camera_with_keys(
        &mut self,
        up: KeyCode,
        down: KeyCode,
        left: KeyCode,
        right: KeyCode,
    ) {
        let x = (is_key_down(right) as isize - is_key_down(left) as isize) as f32;
        let y = (is_key_down(down) as isize - is_key_down(up) as isize) as f32;

        let velocity = (vec2(x, y).normalize_or_zero() * get_frame_time()) * self.speed;

        self.position += velocity;
    }

    pub fn handle_zoom(&mut self) {
        if mouse_wheel().1 != 0.0 {
            self.zoom = (self.zoom + mouse_wheel().1.round()).max(1.0);
        }

        if is_key_down(KeyCode::LeftShift) {
            let zoom_delta =
            (is_key_pressed(KeyCode::Equal) as i8 - is_key_pressed(KeyCode::Minus) as i8) as f32;
            self.zoom = (self.zoom + zoom_delta).max(1.0);
        }
    }

    pub fn begin_pp(&mut self) {
        unsafe {
            miniquad::gl::glDisable(GL_MULTISAMPLE);
        }
    }

    pub fn end_pp(&self) {
        unsafe {
            miniquad::gl::glEnable(GL_MULTISAMPLE);
        }
    }
}

impl Camera for PixelCamera {
    fn matrix(&self) -> macroquad::prelude::Mat4 {
        let even_width = (screen_width() as i32 & 0x0fff_fffe) as f32;
        let even_height = (screen_height() as i32 & 0x0fff_fffe) as f32;

        let matw = 2.0 / even_width * self.zoom;
        let math = -2.0 / even_height * self.zoom;

        let matx = 2.0 / even_width * (-self.position.x * self.zoom).round();
        let maty = -2.0 / even_height * (-self.position.y * self.zoom).round();

        Mat4::from_cols(
            Vec4::new(matw, 0.0, 0.0, 0.0),
            Vec4::new(0.0, math, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(matx, maty, 0.0, 1.0),
        )
    }

    fn depth_enabled(&self) -> bool {
        false
    }

    fn render_pass(&self) -> Option<macroquad::miniquad::RenderPass> {
        None
    }

    fn viewport(&self) -> Option<(i32, i32, i32, i32)> {
        let even_width = screen_width() as i32 & 0x0fff_fffe;
        let even_height = screen_height() as i32 & 0x0fff_fffe;

        Some((0, 0, even_width, even_height))
    }
}
