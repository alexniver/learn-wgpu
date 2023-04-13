use glam::*;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};
pub struct Camera {
    pub eye: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_v_p_m(&self) -> Mat4 {
        let view = Mat4::look_to_rh(self.eye, self.forward, self.up);
        let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        proj * view
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(camera: &Camera) -> Self {
        Self {
            view_proj: camera.build_v_p_m().to_cols_array_2d(),
        }
    }

    pub fn update(&mut self, camera: &Camera) {
        self.view_proj = camera.build_v_p_m().to_cols_array_2d();
    }
}

pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            is_forward_pressed: Default::default(),
            is_backward_pressed: Default::default(),
            is_left_pressed: Default::default(),
            is_right_pressed: Default::default(),
        }
    }
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            ..Default::default()
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                let is_pressed = input.state == ElementState::Pressed;
                match input.virtual_keycode {
                    Some(VirtualKeyCode::W) => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    Some(VirtualKeyCode::S) => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    Some(VirtualKeyCode::D) => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    Some(VirtualKeyCode::A) => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.forward;
        let right = forward.cross(camera.up);

        if self.is_forward_pressed {
            camera.eye += forward * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward * self.speed;
        }

        if self.is_left_pressed {
            camera.eye -= right * self.speed;
        }
        if self.is_right_pressed {
            camera.eye += right * self.speed;
        }
    }
}
