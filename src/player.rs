use crate::{Direction, IsSprinting, Player, Speed, Sprint};
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
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            ..default()
        })
        .insert(Speed(200.0))
        .insert(Sprint(2.0))
        .insert(Player)
        .insert(IsSprinting(false))
        .insert(Direction("R".to_string()));
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<(
        &mut Transform,
        &Speed,
        &Sprint,
        &mut Direction,
        &mut IsSprinting,
        With<Player>,
    )>,
) {
    for (mut transform, speed, sprint, mut direction, mut is_sprinting, _) in player.iter_mut() {
        let mut tmp_sprint = 1.;
        let mut new_pos = Vec3::new(0.0, 0.0, 0.0);

        // left
        if keys.pressed(KeyCode::A) {
            new_pos.x = -1.0;
            direction.0 = "L".to_string();
        }

        // right
        if keys.pressed(KeyCode::D) {
            new_pos.x = 1.0;
            direction.0 = "R".to_string();
        }

        // up
        if keys.pressed(KeyCode::W) {
            new_pos.y = 1.0;
            direction.0 = "U".to_string();
        }

        // down
        if keys.pressed(KeyCode::S) {
            new_pos.y = -1.0;
            direction.0 = "D".to_string();
        }

        // diagonals
        // UR
        if keys.pressed(KeyCode::W) && keys.pressed(KeyCode::D) {
            direction.0 = "UR".to_string();
        }
        // UL
        if keys.pressed(KeyCode::W) && keys.pressed(KeyCode::A) {
            direction.0 = "UL".to_string();
        }
        // DR
        if keys.pressed(KeyCode::S) && keys.pressed(KeyCode::D) {
            direction.0 = "DR".to_string();
        }
        // DL
        if keys.pressed(KeyCode::S) && keys.pressed(KeyCode::A) {
            direction.0 = "DL".to_string();
        }

        // sprint
        if keys.pressed(KeyCode::LShift) {
            tmp_sprint = sprint.0;
            is_sprinting.0 = true;
        } else {
            is_sprinting.0 = false;
        }

        transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
    }
}
