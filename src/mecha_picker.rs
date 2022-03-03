use crate::{
    assets::GameAssets, bullet::BulletEvent, bullet::BulletType, burro, cleanup, game_camera,
    game_state, AppState,
};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct MechaPickerPlugin;
impl Plugin for MechaPickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MechaPicker).with_system(setup))
            .add_event::<PickMechaEvent>()
            .add_system_set(
                SystemSet::on_exit(AppState::MechaPicker).with_system(cleanup::<CleanupMarker>),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MechaPicker)
                    .with_system(pick_mecha)
                    .with_system(animate_mecha_selection)
                    .with_system(handle_mecha_pick_event),
            )
            .insert_resource(TextDisplayTimers::default());
    }
}

#[derive(Component)]
struct TextMarker;

#[derive(Component)]
struct TopTextMarker;

#[derive(Component)]
struct CleanupMarker;

struct PickMechaEvent {
    burro_skin: game_state::BurroSkin,
}

#[derive(Default)]
struct TextDisplayTimers {
    name_change_cooldown: f32,
    overall_name_selection_cooldown: f32,
    mecha_display_cooldown: f32,

    has_picked: bool,
    selected_burro: Option<Entity>,
    mecha_selection_stage: MechaSelectionStage,
    mecha_selection_cooldown: f32,
}

#[derive(PartialEq, Debug)]
enum MechaSelectionStage {
    Initial,
    MovingToBurro,
    ChangingBurro,
    LaserShot,
    ZoomOut,
    StartRound,
}

impl Default for MechaSelectionStage {
    fn default() -> Self {
        MechaSelectionStage::Initial
    }
}

