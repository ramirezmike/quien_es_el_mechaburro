use bevy::prelude::*;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_system(update_camera);
    }
}

#[derive(Component)]
pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

#[derive(Default)]
pub struct CameraSettings {
    look_at: Vec3,
    height: f32,
    orbit: bool,
    speed: f32,
    distance: f32,
    target_distance: f32,
}

impl CameraSettings {
    pub fn set_camera(
        &mut self,
        height: f32,
        look_at: Vec3,
        speed: f32,
        orbit: bool,
        distance: f32,
        target_distance: f32,
    ) {
        self.height = height;
        self.look_at = look_at;
        self.speed = speed;
        self.orbit = orbit;
        self.distance = distance;
        self.target_distance = target_distance;
    }
}

fn update_camera(
    mut cameras: Query<&mut Transform, With<PanOrbitCamera>>,
    camera_settings: ResMut<CameraSettings>,
    time: Res<Time>,
) {
    let mut c = camera_settings;

    if (c.distance - c.target_distance).abs() > 0.1 {
        if c.distance > c.target_distance {
            c.distance -= time.delta_seconds() * 8.0;
        } else {
            c.distance += time.delta_seconds() * 8.0;
        }
    }

    for mut transform in cameras.iter_mut() {
        if transform.translation.is_nan() {
            transform.translation = Vec3::new(0.1, 0.1, 0.1);
        }
        let height_difference = transform.translation.y - c.height;
        if height_difference.abs() > 0.1 {
            transform.translation.y +=
                (c.height - transform.translation.y) * c.speed * time.delta_seconds();
            //          if height_difference > 0.0 {
            //              transform.translation.y -=
            //                  (c.height - transform.translation.y)
            //                 * c.speed
            //                 * time.delta_seconds();
            //          } else {
            //              transform.translation.y +=
            //                  (c.height - transform.translation.y)
            //                 * c.speed
            //                 * time.delta_seconds();
            //          }
        }

        if c.orbit {
            let yaw = Quat::from_rotation_y(time.delta_seconds() as f32 * 0.3);
            transform.rotation *= yaw; // rotate around global y axis
        } else {
            transform.rotation = Quat::from_rotation_y((3.0 * std::f32::consts::PI) / 2.0);
        }

        let rot_matrix = Mat3::from_quat(transform.rotation);
        let new_translation = c.look_at + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, c.distance));

        transform.translation.x = new_translation.x;
        transform.translation.z = new_translation.z;

        transform.look_at(c.look_at, Vec3::Y);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(0.1, 0.1, 0.1);

    let radius = translation.length();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .with_children(|parent| {
            const HALF_SIZE: f32 = 100.0;
            parent.spawn_bundle(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    // Configure the projection to better fit the scene
                    illuminance: 10000.0,
                    shadow_projection: OrthographicProjection {
                        left: -HALF_SIZE,
                        right: HALF_SIZE,
                        bottom: -HALF_SIZE,
                        top: HALF_SIZE,
                        near: -10.0 * HALF_SIZE,
                        far: 10.0 * HALF_SIZE,
                        ..Default::default()
                    },
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: Transform {
                    rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        });
}

pub fn despawn_camera(mut commands: Commands, cameras: Query<Entity, With<PanOrbitCamera>>) {
    for entity in cameras.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
