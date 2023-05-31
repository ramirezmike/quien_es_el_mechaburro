use crate::game_state;
use bevy::prelude::*;
use rand::Rng;

pub struct BurroPlugin;
impl Plugin for BurroPlugin {
    fn build(&self, app: &mut App) {

        app
        .add_event::<BurroFlashEvent>()
        .add_event::<BurroHitEvent>()
        .add_event::<BurroDeathEvent>();
    }
}

pub struct BurroHitEvent {
    pub entity: Entity,
    pub velocity: Vec3,
    pub is_laser: bool,
}

pub struct BurroDeathEvent {
    pub entity: Entity,
    pub skin: game_state::BurroSkin,
}

pub struct BurroFlashEvent {
    entity: Entity,
    show: bool,
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
    pub current_animation: Handle<AnimationClip>,
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
            current_animation: Handle::<AnimationClip>::default(),
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


pub fn handle_burros(
    time: Res<Time>,
    mut burros: Query<(Entity, &mut Burro)>,
    mut burro_death_event_writer: EventWriter<BurroDeathEvent>,
    mut flash_event_writer: EventWriter<BurroFlashEvent>,
    game_state: Res<game_state::GameState>,
) {
    for (entity, mut burro) in burros.iter_mut() {
        let current_sin = (time.elapsed_seconds() * (1.0 + burro.random) * 8.0).sin();

        // handling burro deaths
        if burro.health == 0 {
            burro_death_event_writer.send(BurroDeathEvent {
                entity,
                skin: burro.burro_skin,
            });
            continue;
        }

        // handling firing cool down
        burro.fire_cooldown -= 
            if burro.is_mechaburro { 
                2.0 * game_state.difficulty
            } else { 
                1.0 
            } 
            * time.delta_seconds();

        burro.fire_cooldown = burro.fire_cooldown.clamp(-10.0, 3.0);

        // handling invulnerability
        let is_invulnerable = burro.is_invulnerable();
        burro.invulnerability_cooldown -= time.delta_seconds();
        burro.invulnerability_cooldown = burro.invulnerability_cooldown.clamp(-10.0, 3.0);

        if is_invulnerable && !burro.is_invulnerable() {
            flash_event_writer.send(BurroFlashEvent { entity, show: true });
        } else if burro.is_invulnerable() {
            if current_sin > 0.0 && !burro.is_visible {
                flash_event_writer.send(BurroFlashEvent { entity, show: true });
            } else if current_sin < 0.0 && burro.is_visible {
                flash_event_writer.send(BurroFlashEvent {
                    entity,
                    show: false,
                });
            }
        }
    }
}
