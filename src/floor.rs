use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use rand::seq::SliceRandom;

pub struct FloorPlugin;
impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FloorManager::default());
    }
}

#[derive(Default, Resource)]
pub struct FloorManager {
    floors: Vec::<Floor>,
}

struct Floor {
    min: Vec3,
    max: Vec3,
}

impl FloorManager {
    pub fn clear(&mut self) {
        self.floors.clear();
    }

    pub fn store_floor(&mut self, global_transform: &GlobalTransform, aabb: &Aabb) {
        let matrix = global_transform.compute_matrix();
        self.floors.push(
            Floor {
                min: matrix.transform_point3(aabb.min().into()),
                max: matrix.transform_point3(aabb.max().into()),
            }
        );
    }

    pub fn get_random_spot(&self) -> Option<Vec2> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        if let Some(floor) = self.floors.choose(&mut rng) {
            let x: f32 = rng.gen_range(floor.min.x..floor.max.x);
            let z: f32 = rng.gen_range(floor.min.z..floor.max.z);
            println!("Random spot {} {}", x, z);
            Some(Vec2::new(x, z))
        } else {
            None
        }
    }

    pub fn is_walkable(&self, x: f32, z: f32) -> bool {
        if self.floors.is_empty() {
            return true;
        }

        for floor in self.floors.iter() {
            if x <= floor.max.x && x >= floor.min.x && z <= floor.max.z && z >= floor.min.z {
                return true;
            }
        }

        false
    }
}
