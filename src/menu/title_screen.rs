use crate::{
    asset_loading, assets::GameAssets, cleanup,
    ui, AppState, input, assets, game_camera, audio,
};
use crate::loading::command_ext::*;
use bevy::prelude::*;
use bevy::{app::AppExit, ecs::event::Events};
use crate::input::InputCommandsExt;
use leafwing_input_manager::prelude::*;

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::TitleScreen), setup)
            .init_resource::<TitleScreenState>()
            .add_plugins(MaterialPlugin::<ScrollingImageMaterial>::default())
            .add_systems(Update, (highlight_selection, handle_input).run_if(in_state(AppState::TitleScreen)))
            .add_systems(OnExit(AppState::TitleScreen), cleanup::<CleanupMarker>);
    }
}

#[derive(Component)]
pub struct CleanupMarker;

use bevy::{
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};
#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "b62bb455-a72c-4b56-87bb-81e0554e234f"]
pub struct ScrollingImageMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
}

impl Material for ScrollingImageMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/scroll_texture.wgsl".into()
    }
}

#[derive(Default, Resource)]
pub struct TitleScreenState {
    selected_option: TitleScreenOptions,
}

#[derive(Component, Copy, Clone, PartialEq, Default)]
enum TitleScreenOptions {
    #[default]
    Start,
    Quit
}

const _:() = {
    const OPTIONS: [TitleScreenOptions; 2] = [TitleScreenOptions::Start, TitleScreenOptions::Quit];

    impl TitleScreenOptions  {
        fn get() -> impl IntoIterator::<Item=TitleScreenOptions> + Clone {
            OPTIONS
        }

        fn next(&self) -> Self {
            let position = OPTIONS.iter().position(|x| x == self).unwrap();
            *OPTIONS.iter().cycle().nth(position + 1).unwrap()
        }

        fn previous(&self) -> Self {
            let position = OPTIONS.iter().rev().position(|x| x == self).unwrap();
            *OPTIONS.iter().rev().cycle().nth(position + 1).unwrap()
        }

        fn get_label(&self) -> &str {
            match self {
                TitleScreenOptions::Start => "Start",
                TitleScreenOptions::Quit => "Quit",
            }
        }
    }
};

use bevy::ecs::system::{Command, SystemState};
pub struct TitleScreenLoader;
impl Command for TitleScreenLoader {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
             asset_loading::AssetsHandler,
             ResMut<assets::GameAssets>)> = SystemState::new(world);
        let (mut assets_handler, mut game_assets) = system_state.get_mut(world);

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

fn setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut audio: audio::GameAudio,
    text_scaler: ui::text_size::TextScaler,
    mut scrolling_image_materials: ResMut<Assets<ScrollingImageMaterial>>,
) {
    let camera_transform = Transform::from_xyz(0.0, 4., 0.0).looking_at(Vec3::ZERO, -Vec3::Z);
    game_camera::spawn_camera_with_transform(&mut commands, camera_transform, CleanupMarker);
    commands.spawn_menu_input(CleanupMarker);

    commands.spawn((MaterialMeshBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: scrolling_image_materials.add(ScrollingImageMaterial {
            texture: game_assets.title_screen_background.image.clone()
        }),
        ..default()
    }, CleanupMarker));

  commands
      .spawn((NodeBundle {
          style: Style {
              width: Val::Percent(100.0),
              height: Val::Percent(100.0),
              position_type: PositionType::Absolute,
              justify_content: JustifyContent::FlexStart,
              align_items: AlignItems::Center,
              flex_direction: FlexDirection::Column,
              ..default()
          },
          background_color: BackgroundColor(Color::NONE),
          ..default()
      }, CleanupMarker))
      .with_children(|parent| {
          parent.spawn(ImageBundle {
              style: Style {
                  width: Val::Auto,
                  height: Val::Percent(80.0),
                  margin: UiRect { top: Val::Percent(2.5), ..default() },
                  ..default()
              },
              image: game_assets.title_screen_logo.image.clone().into(),
              ..default()
          });
      });

    commands
        .spawn((TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                ..default()
            },
            text: Text::from_section(
                "by michael ramirez".to_string(),
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: text_scaler.scale(ui::BY_LINE_FONT_SIZE),
                    color: Color::rgba(0.0, 0.0, 0.0, 1.0),
                },
            ),
            ..default()
        }, CleanupMarker));

    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(90.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::End,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        }, CleanupMarker))
        .with_children(|parent| {
            TitleScreenOptions::get()
            .into_iter().for_each(|option| {
                parent
                    .spawn((ButtonBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            width: Val::Percent(18.0),
                            height: Val::Percent(12.5),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: ui::NORMAL_BUTTON.into(),
                        ..Default::default()
                    }, option.clone()))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                option.get_label(),
                                TextStyle {
                                    font: game_assets.font.clone(),
                                    font_size: text_scaler.scale(ui::BUTTON_LABEL_FONT_SIZE),
                                    color: Color::rgb(0.0, 0.0, 0.0),
                                },
                            ),
                            ..default()
                        });
                    });
            });
        });

      audio.play_bgm(&game_assets.bgm_1);
}

fn highlight_selection(
    options_state: Res<TitleScreenState>,
    mut options: Query<(&TitleScreenOptions, Option<&mut BackgroundColor>, Option<&mut Text>)>,
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

fn handle_input(
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
            TitleScreenOptions::Start => commands.load_state(AppState::Options),
            TitleScreenOptions::Quit => exit.send(AppExit),
        }
    }
}
