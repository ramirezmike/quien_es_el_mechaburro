use crate::{assets, bullet, burro, cleanup, game_camera, game_state, ui, AppState, IngameState};
use bevy::prelude::*;
use bevy_toon_shader::ToonShaderMaterial;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct MechaPickerPlugin;
impl Plugin for MechaPickerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "debug")]
        app.add_systems(Update, skip_picking.run_if(in_state(AppState::MechaPicker)));

        app.insert_resource(TextDisplayTimers::default())
            .add_systems(OnEnter(AppState::MechaPicker), setup)
            .add_event::<PickMechaEvent>()
            .add_systems(OnExit(AppState::MechaPicker), cleanup::<CleanupMarker>)
            .add_systems(
                Update,
                (pick_mecha, animate_mecha_selection, handle_mecha_pick_event)
                    .run_if(in_state(AppState::MechaPicker)),
            );
    }
}

#[derive(Component)]
struct TextMarker;

#[derive(Component)]
struct TopTextMarker;

#[derive(Component)]
struct CleanupMarker;

#[derive(Event)]
struct PickMechaEvent {
    selected_burro: usize,
}

#[derive(Default, Resource)]
struct TextDisplayTimers {
    name_change_cooldown: f32,
    overall_name_selection_cooldown: f32,
    mecha_display_cooldown: f32,

    has_picked: bool,
    selected_burro: Option<Entity>,
    mecha_selection_stage: MechaSelectionStage,
    mecha_selection_cooldown: f32,
}

fn skip_picking(
    mut next_state: ResMut<NextState<AppState>>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
) {
    next_state.set(AppState::InGame);
    next_ingame_state.set(IngameState::InGame);
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
    mut next_state: ResMut<NextState<AppState>>,
    mut next_ingame_state: ResMut<NextState<IngameState>>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    game_assets: Res<assets::GameAssets>,
    time: Res<Time>,
    mut burros: Query<(Entity, &mut Transform, &mut burro::Burro)>,
    mut burro_meshes: Query<(&mut Handle<ToonShaderMaterial>, &burro::BurroMeshMarker)>,
    mut camera_settings: ResMut<game_camera::CameraSettings>,
    mut bullet_event_writer: EventWriter<bullet::BulletEvent>,
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

        if let Ok((entity, mut transform, mut burro)) = burros.get_mut(selected_burro_entity) {
            let (mut toon_material, _) = burro_meshes
                .iter_mut()
                .filter(|(_, m)| m.parent.unwrap() == entity)
                .last()
                .unwrap();
            match text_display_timers.mecha_selection_stage {
                MechaSelectionStage::Initial => (),
                MechaSelectionStage::MovingToBurro => {
                    camera_settings.set_camera(2.0, transform.translation, 0.4, true, 30.0, 5.0);
                    for entity in top_texts.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                MechaSelectionStage::ChangingBurro => {
                    *toon_material = game_assets.mechaburro_texture.toon_texture.clone();
                    burro.is_mechaburro = true;
                    for entity in name_texts.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                MechaSelectionStage::LaserShot => {
                    println!("Sending bullet");
                    bullet_event_writer.send(bullet::BulletEvent {
                        source: selected_burro_entity,
                        speed: burro.bullet_speed,
                        time_to_live: burro.bullet_time_alive,
                        position: transform.translation,
                        direction: Vec3::new(1.0, 0.0, 0.0),
                        bullet_type: bullet::BulletType::Laser,
                    });
                    transform.rotation = Quat::from_axis_angle(Vec3::Y, 0.0);
                    transform.scale = Vec3::new(0.7, 1.4, 1.0);
                }
                MechaSelectionStage::ZoomOut => {
                    camera_settings.set_camera(20.0, Vec3::ZERO, 0.4, false, 30.0, 30.0);
                }
                MechaSelectionStage::StartRound => {
                    next_state.set(AppState::InGame);
                    next_ingame_state.set(IngameState::InGame);
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    text_scaler: ui::text_size::TextScaler,
) {
    *text_display_timers = TextDisplayTimers::default();
    text_display_timers.overall_name_selection_cooldown = 3.0;
    text_display_timers.has_picked = false;
    text_display_timers.mecha_selection_cooldown = 2.0;

    commands
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CleanupMarker)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "MECHABURRO!",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE * 1.5),
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..Default::default()
                })
                .insert(CleanupMarker)
                .insert(TextMarker);

            parent
                .spawn(TextBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "Y el \nMechaburro es..",
                        TextStyle {
                            font: game_assets.font.clone(),
                            font_size: text_scaler.scale(ui::DEFAULT_FONT_SIZE),
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..Default::default()
                })
                .insert(TopTextMarker)
                .insert(CleanupMarker);
        });
}

fn handle_mecha_pick_event(
    mut pick_mecha_event_reader: EventReader<PickMechaEvent>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    burros: Query<(Entity, &burro::Burro)>,
) {
    for event in pick_mecha_event_reader.iter() {
        for (entity, burro) in burros.iter() {
            if burro.selected_burro == event.selected_burro {
                text_display_timers.selected_burro = Some(entity);
                break;
            }
        }
    }
}

fn pick_mecha(
    time: Res<Time>,
    mut texts: Query<&mut Text, With<TextMarker>>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    mut pick_mecha_event_writer: EventWriter<PickMechaEvent>,
    game_assets: Res<assets::GameAssets>,
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
                selected_burro: choice.selected_burro,
            });
            text_display_timers.mecha_display_cooldown = 3.0;
            text_display_timers.has_picked = true;

            for mut text in texts.iter_mut() {
                let burro_name = game_assets.burro_assets[choice.selected_burro].name.clone();
                text.sections[0].value = burro_name;
            }
        }
    } else {
        let current_burros = game_state
            .burros
            .iter()
            .map(|x| x.selected_burro)
            .collect::<Vec<_>>();

        for mut text in texts.iter_mut() {
            if let Some(choice) = current_burros.choose(&mut rng) {
                let burro_name = game_assets.burro_assets[*choice].name.clone();
                text.sections[0].value = burro_name;
            }
        }
        text_display_timers.name_change_cooldown = 0.15;
    }
}
