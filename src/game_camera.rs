use crate::inspect;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::PerspectiveProjection;
use noise::{NoiseFn, Perlin};

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Perlin::new())
            .add_system(handle_camera_shake)
            .add_system(pan_orbit_camera);
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

fn handle_camera_shake(
    time: Res<Time>,
    perlin: Res<Perlin>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
    inspector_data: Res<inspect::InspectorData>,
) {
    let delta_seconds = time.delta_seconds() as f64;
    let perlin_value_1 = perlin.get([delta_seconds, 37.7, 2.8]) as f32;
    let perlin_value_2 = perlin.get([delta_seconds, 38.7, 2.8]) as f32;
    let perlin_value_3 = perlin.get([delta_seconds, 39.7, 2.8]) as f32;

    let shake = inspector_data.camera_shake;

    let max_yaw = inspector_data.max_camera_yaw;
    let max_pitch = inspector_data.max_camera_pitch;
    let max_roll = inspector_data.max_camera_roll;

    let yaw = Quat::from_rotation_y(max_yaw * shake * perlin_value_1);
    let pitch = Quat::from_rotation_x(max_pitch * shake * perlin_value_2);
    let roll = Quat::from_rotation_z(max_roll * shake * perlin_value_3);

    for (_, mut transform, _) in query.iter_mut() {
        //      transform.rotation = yaw;
        //      transform.rotation = pitch;
        //      transform.rotation = roll;

        let perlin_value = ((perlin.get([-delta_seconds, delta_seconds]) as f32) + 0.05) * 0.0001;
        //        println!("{}", perlin_value);
        //        transform.translation += Vec3::new(perlin_value, 0.0, 0.0) * 0.5;
        transform.rotation *= Quat::from_rotation_y(perlin_value);
    }
}

pub fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let orbit_key = KeyCode::LShift;
    let pan_button = MouseButton::Middle;
    let pan_key = KeyCode::LAlt;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) || keyboard_input.pressed(orbit_key) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) || keyboard_input.pressed(pan_key) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button)
        || input_mouse.just_pressed(orbit_button)
        || keyboard_input.just_released(orbit_key)
        || keyboard_input.just_pressed(orbit_key)
    {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation *= pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}

pub fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-25.0, 25.0, 0.0);

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

pub fn despawn_camera(
    mut commands: Commands,
    cameras: Query<Entity, With<PanOrbitCamera>>,
) {
    for entity in cameras.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
