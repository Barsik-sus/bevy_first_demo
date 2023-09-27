use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component, Clone, Copy)]
struct Age(u8);

fn add_people(mut commands: Commands) {
    commands.spawn_batch([
        (Person, Name("Igor".into()), Age(13)),
        (Person, Name("Peter".into()), Age(8)),
        (Person, Name("Oleg".into()), Age(100)),
    ])
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<(&Name, &Age), With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (name, age) in &query {
            println!("hello {}, {} y.o.!", name.0, age.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);

    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}