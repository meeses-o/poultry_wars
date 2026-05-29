// src/utils/virtual_screen.rs
use macroquad::prelude::*;

pub struct VirtualScreen {
    camera: Camera2D,
}

impl VirtualScreen {
    pub fn new() -> Self {
        Self {
            camera: Camera2D {
                zoom: vec2(1.0 / 640.0, -1.0 / 480.0),
                target: vec2(640.0, 480.0),
                ..Default::default()
            },
        }
    }

    /// Call this at the very beginning of your loop in each screen
    pub fn begin_frame(&self) {
        set_camera(&self.camera);
    }

    /// Call this at the very end of your loop in each screen, before next_frame()
    pub fn end_frame(&self) {
        set_default_camera();
    }

    /// Replaces macroquad's default mouse_position()
    pub fn mouse_position(&self) -> (f32, f32) {
        let (raw_x, raw_y) = macroquad::input::mouse_position();
        let virtual_mouse = self.camera.screen_to_world(vec2(raw_x, raw_y));
        (virtual_mouse.x, virtual_mouse.y)
    }
}