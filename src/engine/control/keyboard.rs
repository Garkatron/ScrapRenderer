use minifb::{Key, Window, KeyRepeat};

pub struct KeyboardController<'a> {
    window: &'a Window,
}

impl<'a> KeyboardController<'a> {
    /// Crea una nueva instancia de KeyboardController asociada a una ventana minifb.
    pub fn new(window: &'a Window) -> Self {
        KeyboardController { window }
    }

    /// Devuelve un vector con las teclas que están actualmente presionadas.
    pub fn get_pressed_keys(&self) -> Vec<Key> {
        self.window.get_keys()
    }

    /// Devuelve un vector con las teclas que fueron pulsadas por primera vez en este frame.
    /// Usa KeyRepeat::No para evitar repeticiones automáticas.
    pub fn get_newly_pressed_keys(&self) -> Vec<Key> {
        self.window.get_keys_pressed(KeyRepeat::No)
    }

    /// Verifica si una tecla específica está actualmente presionada.
    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    /// Verifica si una tecla específica fue pulsada por primera vez en este frame.
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.get_newly_pressed_keys().contains(&key)
    }
}