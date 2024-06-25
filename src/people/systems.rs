use bevy::prelude::{Commands, Query, With, Without};

use crate::people::components::{Employment, Job, Person};

pub fn spawn_person(mut commands: Commands) {
    commands.spawn(Person {
        name: "Alex".to_string(),
    });
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn(Person {
        name: "Charlie".to_string(),
    });
    commands.spawn(Person {
        name: "David".to_string(),
    });
    commands.spawn(Person {
        name: "Elen".to_string(),
    });
}

pub fn spawn_person_with_job(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Faye".to_string(),
        },
        Employment { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "George".to_string(),
        },
        Employment {
            job: Job::FireFighter,
        },
    ));
    commands.spawn((
        Person {
            name: "Hayden".to_string(),
        },
        Employment { job: Job::Lawyer },
    ));
    commands.spawn((
        Person {
            name: "Isaac".to_string(),
        },
        Employment { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "Jade".to_string(),
        },
        Employment {
            job: Job::FireFighter,
        },
    ));
}

pub fn print_all_person(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Person name: {}", person.name);
    }
}

pub fn print_person_with_job(person_query: Query<&Person, With<Employment>>) {
    for person in person_query.iter() {
        println!("Person name: {} has a job", person.name);
    }
}

pub fn print_person_without_job(person_query: Query<&Person, Without<Employment>>) {
    for person in person_query.iter() {
        println!("Person name: {} is ready for employment", person.name);
    }
}

pub fn print_person_with_their_job_name(person_employment_query: Query<(&Person, &Employment)>) {
    for (person, employment) in person_employment_query.iter() {
        println!(
            "Person name: {} has a job as {:?}",
            person.name, employment.job
        );
    }
}
