use crate::{Collider, Enemy, Player};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision);
    }
}

fn collision(
    player: Query<(&Sprite, &Transform, With<Player>)>,
    mut enemy: Query<(
        &Sprite,
        &mut Transform,
        With<Enemy>,
        With<Collider>,
        Without<Player>,
    )>,
) {
    let player_position = player.single().1.translation;

    let player_size = match player.single().0.custom_size {
        Some(vec) => vec,
        None => todo!(),
    };

    for (enemy_sprite, mut enemy_transform, _, _, _) in enemy.iter_mut() {
        let enemy_size = match enemy_sprite.custom_size {
            Some(vec) => vec,
            None => todo!(),
        };

        // RIGHT
        if enemy_transform.translation.x - enemy_size.x / 2.0
            <= player_position.x + player_size.x / 2.0
            && enemy_transform.translation.x - enemy_size.x / 2.0
                >= player_position.x + player_size.x / 2.0 - 10.0
            && enemy_transform.translation.y - enemy_size.y / 2.0
                <= player_position.y + player_size.y / 2.0
            && enemy_transform.translation.y + enemy_size.y / 2.0
                >= player_position.y - player_size.y / 2.0
        {
            enemy_transform.translation.x =
                player_position.x + (player_size.x + enemy_size.x) / 2.0;
        }
        // LEFT
        else if enemy_transform.translation.x + enemy_size.x / 2.0
            >= player_position.x - player_size.x / 2.0
            && enemy_transform.translation.x + enemy_size.x / 2.0
                <= player_position.x - player_size.x / 2.0 + 10.0
            && enemy_transform.translation.y - enemy_size.y / 2.0
                <= player_position.y + player_size.y / 2.0
            && enemy_transform.translation.y + enemy_size.y / 2.0
                >= player_position.y - player_size.y / 2.0
        {
            enemy_transform.translation.x =
                player_position.x - (player_size.x + enemy_size.x) / 2.0;
        }
        // TOP
        else if enemy_transform.translation.x + enemy_size.x / 2.0
            >= player_position.x - player_size.x / 2.0
            && enemy_transform.translation.x - enemy_size.x / 2.0
                <= player_position.x + player_size.x / 2.0
            && enemy_transform.translation.y - enemy_size.y / 2.0
                <= player_position.y + player_size.y / 2.0
            && enemy_transform.translation.y - enemy_size.y / 2.0
                >= player_position.y + player_size.y / 2.0 - 10.0
        {
            enemy_transform.translation.y =
                player_position.y + (player_size.y + enemy_size.y) / 2.0;
        }
        // BOTTOM
        else if enemy_transform.translation.x + enemy_size.x / 2.0
            >= player_position.x - player_size.x / 2.0
            && enemy_transform.translation.x - enemy_size.x / 2.0
                <= player_position.x + player_size.x / 2.0
            && enemy_transform.translation.y + enemy_size.y / 2.0
                >= player_position.y - player_size.y / 2.0
            && enemy_transform.translation.y + enemy_size.y / 2.0
                <= player_position.y - player_size.y / 2.0 + 10.0
        {
            enemy_transform.translation.y =
                player_position.y - (player_size.y + enemy_size.y) / 2.0;
        }
    }
}
