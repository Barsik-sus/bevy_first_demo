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

fn hello_world() {
    println!("hello world");
}

fn greet_people(query: Query<(&Name, &Age), With<Person>>) {
    for (name, age) in &query {
        println!("hello {}, {} y.o.!", name.0, age.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}