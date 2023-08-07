pub mod command_ext {
    use crate::{
        asset_loading::QueueState,
        ingame,
        menu::{settings, splash, title_screen},
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

            match self.0 {
                AppState::Settings | AppState::CharacterSelect => {
                    title_screen::TitleScreenLoader.apply(world);
                    settings::SettingsMenuLoader.apply(world);

                    // TODO: This is temporary to load burro burro_assets
                    //  before the game_state initializes until the burro select 
                    //  screen is working since it will load the burro_assets
                    ingame::IngameLoader.apply(world)
                },
                AppState::LoadInGame => ingame::IngameLoader.apply(world),
                AppState::Splash => splash::SplashLoader.apply(world),
                AppState::TitleScreen => title_screen::TitleScreenLoader.apply(world),
                _ => (),
            }
        }
    }
}
