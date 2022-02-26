use bevy::prelude::*;

#[derive(Component)]
pub struct Burro {
    pub bullet_speed: f32,
    pub bullet_time_alive: f32,
}

impl Default for Burro {
    fn default() -> Self {
        Burro {
            bullet_speed: 6.0,
            bullet_time_alive: 3.0,
        }
    }
}
