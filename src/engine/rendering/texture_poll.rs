use std::collections::HashMap;
use crate::engine::rendering::texture::Texture;


pub struct TexturePool {
    textures: HashMap<String, Texture>,
}

impl TexturePool {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, texture: Texture) {
        self.textures.insert(String::from(name), texture);
    }

    pub fn reg_from_path(&mut self, name: &str, texture: &str) {
        println!("Loading texture '{}' from path '{}'", name, texture);
        self.textures.insert(
            String::from(name), 
            Texture::load(texture).unwrap_or_else(|e| panic!("❌ Can't load texture {}: {}", texture, e))
        );
    }
    

    pub fn get(&self, name: &str) -> Option<&Texture> {

        self.textures.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.textures.contains_key(name)
    }

    pub fn get_or_panic(&self, name: &str) -> &Texture {
        self.textures
            .get(name)
            .unwrap_or_else(|| panic!("❌ Textura '{}' no encontrada en el pool", name))
    }
}