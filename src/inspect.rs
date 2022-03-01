use crate::{asset_loading, assets::GameAssets, mesh, player};
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy_inspector_egui::bevy_egui::EguiSettings;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
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
    pub max_camera_yaw: f32,
    pub max_camera_pitch: f32,
    pub max_camera_roll: f32,
    pub camera_shake: f32,
}

impl Default for InspectorData {
    fn default() -> Self {
        InspectorData {
            bullet_distance: 1.0,
            max_camera_yaw: 1.0,
            max_camera_pitch: 1.0,
            max_camera_roll: 1.0,
            camera_shake: 0.3,
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

fn paint_ui(
    mut ctx: ResMut<EguiContext>,
    win_size: Res<WindowSize>,
    mut assets_handler: asset_loading::AssetsHandler,
    mut game_assets: ResMut<GameAssets>,
    mut show_inspector: ResMut<ShowInspector>,
) {
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
