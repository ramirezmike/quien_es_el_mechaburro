use bevy::{
    prelude::*,
    scene::SceneInstance,
    ecs::{world::EntityRef, system::EntityCommands},
};
#[derive(Component, Debug)]
pub struct SceneHooked;

#[derive(Bundle)]
pub struct HookedSceneBundle {
    pub hook: SceneHook,
    #[bundle]
    pub scene: SceneBundle,
}

#[derive(Component)]
pub struct SceneHook {
    hook: Box<dyn Fn(&EntityRef, &mut EntityCommands, Option::<&Mesh>) + Send + Sync + 'static>,
}
impl SceneHook {
    pub fn new<F: Fn(&EntityRef, &mut EntityCommands, Option::<&Mesh>) + Send + Sync + 'static>(hook: F) -> Self {
        Self { hook: Box::new(hook) }
    }
}

pub fn run_hooks(
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneHook), Without<SceneHooked>>,
    scene_manager: Res<SceneSpawner>,
    world: &World,
    meshes: Res<Assets<Mesh>>,
    mut cmds: Commands,
) {
    for (entity, instance, hooked) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            cmds.entity(entity).insert(SceneHooked);
        }
        let entities = scene_manager.iter_instance_entities(**instance);
        for entity_ref in entities.filter_map(|e| world.get_entity(e)) {
            let mut cmd = cmds.entity(entity_ref.id());
            let mesh = entity_ref.get::<Handle<Mesh>>()
                                 .map(|m| meshes.get(m))
                                 .flatten();
            (hooked.hook)(&entity_ref, &mut cmd, mesh);
        }
    }
}

pub struct HookPlugin;
impl Plugin for HookPlugin {
    fn build(&self, app: &mut App) { app.add_system(run_hooks); }
}
