
pub mod command_ext {
    use crate::{AppState, menu::options, ingame, asset_loading::QueueState};
    use bevy::prelude::*;
    use bevy::ecs::system::{Command, SystemState};

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
            let mut system_state: SystemState<(ResMut<QueueState>, ResMut<NextState<AppState>>)> = SystemState::new(world);
            let (mut queued_state, mut next_state) = system_state.get_mut(world);

            queued_state.state = self.0;
            next_state.set(AppState::Loading);

            match self.0 {
                AppState::Options => options::OptionsMenutLoader.apply(world),
                AppState::LoadInGame => ingame::IngameLoader.apply(world),
                _ => ()
            }
        }
    }
}
