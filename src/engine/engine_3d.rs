use crate::engine::{control::keyboard::KeyboardController, rendering::{renderer::Renderer, renderer_3d::Renderer3D, texture_poll::TexturePool}};
pub struct Engine3D {
    pub running: bool,
    pub renderer: Renderer3D,
    pub texture_poll: TexturePool,
    pub kbcontroller: KeyboardController,
}

impl Engine3D {
    pub fn render(&mut self, delta_time: f32) {
        self.renderer.render(delta_time);
    }
    
    pub fn update(&mut self, _delta_time: f32) {

    }


    

}
