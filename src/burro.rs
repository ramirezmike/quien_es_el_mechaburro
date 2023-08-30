use crate::{assets, audio, config, game_state, smoke, AppState, IngameState};
use bevy::prelude::*;
use bevy_toon_shader::ToonShaderMaterial;
use rand::Rng;

pub struct BurroPlugin;
impl Plugin for BurroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_burros, 
             handle_burro_death_events.run_if(|g: Res<game_state::GameState>| !g.is_game_over()), 
             handle_burro_hit)
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (handle_fallen_burros, handle_burro_flash_events).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            squish_burros
                .run_if(in_state(AppState::InGame).or_else(in_state(AppState::MechaPicker))),
        )
        .add_event::<BurroFlashEvent>()
        .add_event::<BurroHitEvent>()
        .add_event::<BurroDeathEvent>();
    }
}

#[derive(Event)]
pub struct BurroHitEvent {
    pub entity: Entity,
    pub velocity: Vec3,
    pub is_laser: bool,
}

#[derive(Event)]
pub struct BurroDeathEvent {
    pub entity: Entity,
    pub selected_burro: usize,
}

#[derive(Event)]
pub struct BurroFlashEvent {
    entity: Entity,
    show: bool,
}

#[derive(Component)]
pub struct Burro {
    pub selected_burro: usize,
    pub health: usize,
    pub bullet_speed: f32,
    pub bullet_time_alive: f32,
    pub fire_cooldown: f32,
    pub invulnerability_cooldown: f32,
    pub is_visible: bool,
    pub is_mechaburro: bool,
    pub is_down: bool,
    pub down_cooldown: f32,
    pub speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
    pub random: f32,
    pub current_animation: Handle<AnimationClip>,
}

impl Burro {
    pub fn new(selected_burro: usize) -> Self {
        let mut rng = rand::thread_rng();

        Burro {
            selected_burro,
            health: 3,
            bullet_speed: 12.0,
            bullet_time_alive: 1.0,
            fire_cooldown: 0.0,
            invulnerability_cooldown: 0.0,
            is_visible: true,
            is_mechaburro: false,
            speed: 60.0,
            friction: 0.01,
            velocity: Vec3::ZERO,
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

#[derive(Component)]
pub struct BurroMeshMarker {
    pub parent: Option<Entity>,
}

fn squish_burros(time: Res<Time>, mut burros: Query<&mut Transform, With<Burro>>) {
    for mut transform in burros.iter_mut() {
        // make the burros all squishy like
        if transform.scale.x != 1.0 || transform.scale.y != 1.0 {
            let new_scale = transform
                .scale
                .lerp(Vec3::new(1.0, 1.0, 1.0), time.delta_seconds() * 4.0);
            if new_scale.is_nan() || transform.scale.distance(new_scale) < 0.0001 {
                transform.scale = Vec3::new(1.0, 1.0, 1.0);
            } else {
                transform.scale = new_scale;
            }
        }
    }
}

fn handle_burro_hit(
    mut commands: Commands,
    mut burro_hit_event_reader: EventReader<BurroHitEvent>,
    mut burros: Query<(Entity, &mut Burro, &mut Transform)>,
    mut audio: audio::GameAudio,
    game_config: Res<config::GameConfiguration>,
    game_assets: Res<assets::GameAssets>,
) {
    for event in burro_hit_event_reader.iter() {
        let mut rng = rand::thread_rng();
        if let Ok((entity, mut burro, mut transform)) = burros.get_mut(event.entity) {
            burro.hit(game_config.down_cooldown);

            let random_z = rng.gen_range(0.0..std::f32::consts::TAU);
            transform.rotation = Quat::from_rotation_x((3.0 * std::f32::consts::PI) / 2.0);
            transform.rotation *= Quat::from_rotation_z(random_z);

            burro.velocity += event.velocity * 4.0;

            if event.is_laser {
                audio.play_sfx(&game_assets.laser_hit_sfx);
                audio.play_sfx(&game_assets.smoke_sfx);
                commands.entity(entity).insert(smoke::Smoker::default());
            } else {
                audio.play_sfx(&game_assets.candy_hit_sfx);
            }
        }
    }
}

fn handle_burros(
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
                selected_burro: burro.selected_burro,
            });
            continue;
        }

        // handling firing cool down
        burro.fire_cooldown -= if burro.is_mechaburro {
            2.0 * game_state.difficulty
        } else {
            1.0
        } * time.delta_seconds();

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
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    burros: Query<Entity, With<Burro>>,
    mut audio: audio::GameAudio,
    mut game_state: ResMut<game_state::GameState>,
    game_assets: Res<assets::GameAssets>,
) {
    for death_event in burro_death_event_reader.iter() {
        let number_of_burros = game_state.burros.iter().len();
        if burros.get(death_event.entity).is_ok() {
            commands.entity(death_event.entity).despawn_recursive();

            if !game_state.dead_burros.contains(&death_event.selected_burro) {
                game_state.dead_burros.push(death_event.selected_burro);
            }

            audio.play_sfx(&game_assets.eliminated_sfx);

            let number_of_dead_burros = game_state.dead_burros.iter().len();
            if number_of_dead_burros >= number_of_burros - 1 {
                next_ingame_state.set(IngameState::ScoreDisplay);
            }
        }
    }
}

fn handle_fallen_burros(
    mut commands: Commands,
    mut burros: Query<(Entity, &mut Burro, &mut Transform)>,
    time: Res<Time>,
    game_config: Res<config::GameConfiguration>,
) {
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
            burro.invulnerability_cooldown = game_config.invulnerability_cooldown;
            commands.entity(entity).remove::<smoke::Smoker>();
        }
    }
}

fn handle_burro_flash_events(
    mut materials: ResMut<Assets<ToonShaderMaterial>>,
    mut burros: Query<(&mut Burro, &Handle<ToonShaderMaterial>)>,
    mut flash_event_reader: EventReader<BurroFlashEvent>,
) {
    for event in flash_event_reader.iter() {
        if let Ok((mut burro, material_handle)) = burros.get_mut(event.entity) {
            burro.is_visible = event.show;
            if let Some(material) = materials.get_mut(material_handle) {
                if event.show {
                    //                    material.alpha_mode = AlphaMode::Opaque;
                    material.color.set_a(1.0);
                } else {
                    //                    material.alpha_mode = AlphaMode::Blend;
                    material.color.set_a(0.4);
                }
            }
        }
    }
}
