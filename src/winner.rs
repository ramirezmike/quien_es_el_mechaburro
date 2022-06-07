use crate::{
    assets::GameAssets, audio::GameAudio, cleanup, game_camera, game_state, menus, ui::avatar,
    ui::text_size, AppState,
};
use bevy::prelude::*;

pub struct WinnerPlugin;
impl Plugin for WinnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::WinnerDisplay).with_system(setup))
            .insert_resource(Timers::default())
            .add_system_set(
                SystemSet::on_exit(AppState::WinnerDisplay)
                    .with_system(cleanup::<CleanupMarker>)
                    .with_system(destroy_everything)
                    .with_system(game_camera::despawn_camera),
            )
            .add_system_set(
                SystemSet::on_update(AppState::WinnerDisplay).with_system(update_display),
            );
    }
}

#[derive(Component)]
struct CleanupMarker;

#[derive(Default)]
struct Timers {
    cooldown: f32,
    display_set: bool,
}

fn destroy_everything(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_display(
    mut commands: Commands,
    mut timers: ResMut<Timers>,
    time: Res<Time>,
    cleanups: Query<Entity, With<CleanupMarker>>,
    game_state: Res<game_state::GameState>,
    game_assets: Res<GameAssets>,
    mut app_state: ResMut<State<AppState>>,
    mut audio: GameAudio,
    text_scaler: text_size::TextScaler,
) {
    timers.cooldown -= time.delta_seconds();
    timers.cooldown = timers.cooldown.clamp(-10.0, 6.0);

    if timers.cooldown <= 0.0 {
        if timers.display_set {
            app_state.set(AppState::TitleScreen).unwrap();
            return;
        } else {
            for entity in cleanups.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    } else {
        return;
    }

    timers.display_set = true;
    timers.cooldown = 4.0;

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);

    let mut burros = game_state.burros.iter().collect::<Vec<_>>();
    burros.sort_by_key(|b| b.score);

    for b in burros.iter() {
        println!("{:?} {}", b.skin, b.score);
    }

    let winner = burros
        .last()
        .expect("There should be at least one burro here");
    let player_map = game_state.get_skin_player_map();

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(35.0)),
                margin: Rect::all(Val::Auto),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
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
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    menus::options::add_title(
                        parent,
                        game_assets.font.clone(),
                        text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.2),
                        match winner.skin {
                            game_state::BurroSkin::Pinata => "Pinata!",
                            game_state::BurroSkin::Meow => "Meow!",
                            game_state::BurroSkin::Salud => "Salud!",
                            game_state::BurroSkin::Mexico => "Mexico!",
                            game_state::BurroSkin::Medianoche => "Medianoche!",
                            game_state::BurroSkin::Morir => "Morir!",
                            game_state::BurroSkin::Gators => "Gators!",
                            game_state::BurroSkin::Aguas => "Aguas!",
                        },
                        Vec::<CleanupMarker>::new(), // just an empty vec since can't do <impl Trait>
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
                        &vec![winner],
                        &game_assets,
                        &player_map,
                        text_scaler.scale(menus::BUTTON_LABEL_FONT_SIZE),
                    );
                });
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        if winner.is_bot {
                            "¡Ay, que pena!".to_string()
                        } else {
                            "¡Ay, que padre!".to_string()
                        },
                        TextStyle {
                            font: game_assets.score_font.clone(),
                            font_size: text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.2),
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(CleanupMarker);
        });
    audio.play_sfx(&game_assets.fanfare_sfx);
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut timers: ResMut<Timers>,
    text_scaler: text_size::TextScaler,
) {
    timers.cooldown = 2.0;
    timers.display_set = false;
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(35.0)),
                margin: Rect::all(Val::Auto),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                //flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "And the winner is..",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(menus::DEFAULT_FONT_SIZE * 1.2),
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(CleanupMarker);
        });
}
