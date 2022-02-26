use bevy::prelude::*;

pub struct BurroPlugin;
impl Plugin for BurroPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_burros.label("handle_burros"))
            .add_system(handle_burro_death_events.after("handle_burros"))
            .add_event::<BurroDeathEvent>();
    }
}

struct BurroDeathEvent {
    entity: Entity,
}

#[derive(Component)]
pub struct Burro {
    pub max_health: usize,
    pub health: usize,
    pub bullet_speed: f32,
    pub bullet_time_alive: f32,
    pub fire_cooldown: f32,
}

impl Default for Burro {
    fn default() -> Self {
        Burro {
            max_health: 3,
            health: 3,
            bullet_speed: 6.0,
            bullet_time_alive: 3.0,
            fire_cooldown: 0.0,
        }
    }
}

impl Burro {
    pub fn can_fire(&self) -> bool {
        self.fire_cooldown <= 0.0
    }

    pub fn fire(&mut self) {
        self.fire_cooldown = 0.4;
    }

    pub fn hit(&mut self) {
        if let Some(health) = self.health.checked_sub(1) {
            self.health = health;
            println!("Hit burro! Health {}", self.health);
        }
    }

    pub fn full_health(&self) -> bool {
        self.health == self.max_health
    }

    pub fn almost_dead(&self) -> bool {
        self.health == 1
    }
}

fn handle_burros(
    time: Res<Time>,
    mut burros: Query<(Entity, &mut Burro)>,
    mut burro_death_event_writer: EventWriter<BurroDeathEvent>,
) {
    for (entity, mut burro) in burros.iter_mut() {
        // handling burro deaths
        if burro.health == 0 {
            burro_death_event_writer.send(BurroDeathEvent { entity });
            continue;
        }

        // handling firing cool down
        burro.fire_cooldown -= time.delta_seconds();
        burro.fire_cooldown = burro.fire_cooldown.clamp(-10.0, 3.0);
    }
}

fn handle_burro_death_events(
    mut commands: Commands,
    mut burro_death_event_reader: EventReader<BurroDeathEvent>,
) {
    for death_event in burro_death_event_reader.iter() {
        commands.entity(death_event.entity).despawn_recursive();
        // probably do a bunch of UI/animation stuff here and play sounds or something
    }
}
