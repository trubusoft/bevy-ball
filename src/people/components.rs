use bevy::prelude::Component;

#[derive(Component)]
pub struct Person {
    pub(crate) name: String,
}

#[derive(Component)]
pub struct Employment {
    pub(crate) job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
