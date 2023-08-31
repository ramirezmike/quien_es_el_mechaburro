use crate::{asset_loading, assets};
use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};

pub struct TitleScreenLoader;
impl Command for TitleScreenLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            asset_loading::AssetsHandler,
            ResMut<assets::GameAssets>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

        assets_handler.add_audio(&mut game_assets.title_bgm, "audio/title.ogg");
        assets_handler.add_audio(&mut game_assets.bgm_1, "audio/baila.ogg");
        assets_handler.add_audio(&mut game_assets.sfx_1, "audio/blip.wav");
        assets_handler.add_audio(&mut game_assets.sfx_2, "audio/select.wav");
        assets_handler.add_font(&mut game_assets.font, "fonts/MexicanTequila.ttf");
        assets_handler.add_font(&mut game_assets.score_font, "fonts/monogram.ttf");
        assets_handler.add_material(
            &mut game_assets.title_screen_background,
            "textures/background.png",
            false,
        );
        assets_handler.add_material(
            &mut game_assets.title_screen_logo,
            "textures/logo.png",
            true,
        );
    }
}
