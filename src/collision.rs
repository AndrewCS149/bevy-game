use crate::{Collider, Enemy, Player};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_system);
    }
}

fn collision_system(
    player: Query<(&Sprite, &Transform, With<Player>)>,
    mut enemy: Query<(&Sprite, &mut Transform, With<Enemy>, Without<Player>)>,
) {
    let player_position = player.single().1.translation;

    let player_size = match player.single().0.custom_size {
        Some(vec) => vec,
        None => todo!(),
    };

    for (sprite, mut transform, _, _) in enemy.iter_mut() {
        let enemy_size = match sprite.custom_size {
            Some(vec) => vec,
            None => todo!(),
        };

        if transform.translation.x - (enemy_size.x) < player_position.x + (player_size.x / 2.0) {
            transform.translation.x = player_position.x + player_size.x;
        }
    }

    // if enemy_position.x - (enemy_size.x / 2.0) <= player_position.x + (player_size.x / 2.0) {
    //     enemy.single_mut().1.translation.x = 0.0;
    // }
}
