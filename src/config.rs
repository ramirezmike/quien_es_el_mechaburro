use bevy::prelude::*;

pub const MAX_NUMBER_OF_PLAYERS: isize = 8;

#[derive(Resource)]
pub struct GameConfiguration {
    pub bullet_distance: f32,
    pub burro_speed: f32,
    pub follow_burros: bool,
    pub invulnerability_cooldown: f32,
    pub down_cooldown: f32,
    pub max_camera_yaw: f32,
    pub max_camera_pitch: f32,
    pub max_camera_roll: f32,
    pub camera_shake: f32,
    pub hit_starting_size: f32,
    pub hit_speed: f32,
    pub hit_shrink_speed: f32,
    pub hit_min_spread_x: f32,
    pub hit_max_spread_x: f32,
    pub hit_min_spread_y: f32,
    pub hit_max_spread_y: f32,
    pub hit_min_spread_z: f32,
    pub hit_max_spread_z: f32,
}

impl Default for GameConfiguration {
    fn default() -> Self {
        GameConfiguration {
            bullet_distance: 1.0,
            burro_speed: 40.0,
            follow_burros: true,
            invulnerability_cooldown: 1.0,
            down_cooldown: 1.5,
            max_camera_yaw: 1.0,
            max_camera_pitch: 1.0,
            max_camera_roll: 1.0,
            camera_shake: 0.3,
            hit_starting_size: 0.2,
            hit_speed: 9.0,
            hit_shrink_speed: 0.1,
            hit_min_spread_x: 0.0,
            hit_max_spread_x: 1.0,
            hit_min_spread_y: 0.0,
            hit_max_spread_y: 1.0,
            hit_min_spread_z: 0.0,
            hit_max_spread_z: 1.0,
        }
    }
}
