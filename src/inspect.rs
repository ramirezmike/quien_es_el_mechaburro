use crate::{mesh, player};
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_inspector_egui::bevy_egui::EguiSettings;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::{plugin::InspectorWindows, Inspectable, InspectorPlugin};

pub struct InspectPlugin;
impl Plugin for InspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InspectorPlugin::<InspectorData>::new())
            .add_system(toggle)
            .add_startup_system(hide)
            .add_system(store_current_window_size)
            .add_system(paint_ui)
            .insert_resource(WindowSize {
                width: 0.0,
                height: 0.0,
            })
            .insert_resource(ShowInspector { visible: false })
            .register_type::<player::Player>()
            .register_type::<mesh::ScrollingPane>()
            .insert_resource(EguiSettings { scale_factor: 2.5 });
    }
}

pub struct ShowInspector {
    visible: bool,
}

#[derive(Inspectable)]
pub struct InspectorData {
    pub bullet_distance: f32,
    pub burro_speed: f32,
    pub follow_burros: bool,
    pub invulnerability_cooldown: f32,
    pub down_cooldown: f32,
    pub max_camera_yaw: f32,
    pub max_camera_pitch: f32,
    pub max_camera_roll: f32,
    pub camera_shake: f32,
    pub hit_starting_size: f32,
    pub hit_speed: f32,
    pub hit_shrink_speed: f32,
    pub hit_min_spread_x: f32,
    pub hit_max_spread_x: f32,
    pub hit_min_spread_y: f32,
    pub hit_max_spread_y: f32,
    pub hit_min_spread_z: f32,
    pub hit_max_spread_z: f32,
}

impl Default for InspectorData {
    fn default() -> Self {
        InspectorData {
            bullet_distance: 1.0,
            burro_speed: 40.0,
            follow_burros: true,
            invulnerability_cooldown: 1.0,
            down_cooldown: 1.5,
            max_camera_yaw: 1.0,
            max_camera_pitch: 1.0,
            max_camera_roll: 1.0,
            camera_shake: 0.3,
            hit_starting_size: 0.2,
            hit_speed: 9.0,
            hit_shrink_speed: 0.1,
            hit_min_spread_x: 0.0,
            hit_max_spread_x: 1.0,
            hit_min_spread_y: 0.0,
            hit_max_spread_y: 1.0,
            hit_min_spread_z: 0.0,
            hit_max_spread_z: 1.0,
        }
    }
}

fn hide(mut inspector_windows: ResMut<InspectorWindows>) {
    let mut inspector_window_data = inspector_windows.window_data_mut::<InspectorData>();
    inspector_window_data.visible = false;
}

fn toggle(
    input: ResMut<Input<KeyCode>>,
    mut inspector_windows: ResMut<InspectorWindows>,
    mut show_inspector: ResMut<ShowInspector>,
) {
    if input.just_pressed(KeyCode::Grave) {
        show_inspector.visible = !show_inspector.visible;

        let mut inspector_window_data = inspector_windows.window_data_mut::<InspectorData>();
        inspector_window_data.visible = show_inspector.visible;
    }
}

struct WindowSize {
    width: f32,
    height: f32,
}
fn store_current_window_size(
    windows: Res<Windows>,
    mut win_size: ResMut<WindowSize>,
    mut resize_event: EventReader<WindowResized>,
) {
    if win_size.width == 0.0 && win_size.height == 0.0 {
        if let Some(window) = windows.get_primary() {
            win_size.width = window.width();
            win_size.height = window.height();
        }
    }

    for e in resize_event.iter() {
        win_size.width = e.width;
        win_size.height = e.height;
    }
}

fn paint_ui(mut ctx: ResMut<EguiContext>, show_inspector: Res<ShowInspector>) {
    if !show_inspector.visible {
        return;
    }

    let ctx = ctx.ctx_mut();

    egui::Window::new("")
        .resizable(true)
        .title_bar(false)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.collapsing("Levels", |ui| {
                if ui.button("Level 1").clicked() {
                    //assets_handler.load(AppState::LevelOne, &mut game_assets);
                }
            });
            ui.end_row();
        });
}
