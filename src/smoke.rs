use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::{AppState};

pub struct SmokePlugin;
impl Plugin for SmokePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(handle_smokers)
                .with_system(handle_smokes)
        );
    }
}

#[derive(Component, Default)]
pub struct Smoker {
    pub cooldown: f32,
}

#[derive(Component)]
pub struct Smoke;

fn handle_smokes(
    mut commands: Commands,
    mut smokes: Query<(&mut Transform, &Handle<StandardMaterial>, &Parent), With<Smoke>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (mut transform, material, parent) in smokes.iter_mut() {
        transform.rotate(Quat::from_rotation_x(time.delta_seconds()));
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
        transform.scale *= 1.0 + (time.delta_seconds() * 0.2);

        let target = transform
            .translation
            .lerp(Vec3::Z, time.delta_seconds() * 0.3);
        if !target.is_nan() {
            transform.translation = target;
        }

        transform.translation.z += time.delta_seconds() * 5.0;

        let mut despawn_entity = true; // if the material doesn't exist, just despawn
        if let Some(material) = materials.get_mut(material) {
            let a = material.base_color.a();
            if a > 0.0 {
                despawn_entity = false;
                material.base_color.set_a(a - (time.delta_seconds() * 1.05));
            }
        }

        if despawn_entity {
            commands.entity(**parent).despawn_recursive();
        }
    }
}

fn handle_smokers(
    mut commands: Commands,
    mut smokers: Query<(&mut Smoker, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (mut smoker, transform) in smokers.iter_mut() {
        smoker.cooldown -= time.delta_seconds();
        smoker.cooldown = smoker.cooldown.clamp(-1.0, 3.0);
        if smoker.cooldown <= 0.0 {
            smoker.cooldown = 0.3;

            for _ in 0..6 {
                let inner_mesh_x = thread_rng().gen_range(-25..25) as f32 / 100.0;
                let inner_mesh_z = thread_rng().gen_range(-25..25) as f32 / 100.0;

                let color = Color::rgba(0.3, 0.3, 0.3, 0.7 + inner_mesh_x.abs());

                commands
                    .spawn_bundle(PbrBundle {
                        transform: *transform,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                                material: materials.add(color.into()),
                                transform: {
                                    let mut t = Transform::from_xyz(inner_mesh_x, 0.1, inner_mesh_z);
                                    t.rotate(Quat::from_rotation_x(inner_mesh_z));
                                    t.rotate(Quat::from_rotation_y(inner_mesh_x));
                                    t
                                },
                                visibility: Visibility { is_visible: true },
                                ..Default::default()
                            })
                            .insert(Smoke);
                    });
            }
        }
    }
}
