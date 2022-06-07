use crate::{
    assets::GameAssets, burro, cleanup, game_state, menus, ui::avatar, ui::text_size, AppState,
};
use bevy::prelude::*;

pub struct InGameUIPlugin;
impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Debug).with_system(setup))
            .add_system_set(
                SystemSet::on_enter(AppState::ScoreDisplay).with_system(cleanup::<CleanupMarker>),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_hearts)
                    .with_system(detect_round_over),
            );
    }
}

#[derive(Component)]
struct CleanupMarker;

fn detect_round_over(
    game_state: Res<game_state::GameState>,
    mut app_state: ResMut<State<AppState>>,
    burros: Query<&burro::Burro>,
) {
    if burros.iter().count() <= 1 && !game_state.current_level_over {
        app_state.push(AppState::ScoreDisplay).unwrap();
    }
}

fn update_hearts(
    game_state: Res<game_state::GameState>,
    burros: Query<&burro::Burro>,
    mut hearts: Query<&mut Visibility, With<UiImage>>,
) {
    game_state.burros.iter().for_each(|burro_state| {
        let burro = burros
            .iter()
            .filter(|b| b.burro_skin == burro_state.skin)
            .last();

        if let Some(burro) = burro {
            burro_state
                .hearts
                .iter()
                .enumerate()
                .for_each(|(i, entity)| {
                    if let Ok(mut heart_visibility) = hearts.get_mut(*entity) {
                        heart_visibility.is_visible = i < burro.health;
                    }
                });
        } else {
            // burro must be dead already
            burro_state.hearts.iter().for_each(|entity| {
                if let Ok(mut heart_visibility) = hearts.get_mut(*entity) {
                    heart_visibility.is_visible = false;
                }
            });
        }
    });
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<game_state::GameState>,
    text_scaler: text_size::TextScaler,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(98.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let player_map = game_state.get_skin_player_map();
                    let burros = &game_state.burros.iter().collect();

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                margin: Rect {
                                    top: Val::Percent(2.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            avatar::insert_avatars(
                                parent,
                                burros,
                                &game_assets,
                                &player_map,
                                text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                            );
                        });
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                position_type: PositionType::Relative,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                margin: Rect {
                                    top: Val::Percent(7.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            avatar::insert_health_indicators(
                                parent,
                                &mut game_state.burros,
                                &game_assets,
                            );
                        });
                });
        });
}
