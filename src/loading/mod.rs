pub mod command_ext {
    use crate::{
        asset_loading::QueueState,
        ingame,
        menu::{character_select, settings, splash, title_screen},
        AppState,
    };
    use bevy::ecs::system::{Command, SystemState};
    use bevy::prelude::*;

    pub trait CommandLoading {
        fn load_state(&mut self, state: AppState);
    }

    impl<'w, 's> CommandLoading for Commands<'w, 's> {
        fn load_state(&mut self, state: AppState) {
            self.add(StateSetter(state));
        }
    }

    pub struct StateSetter(AppState);
    impl Command for StateSetter {
        fn apply(self, world: &mut World) {
            let mut system_state: SystemState<(ResMut<QueueState>, ResMut<NextState<AppState>>)> =
                SystemState::new(world);
            let (mut queued_state, mut next_state) = system_state.get_mut(world);

            queued_state.state = self.0;
            next_state.set(AppState::Loading);

            #[cfg(feature = "debug")]
            {
                ingame::IngameLoader.apply(world);
                settings::loader::SettingsMenuLoader.apply(world);
                splash::SplashLoader.apply(world);
                character_select::loader::CharacterSelectLoader.apply(world);
                title_screen::loader::TitleScreenLoader.apply(world);
                return;
            }

            match self.0 {
                AppState::Settings => settings::loader::SettingsMenuLoader.apply(world),
                AppState::CharacterSelect => {
                    character_select::loader::CharacterSelectLoader.apply(world)
                }
                AppState::LoadInGame => ingame::IngameLoader.apply(world),
                AppState::Splash => splash::SplashLoader.apply(world),
                AppState::TitleScreen => title_screen::loader::TitleScreenLoader.apply(world),
                _ => (),
            }
        }
    }
}
