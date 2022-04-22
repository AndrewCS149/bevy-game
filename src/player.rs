use crate::{Direction, Player, Projectile, Speed, Sprint};
use bevy::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(fire_projectile)
            .add_system(projectile_movement)
            .add_system(player_movement);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        })
        .insert(Speed(200.0))
        .insert(Sprint(2.0))
        .insert(Player)
        .insert(Direction("R".to_string()));
}

fn fire_projectile(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    player: Query<(&Transform, &Direction, With<Player>)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let player_pos = player.single().0.translation;

        let projectile = SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.1),
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(player_pos.x, player_pos.y, 0.0),
                ..default()
            },
            ..default()
        };

        commands
            .spawn_bundle(projectile)
            .insert(Direction(player.single().1 .0.clone()))
            .insert(Projectile)
            .insert(Speed(1000.0));
    }
}

fn projectile_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut windows: ResMut<Windows>,
    mut projectile: Query<(Entity, &mut Transform, &Direction, &Speed, With<Projectile>)>,
) {
    for (projectile, mut transform, direction, speed, _) in projectile.iter_mut() {
        let mut new_pos = Vec3::new(0.0, 0.0, 0.0);

        // fire up left
        if direction.0 == "UL".to_string() {
            new_pos.y = 1.0;
            new_pos.x = -1.0;
        // fire up right
        } else if direction.0 == "UR".to_string() {
            new_pos.y = 1.0;
            new_pos.x = 1.0;
        // fire down left
        } else if direction.0 == "DL".to_string() {
            new_pos.y = -1.0;
            new_pos.x = -1.0;
        // fire down right
        } else if direction.0 == "DR".to_string() {
            new_pos.y = -1.0;
            new_pos.x = 1.0;
        // fire right
        } else if direction.0 == "R".to_string() {
            new_pos.x = 1.0;
        // fire left
        } else if direction.0 == "L".to_string() {
            new_pos.x = -1.0;
        // fire up
        } else if direction.0 == "U".to_string() {
            new_pos.y = 1.0;
        // fire down
        } else {
            new_pos.y = -1.0;
        }

        transform.translation += new_pos * time.delta_seconds() * speed.0;

        // despawn the projectile if it is outside of the window bounds
        let window = windows.get_primary_mut().unwrap();
        if transform.translation.x > window.width() / 2.0
            || transform.translation.x < -(window.width() / 2.0)
            || transform.translation.y > window.height() / 2.0
            || transform.translation.y < -(window.height() / 2.0)
        {
            commands.entity(projectile).despawn();
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player: Query<(
        &mut Transform,
        &Speed,
        &Sprint,
        &mut Direction,
        With<Player>,
    )>,
) {
    for (mut transform, speed, sprint, mut direction, _) in player.iter_mut() {
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
        }

        transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
    }
}
