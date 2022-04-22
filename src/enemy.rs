use crate::{BoundaryTrigger, Collider, Enemy, Player, Projectile, Speed};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy)
            .add_system(enemy_tracking)
            .add_system(enemy_death);
    }
}

fn spawn_enemy(mut commands: Commands) {
    let healthbar = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.48, 0.98, 0.0),
            custom_size: Some(Vec2::new(30.0, 3.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(200.0, 240.0, 0.0),
            ..default()
        },
        ..default()
    };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.25),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(200.0, 200.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Collider)
        .insert(Speed(150.0))
        .insert(BoundaryTrigger(250.0));
}

fn enemy_tracking(
    time: Res<Time>,
    mut enemy: Query<(&mut Transform, &Speed, &BoundaryTrigger, With<Enemy>)>,
    mut player: Query<(&mut Transform, With<Player>, Without<Enemy>)>,
) {
    for (mut enemy_transform, enemy_speed, boundary, _) in enemy.iter_mut() {
        let player_pos = player.single_mut().0.translation;

        // only start tracking if within the set boundary trigger
        if enemy_transform
            .translation
            .abs_diff_eq(player_pos, boundary.0)
        {
            let enemy_pos = enemy_transform.translation;
            let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
            let buff = 0.25;

            new_pos.x = match player_pos.x {
                num if enemy_pos.x > num + buff => -1.0,
                num if enemy_pos.x < num - buff => 1.0,
                _ => 0.0,
            };

            new_pos.y = match player_pos.y {
                num if enemy_pos.y > num + buff => -1.0,
                num if enemy_pos.y < num - buff => 1.0,
                _ => 0.0,
            };

            enemy_transform.translation += new_pos * enemy_speed.0 * time.delta_seconds();
        }
    }
}

fn enemy_death(
    mut commands: Commands,
    projectile: Query<(Entity, &Transform, With<Projectile>)>,
    enemy: Query<(
        Entity,
        &Transform,
        &Sprite,
        With<Enemy>,
        Without<Projectile>,
    )>,
) {
    if let Some((projectile, projectile_pos, _)) = projectile.iter().next() {
        for (enemy, enemy_pos, sprite, _, _) in enemy.iter() {
            if enemy_pos.translation.distance(projectile_pos.translation)
                < sprite.custom_size.unwrap().x / 2.0
            {
                commands.entity(enemy).despawn();
                commands.entity(projectile).despawn();
            }
        }
    }
}
