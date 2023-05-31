use bevy::{
    prelude::*,
    scene::SceneInstance,
    ecs::{world::EntityRef, system::EntityCommands},
    render::primitives::Aabb,
    gltf::Gltf,
};
#[derive(Component, Debug)]
pub struct SceneHooked;

#[derive(Bundle)]
pub struct HookedSceneBundle {
    pub hook: SceneHook,
    #[bundle]
    pub scene: SceneBundle,
}

pub struct HookData<'a> {
    pub mesh: Option::<&'a Mesh>,
    pub global_transform: Option::<&'a GlobalTransform>,
    pub aabb: Option::<&'a Aabb>,
}

#[derive(Component)]
pub struct SceneHook {
    hook: Box<dyn Fn(&EntityRef, &mut EntityCommands, HookData) + Send + Sync + 'static>,
}
impl SceneHook {
    pub fn new<F: Fn(&EntityRef, &mut EntityCommands, HookData) + Send + Sync + 'static>(hook: F) -> Self {
        Self { hook: Box::new(hook) }
    }
}


#[derive(Component)]
pub struct SceneOnComplete {
    on_complete: Box<dyn Fn(&mut Commands, &Res<Assets<Gltf>>) + Send + Sync + 'static>,
}
impl SceneOnComplete {
    pub fn new<F: Fn(&mut Commands, &Res<Assets<Gltf>>) + Send + Sync + 'static>(f: F) -> Self {
        Self { on_complete: Box::new(f) }
    }
}

pub fn run_hooks(
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneHook, Option::<&SceneOnComplete>), Without<SceneHooked>>,
    scene_manager: Res<SceneSpawner>,
    world: &World,
    meshes: Res<Assets<Mesh>>,
    gltfs: Res<Assets<Gltf>>,
    mut cmds: Commands,
) {
    for (entity, instance, hooked, maybe_on_complete) in &unloaded_instances {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).insert(SceneHooked);
        }
        let entities = scene_manager.iter_instance_entities(**instance);
        for entity_ref in entities.filter_map(|e| world.get_entity(e)) {
            let mut cmd = cmds.entity(entity_ref.id());
            let mesh = entity_ref.get::<Handle<Mesh>>()
                                 .map(|m| meshes.get(m))
                                 .flatten();
            let global_transform = entity_ref.get::<GlobalTransform>();
            let aabb = entity_ref.get::<Aabb>();

            let hook_data = HookData {
                mesh,
                global_transform,
                aabb,
            };
            (hooked.hook)(&entity_ref, &mut cmd, hook_data);
        }
        if let Some(on_complete) = maybe_on_complete {
            (on_complete.on_complete)(&mut cmds, &gltfs);
        }
    }
}

pub struct HookPlugin;
impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) { app.add_system(run_hooks); }
}
