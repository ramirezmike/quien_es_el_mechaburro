use crate::loading::command_ext::*;
use crate::{
    asset_loading, assets, cleanup, game_camera, ui, ui::text_size, AppState, IngameState,
};
use bevy::prelude::*;

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), setup)
            .init_resource::<SplashTracker>()
            .add_systems(Update, tick.run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), cleanup::<CleanupMarker>);
    }
}

use bevy::ecs::system::{Command, SystemState};
pub struct SplashLoader;
impl Command for SplashLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            asset_loading::AssetsHandler,
            ResMut<assets::GameAssets>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

        assets_handler.add_font(&mut game_assets.font, "fonts/MexicanTequila.ttf");
        assets_handler.add_font(&mut game_assets.score_font, "fonts/monogram.ttf");
        assets_handler.add_material(&mut game_assets.bevy_icon, "textures/bevy.png", true);
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Default, Resource)]
struct SplashTracker {
    time: f32,
}

fn tick(mut commands: Commands, time: Res<Time>, mut splash_tracker: ResMut<SplashTracker>) {
    splash_tracker.time += time.delta_seconds();

    if splash_tracker.time > 3.0 {
        commands.load_state(AppState::TitleScreen);
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    text_scaler: text_size::TextScaler,
    mut splash_tracker: ResMut<SplashTracker>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
) {
    splash_tracker.time = 0.0;

    next_ingame_state.set(IngameState::Disabled);
    game_camera::spawn_camera(&mut commands, CleanupMarker);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Percent(60.0),
                    ..default()
                },
                image: game_assets.bevy_icon.image.clone().into(),
                ..default()
            });

            parent.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                text: Text::from_section(
                    "made with Bevy",
                    TextStyle {
                        font: game_assets.score_font.clone(),
                        font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE * 1.2),
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
        });
}
