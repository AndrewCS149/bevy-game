use crate::{BoundaryTrigger, Collider, Enemy, Player, Speed};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy)
            .add_system(enemy_tracking);
    }
}

fn spawn_enemy(mut commands: Commands) {
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
        .insert(Speed(150.0))
        .insert(Enemy)
        .insert(BoundaryTrigger(250.0))
        .insert(Collider);
}

fn enemy_tracking(
    time: Res<Time>,
    mut enemy: Query<(&mut Transform, &Speed, &BoundaryTrigger, With<Enemy>)>,
    mut player: Query<(&mut Transform, With<Player>, Without<Enemy>)>,
) {
    for (mut transform, speed, boundary, _) in enemy.iter_mut() {
        let player_position = player.single_mut().0.translation;

        // only start tracking if within the set boundary trigger
        if transform
            .translation
            .abs_diff_eq(player_position, boundary.0)
        {
            let mut new_position = Vec3::new(1.0, 1.0, 0.0);

            if transform.translation.x > player_position.x {
                new_position.x = -1.0;
            }

            if transform.translation.y > player_position.y {
                new_position.y = -1.0;
            }

            transform.translation += new_position * speed.0 * time.delta_seconds();
        }
    }
}
