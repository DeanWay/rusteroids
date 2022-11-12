mod constants;
mod cooldown;
mod player;
use crate::player::PlayerPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(Camera2dBundle::default());
}
