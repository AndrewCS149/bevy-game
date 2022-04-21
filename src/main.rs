use bevy::prelude::*;

mod collision;
mod enemy;
mod player;
mod setup;

use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionPlugin)
        .run();
}
#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Sprint(f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct BoundaryTrigger(f32);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Direction(String);

#[derive(Component)]
struct Projectile;
