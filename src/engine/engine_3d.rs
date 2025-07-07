use crate::engine::rendering::{renderer::Renderer, renderer_3d::Renderer3D};
pub struct Engine3D {
    pub renderer: Renderer3D
}

impl Engine3D {
    pub fn render(&mut self, delta_time: f32) {
        self.renderer.render(delta_time);
    }
    
    pub fn update(&mut self, delta_time: f32) {

    } 
}