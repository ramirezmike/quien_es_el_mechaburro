use crate::menu::MenuOption;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct TitleScreenState {
    pub selected_option: TitleScreenOptions,
}

#[derive(Component, Copy, Clone, PartialEq, Default)]
pub enum TitleScreenOptions {
    #[default]
    Start,
    Quit,
}

impl MenuOption<2> for TitleScreenOptions {
    const ITEM: [TitleScreenOptions; 2] = [TitleScreenOptions::Start, TitleScreenOptions::Quit];

    fn get_label(&self) -> &str {
        match self {
            TitleScreenOptions::Start => "Start",
            TitleScreenOptions::Quit => "Quit",
        }
    }
}
