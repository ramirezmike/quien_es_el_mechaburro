use crate::assets::GameAssets;
use bevy::prelude::*;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletEvent>()
            .add_system(handle_bullet_events)
            .add_system(handle_bullets);
    }
}

pub struct BulletEvent {
    pub position: Vec3,
    pub direction: Vec3,
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Component)]
struct Bullet {
    speed: f32,
    direction: Vec3,
}

fn handle_bullet_events(
    mut commands: Commands,
    mut bullet_reader: EventReader<BulletEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    game_assets: Res<GameAssets>,
) {
    for bullet in bullet_reader.iter() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_xyz(
                    bullet.position.x,
                    bullet.position.y,
                    bullet.position.z,
                ),
                ..Default::default()
            })
            .insert(Bullet {
                speed: 6.0,
                direction: bullet.direction,
            })
            .insert(CleanupMarker);
    }
}

fn handle_bullets(
    //    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(&mut Bullet, &mut Transform)>,
) {
    for (bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}
