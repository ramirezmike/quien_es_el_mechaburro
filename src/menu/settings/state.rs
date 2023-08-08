use crate::util::num_ext::*;
use crate::{config, menu::MenuOption};
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SettingsMenuState {
    pub selected_setting: Settings,
    pub number_of_players: isize,
    pub number_of_bots: isize,
    pub unfair_advantage: isize,
}

impl SettingsMenuState {
    pub fn display(&self, setting: &Settings) -> String {
        match setting {
            Settings::NumberOfBots => format!("{}", self.number_of_bots),
            Settings::UnfairAdvantage => match self.unfair_advantage {
                0 => "Mechaburrito".to_string(),
                1 => " Mechaburro ".to_string(),
                _ => "Mechagigante".to_string(),
            },
            setting => setting.get_label().to_string(),
        }
    }

    pub fn max_bots(&self) -> isize {
        config::MAX_NUMBER_OF_PLAYERS - self.number_of_players
    }

    pub fn min_bots(&self) -> isize {
        match self.number_of_players {
            1 => 1,
            _ => 0,
        }
    }

    pub fn increment(&mut self) {
        match self.selected_setting {
            Settings::NumberOfBots => {
                self.number_of_bots = self
                    .number_of_bots
                    .circular_increment(self.min_bots(), self.max_bots());
            }
            Settings::UnfairAdvantage => {
                self.unfair_advantage = self.unfair_advantage.circular_increment(0, 2);
            }
            _ => (),
        }
    }

    pub fn decrement(&mut self) {
        match self.selected_setting {
            Settings::NumberOfBots => {
                self.number_of_bots = self
                    .number_of_bots
                    .circular_decrement(self.min_bots(), self.max_bots());
            }
            Settings::UnfairAdvantage => {
                self.unfair_advantage = self.unfair_advantage.circular_decrement(0, 2);
            }
            _ => (),
        }
    }
}

#[derive(Component, Copy, Clone, PartialEq, Default)]
pub enum Settings {
    #[default]
    NumberOfBots,
    UnfairAdvantage,
    Vamos,
}

impl MenuOption<3> for Settings {
    const ITEM: [Settings; 3] = [
        Settings::NumberOfBots,
        Settings::UnfairAdvantage,
        Settings::Vamos,
    ];

    fn get_label(&self) -> &str {
        match self {
            Settings::NumberOfBots => "Number of Bots",
            Settings::UnfairAdvantage => "Unfair Advantage",
            Settings::Vamos => "¡Vamos!",
        }
    }
}
