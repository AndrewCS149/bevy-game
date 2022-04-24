use crate::{Damage, Direction, IsSprinting, Player, Projectile, Speed};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fire_projectile)
            .add_system(projectile_movement)
            .add_system(despawn_projectile);
    }
}

fn fire_projectile(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    player: Query<(&Transform, &Direction, &IsSprinting, With<Player>)>,
) {
    let sprinting = player.single().2 .0;
    if keys.just_pressed(KeyCode::Space) && !sprinting {
        let player_pos = player.single().0.translation;

        let projectile = SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
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
            .insert(Damage(25.0))
            .insert(Speed(1000.0));
    }
}

fn projectile_movement(
    time: Res<Time>,
    mut projectile: Query<(&mut Transform, &Direction, &Speed, With<Projectile>)>,
) {
    for (mut transform, direction, speed, _) in projectile.iter_mut() {
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
    }
}

// despawn the projectile if it is outside of the window bounds
fn despawn_projectile(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    projectile: Query<(Entity, &Transform, With<Projectile>)>,
) {
    for (projectile, transform, _) in projectile.iter() {
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
