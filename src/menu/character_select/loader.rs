use crate::{asset_loading, assets};
use bevy::{
    ecs::system::{Command, SystemState},
    prelude::*,
};
use bevy_toon_shader::ToonShaderMaterial;
use std::fs;

pub struct CharacterSelectLoader;
impl Command for CharacterSelectLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            asset_loading::AssetsHandler,
            ResMut<assets::GameAssets>,
            ResMut<Assets<ToonShaderMaterial>>,
        )> = SystemState::new(world);
        let (mut assets_handler, mut game_assets, mut toon_materials) = system_state.get_mut(world);

        assets_handler.add_font(&mut game_assets.score_font, "fonts/monogram.ttf");
        assets_handler.add_glb(&mut game_assets.burro, "models/burro_new.glb");

        let folder_path = "assets/textures/burros";
        if let Ok(entries) = fs::read_dir(folder_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();

                    if let Some(extension) = file_path.extension() {
                        if extension == "png" {
                            if let Some(file_name) = file_path.file_stem() {
                                if let Some(name) = file_name.to_str() {
                                    let mut texture = asset_loading::GameTexture::default();
                                    assets_handler.add_material(
                                        &mut texture,
                                        &format!("textures/burros/{}.png", name),
                                        false,
                                    );

                                    let toon_material_textured =
                                        toon_materials.add(ToonShaderMaterial {
                                            base_color_texture: Some(texture.image.clone()),
                                            color: Color::default(),
                                            sun_dir: Vec3::new(0.0, 0.0, 0.0),
                                            sun_color: Color::default(),
                                            camera_pos: Vec3::new(0.0, 0.5, 10.0),
                                            ambient_color: Color::default(),
                                        });

                                    game_assets.burro_assets.push(assets::BurroAsset {
                                        name: name.replace("_", " ").to_uppercase().into(),
                                        texture,
                                        toon_texture: toon_material_textured,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        } else {
            panic!("Assets folder or burros folder not found, can't run the game");
        }
    }
}
