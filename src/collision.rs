use crate::{Collider, Enemy, Player};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_system);
    }
}

fn collision_system(
    time: Res<Time>,
    player: Query<(&Sprite, &Transform, With<Player>)>,
    mut enemy: Query<(&Sprite, &mut Transform, With<Enemy>, Without<Player>)>,
) {
    let player_position = player.single().1.translation;

    let player_size = match player.single().0.custom_size {
        Some(vec) => vec,
        None => todo!(),
    };

    for (enemy_sprite, mut enemy_transform, _, _) in enemy.iter_mut() {
        let enemy_size = match enemy_sprite.custom_size {
            Some(vec) => vec,
            None => todo!(),
        };

        let y_range =
            (player_position.y - player_size.y / 2.0)..=(player_position.y + player_size.y / 2.0);

        let x_range =
            (player_position.x - player_size.x / 2.0)..=(player_position.x + player_size.x / 2.0);

        println!("{}", player_position.distance(enemy_transform.translation));
        // right and left collision
        let y_range =
            (player_position.y - player_size.y / 2.0)..=(player_position.y + player_size.y / 2.0);
        if y_range.contains(&enemy_transform.translation.y) {
            // right
            if enemy_transform.translation.x - enemy_size.x / 2.0
                <= player_position.x + player_size.x / 2.0
            {
                enemy_transform.translation.x =
                    player_position.x + (player_size.x + enemy_size.x) / 2.0;
            }
            // left
            // if enemy_transform.translation.x + enemy_size.x / 2.0
            //     >= player_position.x - player_size.x / 2.0
            // {
            //     enemy_transform.translation.x =
            //         player_position.x - (player_size.x / 2.0 + enemy_size.x / 2.0);
            // }
        }
    }
}
