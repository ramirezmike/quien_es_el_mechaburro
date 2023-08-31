use super::state::{TitleScreenOptions, TitleScreenState};
use crate::loading::command_ext::*;
use crate::{assets, audio, input, menu, ui, AppState};
use bevy::prelude::*;
use bevy::{app::AppExit, ecs::event::Events};
use leafwing_input_manager::prelude::*;
use menu::MenuOption;

pub fn highlight_selection(
    options_state: Res<TitleScreenState>,
    mut options: Query<(
        &TitleScreenOptions,
        Option<&mut BackgroundColor>,
        Option<&mut Text>,
    )>,
) {
    for (&option, maybe_background_color, maybe_text) in &mut options {
        if option == options_state.selected_option {
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

pub fn handle_input(
    mut commands: Commands,
    mut title_screen_state: ResMut<TitleScreenState>,
    action_state: Query<&ActionState<input::MenuAction>>,
    game_assets: Res<assets::GameAssets>,
    mut exit: ResMut<Events<AppExit>>,
    mut audio: audio::GameAudio,
    mut axis_timer: Local<Timer>,
    time: Res<Time>,
) {
    let action_state = action_state.single();

    if action_state.pressed(input::MenuAction::Move) && axis_timer.tick(time.delta()).finished() {
        let axis_pair = action_state.clamped_axis_pair(input::MenuAction::Move).unwrap();
        if axis_pair.y() == 1.0 {
            audio.play_sfx(&game_assets.sfx_1);
            title_screen_state.selected_option = title_screen_state.selected_option.previous();
            *axis_timer = Timer::from_seconds(0.2, TimerMode::Once);
        }
        if axis_pair.y() == -1.0 {
            audio.play_sfx(&game_assets.sfx_1);
            title_screen_state.selected_option = title_screen_state.selected_option.next();
            *axis_timer = Timer::from_seconds(0.2, TimerMode::Once);
        }
    }
    if action_state.just_pressed(input::MenuAction::Up) {
        audio.play_sfx(&game_assets.sfx_1);
        title_screen_state.selected_option = title_screen_state.selected_option.previous();
    }

    if action_state.just_pressed(input::MenuAction::Down) {
        audio.play_sfx(&game_assets.sfx_1);
        title_screen_state.selected_option = title_screen_state.selected_option.next();
    }

    if action_state.just_pressed(input::MenuAction::Select) {
        audio.play_sfx(&game_assets.sfx_1);
        match title_screen_state.selected_option {
            TitleScreenOptions::Start => commands.load_state(AppState::CharacterSelect),
            TitleScreenOptions::Quit => exit.send(AppExit),
        }
    }
}
