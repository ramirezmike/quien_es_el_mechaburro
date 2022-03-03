use crate::{game_state, AppState};
use bevy::prelude::*;

pub struct BurroPlugin;
impl Plugin for BurroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(handle_burros.label("handle_burros"))
                .with_system(handle_burro_death_events.after("handle_burros"))
                .with_system(handle_burro_flash_events),
        )
        .add_event::<BurroFlashEvent>()
        .add_event::<BurroDeathEvent>();
    }
}

pub struct BurroDeathEvent {
    pub entity: Entity,
    pub skin: game_state::BurroSkin,
}

struct BurroFlashEvent {
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
    pub random: f32,
}

impl Burro {
    pub fn new(burro_skin: game_state::BurroSkin) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Burro {
            burro_skin,
            max_health: 3,
            health: 3,
            bullet_speed: 6.0,
            bullet_time_alive: 3.0,
            fire_cooldown: 0.0,
            invulnerability_cooldown: 0.0,
            is_visible: true,
            is_mechaburro: false,
            random: rng.gen_range(0.5..1.0),
        }
    }

    pub fn can_fire(&self) -> bool {
        self.fire_cooldown <= 0.0 && !self.is_invulnerable()
    }

    pub fn fire(&mut self) {
        self.fire_cooldown = 0.4;
    }

    pub fn hit(&mut self) {
        if self.is_invulnerable() {
            return;
        }

        if let Some(health) = self.health.checked_sub(1) {
            self.health = health;
            self.invulnerability_cooldown = 3.0;
        }
    }

    pub fn full_health(&self) -> bool {
        self.health == self.max_health
    }

    pub fn almost_dead(&self) -> bool {
        self.health == 1
    }

    pub fn is_invulnerable(&self) -> bool {
        self.invulnerability_cooldown > 0.0
    }
}

fn handle_burro_flash_events(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut burros: Query<(&mut Burro, &Handle<StandardMaterial>)>,
    mut flash_event_reader: EventReader<BurroFlashEvent>,
) {
    for event in flash_event_reader.iter() {
        if let Ok((mut burro, material_handle)) = burros.get_mut(event.entity) {
            burro.is_visible = event.show;
            if let Some(mut material) = materials.get_mut(material_handle) {
                if event.show {
                    material.alpha_mode = AlphaMode::Opaque;
                    material.base_color.set_a(1.0);
                } else {
                    material.alpha_mode = AlphaMode::Blend;
                    material.base_color.set_a(0.6);
                }
            }
        }
    }
}

fn handle_burros(
    time: Res<Time>,
    mut assets_materials: ResMut<Assets<StandardMaterial>>,
    mut burros: Query<(Entity, &mut Burro)>,
    mut burro_death_event_writer: EventWriter<BurroDeathEvent>,
    mut flash_event_writer: EventWriter<BurroFlashEvent>,
) {
    for (entity, mut burro) in burros.iter_mut() {
        let current_sin = (time.seconds_since_startup() as f32 * (1.0 + burro.random) * 8.0).sin();

        // handling burro deaths
        if burro.health == 0 {
            burro_death_event_writer.send(BurroDeathEvent {
                entity,
                skin: burro.burro_skin,
            });
            continue;
        }

        // handling firing cool down
        burro.fire_cooldown -= if burro.is_mechaburro { 2.0 } else { 1.0 } * time.delta_seconds();
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

fn handle_burro_death_events(
    mut commands: Commands,
    mut burro_death_event_reader: EventReader<BurroDeathEvent>,
    mut game_state: ResMut<game_state::GameState>,
) {
    for death_event in burro_death_event_reader.iter() {
        commands.entity(death_event.entity).despawn_recursive();
        game_state.dead_burros.push(death_event.skin);
        // probably do a bunch of UI/animation stuff here and play sounds or something
    }
}
