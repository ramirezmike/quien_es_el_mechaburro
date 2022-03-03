use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct WorldCollisionPlugin;
impl Plugin for WorldCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Collidables::default())
            .add_system(store_collidables);
    }
}

#[derive(Default)]
pub struct Collidables {
    aabbs: HashMap<Entity, WorldAabb>,
    load_attempts: usize,
}

impl Collidables {
    pub fn reset(&mut self) {
        self.aabbs = HashMap::new();
        self.load_attempts = 0;
    }

    pub fn get_random_spot(&self) -> Option<Vec2> {
        use rand::Rng;
        let aabbs: Vec<_> = self.aabbs.iter().collect();
        let mut rng = rand::thread_rng();

        if let Some((_, aabb)) = aabbs.choose(&mut rng) {
            let x: f32 = rng.gen_range(aabb.min.x..aabb.max.x);
            let z: f32 = rng.gen_range(aabb.min.z..aabb.max.z);
            Some(Vec2::new(x, z))
        } else {
            None
        }
    }

    pub fn is_walkable(&self, x: f32, z: f32) -> bool {
        if self.aabbs.is_empty() {
            return true;
        }

        for (_, aabb) in self.aabbs.iter() {
            if x <= aabb.max.x && x >= aabb.min.x && z <= aabb.max.z && z >= aabb.min.z {
                return true;
            }
        }

        false
    }

    pub fn fit_in(&self, current: &Vec3, new: &mut Vec3, velocity: &mut Vec3) {
        if self.aabbs.is_empty() {
            return;
        }

        let mut is_valid = false;
        let mut current_aabbs = vec![];
        for (_, aabb) in self.aabbs.iter() {
            if new.x <= aabb.max.x
                && new.x >= aabb.min.x
                && new.z <= aabb.max.z
                && new.z >= aabb.min.z
            {
                is_valid = true;
            }

            if current.x <= aabb.max.x
                && current.x >= aabb.min.x
                && current.z <= aabb.max.z
                && current.z >= aabb.min.z
            {
                current_aabbs.push(aabb);
            }
        }

        if is_valid {
            return;
        }

        let mut temp_new = new.clone();
        if !current_aabbs.is_empty() {
            let aabb = current_aabbs[0];
            if temp_new.x < aabb.min.x {
                temp_new.x = aabb.min.x;
            } else if temp_new.x > aabb.max.x {
                temp_new.x = aabb.max.x;
            }

            if temp_new.z < aabb.min.z {
                temp_new.z = aabb.min.z;
            } else if temp_new.z > aabb.max.z {
                temp_new.z = aabb.max.z;
            }
        } else {
            temp_new = *current;
        }

        // All this allows sliding against walls
        let x_changed = temp_new.x != new.x;
        let z_changed = temp_new.z != new.z;
        let get_sign = |num: f32| {
            let sign = num.signum();
            if sign.is_nan() {
                1.0
            } else {
                sign
            }
        };

        match (x_changed, z_changed) {
            (true, true) => {
                velocity.x = 0.0;
                velocity.z = 0.0;
            }
            (true, false) => {
                velocity.z = get_sign(velocity.z) * f32::max(velocity.z.abs(), velocity.x.abs());
                velocity.x = 0.0;
            }
            (false, true) => {
                velocity.x = get_sign(velocity.x) * f32::max(velocity.z.abs(), velocity.x.abs());
                velocity.z = 0.0;
            }
            _ => (),
        }

        *new = temp_new;
    }
}

#[derive(Debug, Default)]
struct WorldAabb {
    min: Vec3,
    max: Vec3,
}

fn store_collidables(
    collidable_query: Query<(Entity, Option<&Aabb>, &GlobalTransform)>,
    mut collidables: ResMut<Collidables>,
) {
    if collidables.load_attempts > 60 {
        return;
    }
    collidables.load_attempts += 1;

    for (entity, maybe_aabb, global_transform) in collidable_query.iter() {
        if !collidables.aabbs.contains_key(&entity) {
            if let Some(aabb) = maybe_aabb {
                let matrix = global_transform.compute_matrix();

                // TODO: look into the bevy blender plugin
                // this is a hack to mark walkable areas by
                // checking if their mesh is below 0.0.
                if matrix.transform_point3(aabb.center).y < 0.0 {
                    collidables.aabbs.insert(
                        entity,
                        WorldAabb {
                            min: matrix.transform_point3(aabb.min()),
                            max: matrix.transform_point3(aabb.max()),
                        },
                    );
                }
            }
        }
    }
}
