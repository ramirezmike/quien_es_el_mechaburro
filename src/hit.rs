use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::{inspect};

pub struct HitPlugin;
impl Plugin for HitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_create_hit_event)
            .add_event::<CreateHitEvent>()
            .add_system(animate_hit);
    }
}

#[derive(Component)]
pub struct Hit {
    pub move_toward: Vec3,
}

pub struct CreateHitEvent {
    pub position: Vec3,
    pub is_candy: bool,
}

pub fn animate_hit(
    mut commands: Commands,
    mut hits: Query<(&Hit, &mut Transform, &Handle<StandardMaterial>, &Parent)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    inspector_data: Res<inspect::InspectorData>,
) {
    for (hit, mut transform, material, parent) in hits.iter_mut() {
        transform.rotate(Quat::from_rotation_x(time.delta_seconds()));
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
        transform.scale *= 1.0 - (time.delta_seconds() * inspector_data.hit_shrink_speed);

        let target = transform
            .translation
            .lerp(hit.move_toward, time.delta_seconds() * inspector_data.hit_speed);
        if !target.is_nan() {
            transform.translation = target;
        }

        //transform.translation.y += time.delta_seconds() * 0.5;

        let mut despawn_entity = true; // if the material doesn't exist, just despawn
        if let Some(material) = materials.get_mut(material) {
            let a = material.base_color.a();
            if a > 0.0 {
                despawn_entity = false;
                material.base_color.set_a(a - (time.delta_seconds() * 1.25));
            }
        }

        if despawn_entity {
            commands.entity(**parent).despawn_recursive();
        }
    }
}

pub fn handle_create_hit_event(
    mut commands: Commands,
    mut create_hit_event_reader: EventReader<CreateHitEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    inspector_data: Res<inspect::InspectorData>,
) {
    for event in create_hit_event_reader.iter() {
        let position = event.position;

        let transform =
            Transform::from_xyz(position.x as f32, position.y as f32, position.z as f32);

        for _ in 0..6 {
            let inner_mesh_x = thread_rng().gen_range(-25..25) as f32 / 100.0;
            let inner_mesh_z = thread_rng().gen_range(-25..25) as f32 / 100.0;

            let color = if event.is_candy {
                Color::rgba(0.3, 0.5, 0.3, 0.7 + inner_mesh_x.abs())
            } else {
                Color::rgba(0.6, 0.0, 0.0, 0.7 + inner_mesh_x.abs())
            };

            let move_toward_x = thread_rng().gen_range(inspector_data.hit_min_spread_x..inspector_data.hit_max_spread_x) as f32;
            let move_toward_y = thread_rng().gen_range(inspector_data.hit_min_spread_y..inspector_data.hit_max_spread_y) as f32;
            let move_toward_z = thread_rng().gen_range(inspector_data.hit_min_spread_z..inspector_data.hit_max_spread_z) as f32;
            let move_toward = Vec3::new(move_toward_x, move_toward_y, move_toward_z);

            commands
                .spawn_bundle(PbrBundle {
                    transform,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: inspector_data.hit_starting_size })),
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
                        .insert(Hit { move_toward });
                });
        }
    }
}
