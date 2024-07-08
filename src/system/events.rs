use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

#[derive(Event)]
pub struct CollidedWithStar {
    pub star_entity: Entity,
}
