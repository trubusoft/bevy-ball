use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct PlayerDead {
    pub score: u32,
}

#[derive(Event)]
pub struct CollidedWithStar {
    pub star_entity: Entity,
}
