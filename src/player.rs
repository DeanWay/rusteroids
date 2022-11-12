use std::time::Duration;

use crate::constants::{BOUNDS, TIME_STEP};
use crate::cooldown::Cooldown;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, time::FixedTimestep};

const RESISTENCE: f32 = 0.01;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(player_movement_system),
            )
            .add_system(player_fire_system);
    }
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
    movement_acceleration: f32,
    max_speed: f32,
    fire_cooldown: Cooldown,
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_texture = asset_server.load("textures/ship.png");

    // player controlled ship
    commands
        .spawn_bundle(SpriteBundle {
            texture: ship_texture,
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        })
        .insert(Player {
            movement_speed: 0.0,                    // metres per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
            movement_acceleration: 10.0,
            max_speed: option_env!("PLAYER_MAX_SPEED")
                .map(|s| s.parse().unwrap())
                .unwrap_or(250.0),
            fire_cooldown: Cooldown::new(Duration::from_millis(250)),
        });
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement_factor -= 0.25;
    }
    let slow_multiplier = 1.0 - RESISTENCE;
    let acceleration = movement_factor * ship.movement_acceleration;
    let new_speed = (ship.movement_speed + acceleration) * slow_multiplier;
    ship.movement_speed = new_speed.min(ship.max_speed).max(0.0);
    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * TIME_STEP);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = ship.movement_speed * TIME_STEP;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}

fn player_fire_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Player>,
    missile_query: Query<Entity, &Missile>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut ship = player_query.single_mut();
    ship.fire_cooldown.tick(time.delta());
    if ship.fire_cooldown.available() {
        for missile_entity in missile_query.iter() {
            commands.entity(missile_entity).despawn();
        }
    }
    if keyboard_input.pressed(KeyCode::Space) && ship.fire_cooldown.available() {
        ship.fire_cooldown.trigger();
        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform::default().with_scale(Vec3::splat(10.)),
                material: materials.add(ColorMaterial::from(Color::RED)),
                ..default()
            })
            .insert(Missile {});
    }
}

#[derive(Component)]
struct Missile {}
