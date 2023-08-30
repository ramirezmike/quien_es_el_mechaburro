use bevy::prelude::*;
use crate::{ AppState, IngameState, cleanup, ui, assets, input, game_state };
use crate::loading::command_ext::*;
use crate::input::InputCommandsExt;
use leafwing_input_manager::prelude::*;

pub struct WinnerCirclePlugin;
impl Plugin for WinnerCirclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WinnerCircleTimer>()
            .add_systems(OnEnter(IngameState::WinnerCircle), setup)
            .add_systems(Update, handle_exit.run_if(in_state(IngameState::WinnerCircle)))
            .add_systems(OnExit(IngameState::WinnerCircle), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Resource, Default)]
struct WinnerCircleTimer(Timer);

fn setup( 
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    text_scaler: ui::text_size::TextScaler,
    mut timer: ResMut<WinnerCircleTimer>,
) {
    timer.0 = Timer::from_seconds(20., TimerMode::Once);
    commands.spawn_menu_input(CleanupMarker);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|builder| {
            builder
                .spawn(TextBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect {
                            top: Val::Percent(2.5),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        "Congratulations!",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE * 1.5),
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..Default::default()
                });
        });
}

fn handle_exit(
    action_state: Query<&ActionState<input::MenuAction>>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut timer: ResMut<WinnerCircleTimer>,
    mut game_state: ResMut<game_state::GameState>,
    time: Res<Time>,
) {
    let action_state = action_state.single();

    if action_state.just_pressed(input::MenuAction::Start) || timer.0.tick(time.delta()).finished() {
        game_state.current_level = 0;
        next_state.set(AppState::Splash);
        next_ingame_state.set(IngameState::Disabled);
    }
}
