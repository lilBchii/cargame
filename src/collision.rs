use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    height: f32,
    width: f32,
}

impl Hitbox {
    
    pub fn from(x: f32, y: f32) -> Self 
    {
        Self {height: x, width: y}
    }
}
