use bevy::prelude::App;

use people::plugins::PeoplePlugin;

mod people;

fn main() {
    App::new().add_plugins(PeoplePlugin).run();
}
