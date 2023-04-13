use glam::{Mat4, Vec3};

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(translation: Vec3, scale: Vec3, rotation: Vec3) -> Self {
        Self {
            translation,
            scale,
            rotation,
        }
    }

    pub fn to_mat4(&self) -> Mat4 {
        let translation = Mat4::from_translation(self.translation);
        let rotation = Mat4::from_rotation_x(self.rotation.x)
            * Mat4::from_rotation_y(self.rotation.y)
            * Mat4::from_rotation_z(self.rotation.z);
        let scale = Mat4::from_scale(self.scale);
        scale * rotation * translation
    }
}
