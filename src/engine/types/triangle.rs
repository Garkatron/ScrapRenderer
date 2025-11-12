use crate::engine::types::vector::Vec3;

#[derive(Clone, PartialEq)]
pub struct Triangle {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub light_color: u32,
    pub uv: [Vec3; 3],
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Self {
            v1,
            v2,
            v3,
            light_color: 0,
            uv: [Vec3::zeros(); 3],
        }
    }

    pub fn set_uv(mut self, uv: [Vec3; 3]) -> Self {
        self.uv = uv;
        self
    }

    pub fn set_light_color(mut self, light_color: u32) -> Self {
        self.light_color = light_color;
        self
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            v1: Vec3::zeros(),
            v2: Vec3::zeros(),
            v3: Vec3::zeros(),
            light_color: 0,
            uv: [Vec3::zeros(); 3],
        }
    }
}
