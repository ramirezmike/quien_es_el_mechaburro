use crate::{asset_loading, audio::GameAudio, assets::GameAssets, burro, cleanup, game_camera, game_state, AppState};
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
        println!("{:?} {}",b.skin,b.score);
    }

    let winner = burros
        .last()
        .expect("There should be at least one burro here");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                margin: Rect {
                    top: Val::Px(-40.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
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
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        match winner.skin {
                            game_state::BurroSkin::Pinata => "Pinata!".to_string(),
                            game_state::BurroSkin::Meow => "Meow!".to_string(),
                            game_state::BurroSkin::Salud => "Salud!".to_string(),
                            game_state::BurroSkin::Mexico => "Mexico!".to_string(),
                            game_state::BurroSkin::Medianoche => "Medianoche!".to_string(),
                            game_state::BurroSkin::Morir => "Morir!".to_string(),
                            game_state::BurroSkin::Gators => "Gators!".to_string(),
                            game_state::BurroSkin::Aguas => "Aguas!".to_string(),
                        },
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 60.0,
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

            use game_state::BurroSkin;
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Auto),
                    ..Default::default()
                },
                image: match winner.skin {
                    BurroSkin::Pinata => game_assets.pinata_logo_texture.image.clone().into(),
                    BurroSkin::Meow => game_assets.meow_logo_texture.image.clone().into(),
                    BurroSkin::Salud => game_assets.salud_logo_texture.image.clone().into(),
                    BurroSkin::Mexico => game_assets.mexico_logo_texture.image.clone().into(),
                    BurroSkin::Medianoche => {
                        game_assets.medianoche_logo_texture.image.clone().into()
                    }
                    BurroSkin::Morir => game_assets.morir_logo_texture.image.clone().into(),
                    BurroSkin::Gators => game_assets.gators_logo_texture.image.clone().into(),
                    BurroSkin::Aguas => game_assets.aguas_logo_texture.image.clone().into(),
                },
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                margin: Rect {
                    top: Val::Px(40.0),
                    ..Default::default()
                },
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
                        size: Size::new(Val::Percent(100.0), Val::Px(140.0)),
                        position_type: PositionType::Relative,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        if winner.is_bot {
                            "Ay que pena!".to_string()
                        } else {
                            "Congratulations!".to_string()
                        },
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 60.0,
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

fn setup(mut commands: Commands, game_assets: Res<GameAssets>, mut timers: ResMut<Timers>) {
    timers.cooldown = 2.0;
    timers.display_set = false;
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
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
                            font_size: 60.0,
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
