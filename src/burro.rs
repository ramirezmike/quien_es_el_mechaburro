use bevy::prelude::*;

pub struct BurroPlugin;
impl Plugin for BurroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_burros);
    }
}

#[derive(Component)]
pub struct Burro {
    pub bullet_speed: f32,
    pub bullet_time_alive: f32,
    pub cool_down: f32,
    pub cool_down_limit: f32,
}

impl Default for Burro {
    fn default() -> Self {
        Burro {
            bullet_speed: 6.0,
            bullet_time_alive: 3.0,
            cool_down: 0.0,
            cool_down_limit: 0.4,
        }
    }
}

impl Burro {
    pub fn can_fire(&self) -> bool {
        self.cool_down >= self.cool_down_limit
    }

    pub fn fire(&mut self) {
        self.cool_down = 0.0;
    }
}

fn handle_burros(time: Res<Time>, mut burros: Query<&mut Burro>) {
    for mut burro in burros.iter_mut() {
        // handling firing cool down
        burro.cool_down += time.delta_seconds();
        burro.cool_down = burro.cool_down.clamp(0.0, burro.cool_down_limit + 1.0);
    }
}
