use super::{
    state::{Settings, SettingsMenuState},
    SettingDisplayMarker,
};
use crate::loading::command_ext::*;
use crate::{assets, audio, game_state, input, menu, ui, AppState};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use menu::MenuOption;

pub fn highlight_selection(
    settings_state: Res<SettingsMenuState>,
    mut settings: Query<(&Settings, Option<&mut BackgroundColor>, Option<&mut Text>)>,
) {
    for (&setting, maybe_background_color, maybe_text) in &mut settings {
        if setting == settings_state.selected_setting {
            if let Some(mut background_color) = maybe_background_color {
                *background_color = BackgroundColor(ui::HOVERED_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::BLACK;
                }
            }
        } else {
            if let Some(mut background_color) = maybe_background_color {
                *background_color = BackgroundColor(ui::NORMAL_BUTTON);
            }
            if let Some(mut text) = maybe_text {
                for text_section in text.sections.iter_mut() {
                    text_section.style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn update_values(
    setting_state: ResMut<SettingsMenuState>,
    mut settings: Query<(&mut Text, &Settings), With<SettingDisplayMarker>>,
) {
    for (mut text, setting) in &mut settings {
        text.sections[0].value = setting_state.display(setting).to_string();
    }
}

pub fn handle_input(
    mut commands: Commands,
    mut setting_state: ResMut<SettingsMenuState>,
    action_state: Query<&ActionState<input::MenuAction>>,
    game_assets: Res<assets::GameAssets>,
    player_selection: Res<menu::character_select::state::PlayerSelection>,
    mut game_state: ResMut<game_state::GameState>,
    mut audio: audio::GameAudio,
    mut axis_timer: Local<Timer>,
    time: Res<Time>,
) {
    let action_state = action_state.single();

    if axis_timer.tick(time.delta()).finished() && action_state.pressed(input::MenuAction::Move) {
        let axis_pair = action_state
            .clamped_axis_pair(input::MenuAction::Move)
            .unwrap();
        if axis_pair.y() == 1.0 {
            audio.play_sfx(&game_assets.sfx_1);
            setting_state.selected_setting = setting_state.selected_setting.previous();
            *axis_timer = Timer::from_seconds(0.2, TimerMode::Once);
        }
        if axis_pair.y() == -1.0 {
            audio.play_sfx(&game_assets.sfx_1);
            setting_state.selected_setting = setting_state.selected_setting.next();
            *axis_timer = Timer::from_seconds(0.2, TimerMode::Once);
        }

        if axis_pair.x() == 1.0 {
            audio.play_sfx(&game_assets.sfx_1);
            setting_state.increment();
            *axis_timer = Timer::from_seconds(0.2, TimerMode::Once);
        }
        if axis_pair.x() == -1.0 {
            audio.play_sfx(&game_assets.sfx_1);
            setting_state.decrement();
            *axis_timer = Timer::from_seconds(0.2, TimerMode::Once);
        }
    }

    if action_state.just_pressed(input::MenuAction::Up) {
        audio.play_sfx(&game_assets.sfx_1);
        setting_state.selected_setting = setting_state.selected_setting.previous();
    }

    if action_state.just_pressed(input::MenuAction::Down) {
        audio.play_sfx(&game_assets.sfx_1);
        setting_state.selected_setting = setting_state.selected_setting.next();
    }

    if action_state.just_pressed(input::MenuAction::Left) {
        audio.play_sfx(&game_assets.sfx_1);
        setting_state.decrement();
    }

    if action_state.just_pressed(input::MenuAction::Right) {
        audio.play_sfx(&game_assets.sfx_1);
        setting_state.increment();
    }

    if action_state.just_pressed(input::MenuAction::Select)
        && setting_state.selected_setting == Settings::Vamos
    {
        audio.play_sfx(&game_assets.sfx_1);

        const MIN_DIFFICULTY: f32 = 0.5;
        *game_state = game_state::GameState::initialize(
            player_selection
                .players
                .iter()
                .map(|x| game_state::BurroState::from(x.clone()))
                .collect::<Vec<_>>(),
            setting_state.number_of_bots.try_into().unwrap(),
            setting_state.unfair_advantage as f32 + MIN_DIFFICULTY,
            &game_assets.burro_assets,
        );

        audio.play_bgm(&game_assets.bgm_1);
        commands.load_state(AppState::LoadInGame);
    }
}