fn animate_mecha_selection(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    game_state: Res<game_state::GameState>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
    mut burros: Query<(
        &mut Transform,
        &mut burro::Burro,
        &mut Handle<StandardMaterial>,
    )>,
    mut camera_settings: ResMut<game_camera::CameraSettings>,
    mut bullet_event_writer: EventWriter<BulletEvent>,
    top_texts: Query<Entity, With<TopTextMarker>>,
    name_texts: Query<Entity, With<TextMarker>>,
) {
    if let Some(selected_burro_entity) = text_display_timers.selected_burro {
        text_display_timers.mecha_selection_cooldown -= time.delta_seconds();
        text_display_timers.mecha_selection_cooldown = text_display_timers
            .mecha_selection_cooldown
            .clamp(-10.0, 10.0);
        if text_display_timers.mecha_selection_cooldown <= 0.0 {
            let (next_state, cooldown) = match text_display_timers.mecha_selection_stage {
                MechaSelectionStage::Initial => (MechaSelectionStage::MovingToBurro, 2.0),
                MechaSelectionStage::MovingToBurro => (MechaSelectionStage::ChangingBurro, 2.0),
                MechaSelectionStage::ChangingBurro => (MechaSelectionStage::LaserShot, 2.0),
                MechaSelectionStage::LaserShot => (MechaSelectionStage::ZoomOut, 2.0),
                MechaSelectionStage::ZoomOut => (MechaSelectionStage::StartRound, 0.0),
                MechaSelectionStage::StartRound => (MechaSelectionStage::StartRound, 0.0),
            };

            text_display_timers.mecha_selection_stage = next_state;
            text_display_timers.mecha_selection_cooldown = cooldown;
        } else {
            return;
        }

        if let Ok((mut transform, mut burro, mut handle)) = burros.get_mut(selected_burro_entity) {
            match text_display_timers.mecha_selection_stage {
                MechaSelectionStage::Initial => (),
                MechaSelectionStage::MovingToBurro => {
                    camera_settings.set_camera(2.0, transform.translation, 0.4, true, 30.0, 5.0);
                    for entity in top_texts.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                MechaSelectionStage::ChangingBurro => {
                    *handle = game_assets.mechaburro_texture.material.clone();
                    burro.is_mechaburro = true;
                    for entity in name_texts.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                MechaSelectionStage::LaserShot => {
                    use std::f32::consts::PI;

                    bullet_event_writer.send(BulletEvent {
                        source: selected_burro_entity,
                        speed: burro.bullet_speed,
                        time_to_live: burro.bullet_time_alive,
                        position: transform.translation,
                        direction: Vec3::new(1.0, 0.0, 0.0),
                        bullet_type: BulletType::Laser,
                    });
                    transform.rotation = Quat::from_axis_angle(Vec3::Y, 0.0);
                    transform.scale = Vec3::new(0.7, 1.4, 1.0);
                }
                MechaSelectionStage::ZoomOut => {
                    camera_settings.set_camera(20.0, Vec3::ZERO, 0.4, false, 30.0, 30.0);
                }
                MechaSelectionStage::StartRound => {
                    app_state.set(AppState::InGame).unwrap();
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
) {
    *text_display_timers = TextDisplayTimers::default();
    text_display_timers.overall_name_selection_cooldown = 3.0;
    text_display_timers.has_picked = false;
    text_display_timers.mecha_selection_cooldown = 2.0;

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
                        "Y el \nMechaburro es..",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 60.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(TopTextMarker)
                .insert(CleanupMarker);

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
                        "MECHABURRO!",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: 100.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(CleanupMarker)
                .insert(TextMarker);
        });
}

fn handle_mecha_pick_event(
    mut pick_mecha_event_reader: EventReader<PickMechaEvent>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    burros: Query<(Entity, &burro::Burro)>,
) {
    for event in pick_mecha_event_reader.iter() {
        for (entity, burro) in burros.iter() {
            if burro.burro_skin == event.burro_skin {
                text_display_timers.selected_burro = Some(entity);
                break;
            }
        }
    }
}

fn pick_mecha(
    mut app_state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut texts: Query<&mut Text, With<TextMarker>>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    mut pick_mecha_event_writer: EventWriter<PickMechaEvent>,
    game_state: Res<game_state::GameState>,
) {
    if text_display_timers.has_picked {
        text_display_timers.mecha_display_cooldown -= time.delta_seconds();
        text_display_timers.mecha_display_cooldown =
            text_display_timers.mecha_display_cooldown.clamp(-10.0, 3.0);

        return;
    }

    text_display_timers.name_change_cooldown -= time.delta_seconds();
    text_display_timers.name_change_cooldown =
        text_display_timers.name_change_cooldown.clamp(-10.0, 3.0);
    text_display_timers.overall_name_selection_cooldown -= time.delta_seconds();
    text_display_timers.overall_name_selection_cooldown = text_display_timers
        .overall_name_selection_cooldown
        .clamp(-10.0, 3.0);

    if text_display_timers.name_change_cooldown > 0.0 {
        return;
    }

    let mut rng = thread_rng();
    if text_display_timers.overall_name_selection_cooldown < 0.0 {
        // select mechaburro
        let actual_choices = game_state
            .burros
            .iter()
            .filter(|b| b.is_bot)
            .collect::<Vec<_>>();
        if let Some(choice) = actual_choices.choose(&mut rng) {
            pick_mecha_event_writer.send(PickMechaEvent {
                burro_skin: choice.skin,
            });
            text_display_timers.mecha_display_cooldown = 3.0;
            text_display_timers.has_picked = true;

            for mut text in texts.iter_mut() {
                text.sections[0].value = match choice.skin {
                    game_state::BurroSkin::Pinata => "Pinata",
                    game_state::BurroSkin::Meow => "Meow",
                    game_state::BurroSkin::Salud => "Salud",
                    game_state::BurroSkin::Mexico => "Mexico",
                    game_state::BurroSkin::Medianoche => "Medianoche",
                    game_state::BurroSkin::Morir => "Morir",
                    game_state::BurroSkin::Gators => "Gators",
                    game_state::BurroSkin::Aguas => "Aguas",
                }
                .to_string();
            }
        }
    } else {
        // change name
        let display_texts = [
            "Pinata",
            "Meow",
            "Salud",
            "Mexico",
            "Medianoche",
            "Morir",
            "Gators",
            "Aguas",
        ];

        for mut text in texts.iter_mut() {
            if let Some(choice) = display_texts.choose(&mut rng) {
                text.sections[0].value = choice.to_string();
            }
        }
        text_display_timers.name_change_cooldown = 0.15;
    }
}
