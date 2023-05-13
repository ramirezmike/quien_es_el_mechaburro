use crate::{assets::GameAssets, game_state, player, AppState};
use bevy::prelude::*;
use rand::Rng;

pub struct BurroPlugin;
impl Plugin for BurroPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component)]
pub struct Burro {
    pub burro_skin: game_state::BurroSkin,
    pub max_health: usize,
    pub health: usize,
    pub bullet_speed: f32,
    pub bullet_time_alive: f32,
    pub fire_cooldown: f32,
    pub invulnerability_cooldown: f32,
    pub is_visible: bool,
    pub is_mechaburro: bool,
    pub is_down: bool,
    pub down_cooldown: f32,
    pub random: f32,
}

impl Burro {
    pub fn new(burro_skin: game_state::BurroSkin) -> Self {
        let mut rng = rand::thread_rng();

        Burro {
            burro_skin,
            max_health: 3,
            health: 3,
            bullet_speed: 12.0,
            bullet_time_alive: 1.0,
            fire_cooldown: 0.0,
            invulnerability_cooldown: 0.0,
            is_visible: true,
            is_mechaburro: false,
            random: rng.gen_range(0.5..1.0),
            is_down: false,
            down_cooldown: 0.0,
        }
    }

    pub fn can_fire(&self) -> bool {
        self.fire_cooldown <= 0.0 && !self.is_invulnerable()
    }

    pub fn fire(&mut self) {
        self.fire_cooldown = 0.4;
    }

    pub fn hit(&mut self, down_cooldown: f32) {
        if !self.can_be_hit() {
            return;
        }

        if let Some(health) = self.health.checked_sub(1) {
            self.health = health;
        }
        self.is_down = true;
        self.down_cooldown = down_cooldown;
    }

    pub fn can_be_hit(&self) -> bool {
        !self.is_down && !self.is_invulnerable()
    }

    pub fn is_invulnerable(&self) -> bool {
        self.invulnerability_cooldown > 0.0
    }
}
