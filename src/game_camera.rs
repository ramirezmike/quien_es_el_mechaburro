//use crate::{inspect, player};
use crate::AppState;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::Window;
use bevy_toon_shader::ToonShaderMainCamera;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            //            .add_system(debug_camera.in_set(OnUpdate(AppState::InGame)))
            //          .add_system(check_for_follow_burros)
            //          .add_system(follow_following)
            .add_systems(
                Update,
                update_camera.run_if(in_state(AppState::MechaPicker)),
            )
            .add_systems(Update, update_camera.run_if(in_state(AppState::InGame)));
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

#[derive(Default, Resource)]
pub struct CameraSettings {
    look_at: Vec3,
    height: f32,
    orbit: bool,
    speed: f32,
    distance: f32,
    target_distance: f32,
    pub follow: Option<Entity>,
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

#[cfg(feature = "debug")]
fn update_camera(
    mut cameras: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
    //  camera_settings: ResMut<CameraSettings>,
    //  time: Res<Time>,
    windows: Query<&Window>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let orbit_key = KeyCode::LShift;
    let pan_button = MouseButton::Middle;
    let pan_key = KeyCode::LAlt;
    let window = windows.single();

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

    for (mut pan_orbit, mut transform, projection) in cameras.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted,
            // so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(window);
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
            transform.rotation *= yaw; // rotate around global y axis
            transform.rotation *= pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(window);
            if let Projection::Perspective(p) = projection {
                pan *= Vec2::new(p.fov * p.aspect_ratio, p.fov) / window;
            }

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

#[cfg(not(feature = "debug"))]
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

fn get_primary_window_size(window: &Window) -> Vec2 {
    Vec2::new(window.width(), window.height())
}

pub fn spawn_camera_with_transform<T: Component>(
    commands: &mut Commands,
    transform: Transform,
    cleanup_marker: T,
) {
    let radius = transform.translation.length();

    commands.spawn((
        Camera3dBundle {
            transform,
            ..default()
        },
        cleanup_marker,
        ToonShaderMainCamera,
        ComputedVisibility::default(),
        Visibility::Visible,
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));
}

pub fn spawn_camera<T: Component>(commands: &mut Commands, cleanup_marker: T) {
    #[cfg(not(feature = "debug"))]
    let translation = Vec3::new(-4.0, 1.0, 0.0);

    #[cfg(feature = "debug")]
    let translation = Vec3::new(-30.0, 20.0, 0.0);

    let transform = Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y);
    spawn_camera_with_transform(commands, transform, cleanup_marker);
}
