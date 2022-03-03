use crate::{assets::GameAssets, audio, burro, cleanup, inspect, AppState};
use bevy::prelude::*;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletEvent>()
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(cleanup::<CleanupMarker>),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_bullet_events)
                    .with_system(handle_bullets),
            );
    }
}

pub struct BulletEvent {
    pub source: Entity,
    pub speed: f32,
    pub position: Vec3,
    pub direction: Vec3,
    pub time_to_live: f32,
    pub bullet_type: BulletType,
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Component)]
struct Bullet {
    time_to_live: f32,
    time_alive: f32,
    source: Entity,
    speed: f32,
    direction: Vec3,
    bullet_type: BulletType,
}

#[derive(Copy, Clone, PartialEq)]
pub enum BulletType {
    Candy,
    Laser,
}

impl Default for BulletType {
    fn default() -> Self {
        BulletType::Candy
    }
}

fn handle_bullet_events(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut bullet_reader: EventReader<BulletEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut audio: audio::GameAudio,
) {
    for bullet in bullet_reader.iter() {
        commands
            .spawn_bundle(match bullet.bullet_type {
                BulletType::Candy => PbrBundle {
                    mesh: game_assets.candy.mesh.clone(),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    transform: Transform::from_xyz(
                        bullet.position.x + bullet.direction.x,
                        bullet.position.y + 0.5,
                        bullet.position.z + bullet.direction.z,
                    ),
                    ..Default::default()
                },
                BulletType::Laser => PbrBundle {
                    mesh: game_assets.laser.mesh.clone(),
                    material: materials.add(Color::rgb(0.6, 0.0, 0.0).into()),
                    transform: {
                        let mut transform = Transform::from_xyz(
                            bullet.position.x + bullet.direction.x,
                            bullet.position.y + 0.5,
                            bullet.position.z + bullet.direction.z,
                        );
                        if bullet.direction.z != 0.0 {
                            transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.0));
                        }

                        transform
                    },
                    ..Default::default()
                },
            })
            .insert(Bullet {
                source: bullet.source,
                time_to_live: if bullet.bullet_type == BulletType::Laser {
                    bullet.time_to_live + 10.0
                } else {
                    bullet.time_to_live
                },
                time_alive: 0.0,
                speed: if bullet.bullet_type == BulletType::Laser {
                    bullet.speed + 2.0
                } else {
                    bullet.speed
                },
                direction: bullet.direction,
                bullet_type: bullet.bullet_type,
            })
            .insert(CleanupMarker);
        if bullet.bullet_type == BulletType::Laser {
            audio.play_sfx(&game_assets.laser_sfx);
        } else {
            audio.play_sfx(&game_assets.bloop_sfx);
        }
    }
}

fn handle_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform), Without<burro::Burro>>,
    mut burros: Query<(Entity, &Transform, &mut burro::Burro), Without<Bullet>>,
    inspector: Res<inspect::InspectorData>,
) {
    'bullets: for (entity, mut bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
        if bullet.bullet_type == BulletType::Candy {
            transform.rotate(Quat::from_rotation_y(2.0 * time.delta_seconds()));
            transform.rotate(Quat::from_rotation_x(1.75 * time.delta_seconds()));
            transform.rotate(Quat::from_rotation_z(0.75 * time.delta_seconds()));
        }

        bullet.time_alive += time.delta_seconds();

        if bullet.time_alive > bullet.time_to_live {
            // time to die
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let bullet_position = Vec2::new(transform.translation.x, transform.translation.z);
        'burros: for (burro_entity, burro_transform, mut burro) in burros.iter_mut() {
            if burro_entity == bullet.source {
                // don't shoot yourself
                continue;
            }

            let burro_position =
                Vec2::new(burro_transform.translation.x, burro_transform.translation.z);
            if bullet_position.distance(burro_position) <= inspector.bullet_distance {
                commands.entity(entity).despawn_recursive();
                burro.hit();

                continue 'bullets;
            }
        }
    }
}
