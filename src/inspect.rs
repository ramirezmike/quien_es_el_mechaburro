use crate::{mesh, player};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiSettings;
use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

pub struct InspectPlugin;
impl Plugin for InspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InspectorPlugin::<InspectorData>::new())
            .register_type::<player::Player>()
            .register_type::<mesh::ScrollingPane>()
            .insert_resource(EguiSettings { scale_factor: 2.5 });
    }
}

#[derive(Inspectable, Default)]
struct InspectorData {
    #[inspectable(despawnable = true)]
    player: InspectorQuerySingle<Entity, With<player::Player>>,
    scrolls: InspectorQuery<Entity, With<mesh::ScrollingPane>>,
}
