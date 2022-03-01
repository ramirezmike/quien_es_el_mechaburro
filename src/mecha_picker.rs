use crate::{assets::GameAssets, burro, cleanup, game_state, AppState};
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
                    .with_system(handle_mecha_pick_event),
            )
            .insert_resource(TextDisplayTimers::default());
    }
}

#[derive(Component)]
struct TextMarker;

#[derive(Component)]
struct CleanupMarker;

struct PickMechaEvent {
    burro_skin: game_state::BurroSkin,
}

#[derive(Default)]
struct TextDisplayTimers {
    name_change_cooldown: f32,
    overall_name_selection_cooldown: f32,
    has_picked: bool,
}

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
) {
    text_display_timers.overall_name_selection_cooldown = 3.0;
    text_display_timers.has_picked = false;

    // UI camera
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CleanupMarker);
    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                //justify_content: JustifyContent::Center,
                //position_type: PositionType::Absolute,
                //              margin: Rect {
                //                  left: Val::Auto,
                //                  right: Val::Auto,
                //                  ..Default::default()
                //              },
                //              position: Rect {
                //                  left: Val::Auto,
                //                  right: Val::Auto,
                //                  //top: Val::Percent(50.0),
                //                  ..Default::default()
                //              },
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
}

fn handle_mecha_pick_event(
    mut app_state: ResMut<State<AppState>>,
    mut pick_mecha_event_reader: EventReader<PickMechaEvent>,
    game_state: Res<game_state::GameState>,
    game_assets: Res<GameAssets>,
    mut burros: Query<(&mut burro::Burro, &mut Handle<StandardMaterial>)>,
) {
    let mut mecha_set = false;
    for event in pick_mecha_event_reader.iter() {
        for (mut burro, mut handle) in burros.iter_mut() {
            if burro.burro_skin == event.burro_skin {
                *handle = game_assets.mechaburro_texture.material.clone();
                burro.is_mechaburro = true;
                mecha_set = true;
                break;
            }
        }
    }

    if mecha_set {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn pick_mecha(
    time: Res<Time>,
    mut texts: Query<&mut Text, With<TextMarker>>,
    mut text_display_timers: ResMut<TextDisplayTimers>,
    mut pick_mecha_event_writer: EventWriter<PickMechaEvent>,
    game_state: Res<game_state::GameState>,
) {
    if text_display_timers.has_picked {
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
