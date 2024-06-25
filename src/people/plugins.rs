use bevy::app::{App, Plugin, Startup, Update};

use crate::people::systems::{
    print_all_person, print_person_with_job, print_person_with_their_job_name,
    print_person_without_job, spawn_person, spawn_person_with_job,
};

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_person)
            .add_systems(Startup, spawn_person_with_job)
            .add_systems(Update, print_all_person)
            .add_systems(Update, print_person_with_job)
            .add_systems(Update, print_person_without_job)
            .add_systems(Update, print_person_with_their_job_name);
    }
}
