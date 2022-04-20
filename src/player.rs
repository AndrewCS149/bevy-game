use crate::{Collider, Player, Speed, Sprint};
use bevy::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            ..default()
        })
        .insert(Speed(200.))
        .insert(Sprint(2.))
        .insert(Player);
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &Speed, &Sprint, With<Player>)>,
) {
    for (mut transform, speed, sprint, _) in player.iter_mut() {
        let mut tmp_sprint = 1.;
        let mut new_position = Vec3::new(0.0, 0.0, 0.0);

        // left
        if keys.pressed(KeyCode::A) {
            new_position.x = -1.0;
        }

        // right
        if keys.pressed(KeyCode::D) {
            new_position.x = 1.0;
        }

        // up
        if keys.pressed(KeyCode::W) {
            new_position.y = 1.0;
        }

        // down
        if keys.pressed(KeyCode::S) {
            new_position.y = -1.0;
        }

        // sprint
        if keys.pressed(KeyCode::LShift) {
            tmp_sprint = sprint.0;
        }

        transform.translation += new_position * speed.0 * tmp_sprint * time.delta_seconds();
    }
}
