use crate::{follow_text, game_state, player, AppState, smoke};
use bevy::prelude::*;
use rand::Rng;

pub struct BurroPlugin;
impl Plugin for BurroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(handle_burros.label("handle_burros"))
                .with_system(handle_burro_death_events.after("handle_burros"))
                .with_system(handle_burro_hit.after("move_player"))
                .with_system(handle_fallen_burros)
                .with_system(handle_burro_flash_events),
        )
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

    pub fn hit(&mut self) {
        if !self.can_be_hit() {
            return;
        }

        if let Some(health) = self.health.checked_sub(1) {
            self.health = health;
        }
        self.is_down = true;
        self.down_cooldown = 1.75;
    }

    pub fn can_be_hit(&self) -> bool {
        !self.is_down && !self.is_invulnerable()
    }

    pub fn is_invulnerable(&self) -> bool {
        self.invulnerability_cooldown > 0.0
    }
}

fn handle_burro_hit(
    mut commands: Commands,
    mut burro_hit_event_reader: EventReader<BurroHitEvent>,
    mut burros: Query<(Entity, &mut Burro, &mut Transform, &mut player::Player)>,
) {
    for event in burro_hit_event_reader.iter() {
        let mut rng = rand::thread_rng();
        if let Ok((entity, mut burro, mut transform, mut player)) = burros.get_mut(event.entity) {
            burro.hit();

            let random_z = rng.gen_range(0.0..6.2831);
            transform.rotation = Quat::from_rotation_x((3.0 * std::f32::consts::PI) / 2.0);
            transform.rotation *= Quat::from_rotation_z(random_z);

            player.velocity += event.velocity * 4.0;
            player.is_firing = false;

            if event.is_laser {
                commands.entity(entity).insert(smoke::Smoker::default());
            }
        }
    }
}

fn handle_fallen_burros(mut commands: Commands, mut burros: Query<(Entity, &mut Burro, &mut Transform)>, time: Res<Time>) {
    for (entity, mut burro, mut transform) in burros.iter_mut() {
        if !burro.is_down {
            continue;
        }

        burro.down_cooldown -= time.delta_seconds();
        burro.down_cooldown = burro.down_cooldown.clamp(-3.0, 10.0);
        if burro.down_cooldown < 0.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI * 2.0);
            burro.down_cooldown = 0.0;
            burro.is_down = false;
            burro.invulnerability_cooldown = 3.0;
            commands.entity(entity).remove::<smoke::Smoker>();
        }
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
                    material.base_color.set_a(0.4);
                }
            }
        }
    }
}

fn handle_burros(
    time: Res<Time>,
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
    burros: Query<Entity, With<Burro>>,
    mut game_state: ResMut<game_state::GameState>,
    follow_texts: Query<(Entity, &follow_text::FollowText)>,
) {
    for death_event in burro_death_event_reader.iter() {
        if let Ok(_) = burros.get(death_event.entity) {
            commands.entity(death_event.entity).despawn_recursive();
            for (text_entity, text) in follow_texts.iter() {
                if text.following == death_event.entity {
                    commands.entity(text_entity).despawn_recursive();
                }
            }
            if !game_state.dead_burros.contains(&death_event.skin) {
                game_state.dead_burros.push(death_event.skin);
            }
            // probably do a bunch of UI/animation stuff here and play sounds or something
        }
    }
}
