use crate::{BoundaryTrigger, Collider, Enemy, Player, Projectile, Speed};
use bevy::{core::FixedTimestep, prelude::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(spawn_enemy),
        )
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

    // commands.spawn_bundle(healthbar);
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

            let closure = |e_pos: f32, p_pos: f32| match p_pos {
                num if e_pos > num + buff => -1.0,
                num if e_pos < num - buff => 1.0,
                _ => 0.0,
            };

            new_pos.x = closure(enemy_pos.x, player_pos.x);
            new_pos.y = closure(enemy_pos.y, player_pos.y);

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
