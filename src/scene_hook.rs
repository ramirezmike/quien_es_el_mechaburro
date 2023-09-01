use crate::{assets, floor, game_state};
use bevy::{
    ecs::system::EntityCommands,
    gltf::Gltf,
    prelude::*,
    render::primitives::Aabb,
    scene::SceneInstance,
};

#[derive(Component, Debug)]
pub struct SceneHooked;

#[derive(Bundle)]
pub struct HookedSceneBundle {
    pub hook: SceneHook,
    pub scene: SceneBundle,
}

pub struct HookData<'a, 'w> {
    pub mesh: Option<&'a Mesh>,
    pub global_transform: Option<&'a GlobalTransform>,
    pub aabb: Option<&'a Aabb>,
    pub name: Option<&'a Name>,
    pub floor_manager: &'a mut ResMut<'w, floor::FloorManager>,
}

#[derive(Component)]
pub struct SceneHook {
    hook: Box<dyn Fn(&mut EntityCommands, HookData) + Send + Sync + 'static>,
}
impl SceneHook {
    pub fn new<F: Fn(&mut EntityCommands, HookData) + Send + Sync + 'static>(hook: F) -> Self {
        Self {
            hook: Box::new(hook),
        }
    }
}

#[derive(Component)]
pub struct SceneOnComplete {
    on_complete: Box<
        dyn Fn(
                &mut Commands,
                &Res<Assets<Gltf>>,
                &Res<assets::GameAssets>,
                &Res<game_state::GameState>,
            ) + Send
            + Sync
            + 'static,
    >,
}
impl SceneOnComplete {
    pub fn new<
        F: Fn(
                &mut Commands,
                &Res<Assets<Gltf>>,
                &Res<assets::GameAssets>,
                &Res<game_state::GameState>,
            ) + Send
            + Sync
            + 'static,
    >(
        f: F,
    ) -> Self {
        Self {
            on_complete: Box::new(f),
        }
    }
}

pub fn run_hooks(
    unloaded_instances: Query<
        (Entity, &SceneInstance, &SceneHook, Option<&SceneOnComplete>),
        Without<SceneHooked>,
    >,
    scene_manager: Res<SceneSpawner>,
    meshes: Res<Assets<Mesh>>,
    components: Query<(
        Option<&GlobalTransform>,
        Option<&Aabb>,
        Option<&Handle<Mesh>>,
        Option<&Name>,
    )>,
    gltfs: Res<Assets<Gltf>>,
    game_assets: Res<assets::GameAssets>,
    game_state: Res<game_state::GameState>,
    mut floor_manager: ResMut<floor::FloorManager>,
    mut cmds: Commands,
) {
    for (entity, instance, hooked, maybe_on_complete) in &unloaded_instances {
        for entity in scene_manager.iter_instance_entities(**instance) {
            if let Ok((global_transform, aabb, mesh_handle, name)) = components.get(entity) {
                let mesh = mesh_handle.and_then(|m| meshes.get(m));

                let hook_data = HookData {
                    mesh,
                    global_transform,
                    aabb,
                    name,
                    floor_manager: &mut floor_manager,
                };

                let mut cmd = cmds.entity(entity);
                (hooked.hook)(&mut cmd, hook_data);
            }
        }

        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).insert(SceneHooked);
            if let Some(on_complete) = maybe_on_complete {
                (on_complete.on_complete)(&mut cmds, &gltfs, &game_assets, &game_state);
            }
        }
    }
}

pub struct HookPlugin;
impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_hooks);
    }
}
