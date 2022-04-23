use crate::{BoundaryTrigger, Collider, Damage, Enemy, Health, Player, Projectile, Speed};
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
            color: Color::GREEN,
            custom_size: Some(Vec2::new(30.0, 3.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            ..default()
        },
        ..default()
    };

    let enemy = SpriteBundle {
        sprite: Sprite {
            color: Color::TOMATO,
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(200.0, 200.0, 0.0),
            ..default()
        },
        ..default()
    };

    commands
        .spawn_bundle(enemy)
        .with_children(|parent| {
            parent.spawn_bundle(healthbar);
        })
        .insert(Enemy)
        .insert(Health(100))
        .insert(Speed(150.0))
        .insert(Collider)
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

            let calc_new_pos = |e_pos: f32, p_pos: f32| match p_pos {
                num if e_pos > num + buff => -1.0,
                num if e_pos < num - buff => 1.0,
                _ => 0.0,
            };

            new_pos.x = calc_new_pos(enemy_pos.x, player_pos.x);
            new_pos.y = calc_new_pos(enemy_pos.y, player_pos.y);

            enemy_transform.translation += new_pos * enemy_speed.0 * time.delta_seconds();
        }
    }
}

fn enemy_death(
    mut commands: Commands,
    projectile: Query<(Entity, &Transform, &Damage, With<Projectile>)>,
    mut enemy: Query<(
        Entity,
        &Transform,
        &Sprite,
        &mut Health,
        With<Enemy>,
        Without<Projectile>,
    )>,
) {
    if let Some((projectile, projectile_pos, damage, _)) = projectile.iter().next() {
        for (enemy, enemy_pos, sprite, mut health, _, _) in enemy.iter_mut() {
            if enemy_pos.translation.distance(projectile_pos.translation)
                < sprite.custom_size.unwrap().x / 2.0
            {
                commands.entity(projectile).despawn();

                health.0 -= damage.0;

                if health.0 <= 0 {
                    commands.entity(enemy).despawn_recursive();
                }
            }
        }
    }
}
