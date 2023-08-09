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
) {
    let action_state = action_state.single();

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
