//use crate::{inspect, player};
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::window::Window;
use crate::AppState;

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
//            .add_system(debug_camera.in_set(OnUpdate(AppState::InGame)))
//          .add_system(check_for_follow_burros)
//          .add_system(follow_following)
           .add_system(update_camera.in_set(OnUpdate(AppState::InGame)));
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
            let window = get_primary_window_size(&window);
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
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&window);
            match projection {
                Projection::Perspective(p) => {
                    pan *= Vec2::new(p.fov * p.aspect_ratio, p.fov) / window;
                },
                _ => ()
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

fn get_primary_window_size(window: &Window) -> Vec2 {
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

pub fn spawn_camera<T: Component>(commands: &mut Commands, cleanup_marker: T) {
    let translation = Vec3::new(-4.0, 1.0, 0.0);

    let radius = translation.length();

    commands.spawn((Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        cleanup_marker,
        ComputedVisibility::default(),
        Visibility::Visible,
        PanOrbitCamera {
            radius,
            ..Default::default()
        }
    ));
}
