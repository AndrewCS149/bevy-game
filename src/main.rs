use bevy::prelude::*;

mod collision;
mod enemy;
mod player;
mod projectile;
mod setup;

use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(ProjectilePlugin)
        .run();
}

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct IsSprinting(bool);

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

#[derive(Component)]
struct Damage(f32);
