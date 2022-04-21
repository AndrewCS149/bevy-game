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
    let player_pos = player.single().1.translation;

    let player_size = match player.single().0.custom_size {
        Some(vec) => vec,
        None => todo!(),
    };

    for (enemy_sprite, mut enemy_transform, _, _, _) in enemy.iter_mut() {
        let enemy_size = match enemy_sprite.custom_size {
            Some(vec) => vec,
            None => todo!(),
        };

        let enemy_pos = enemy_transform.translation;

        // this gives the collision space more pixels to hit. If it were just one, the collision would almost never happen
        let half_player_x = player_size.x / 2.0;
        let half_player_y = player_size.y / 2.0;

        let half_enemy_x = enemy_size.x / 2.0;
        let half_enemy_y = enemy_size.y / 2.;

        // RIGHT
        if enemy_pos.x - half_enemy_x <= player_pos.x + half_player_x
            && enemy_pos.x - half_enemy_x >= player_pos.x + half_player_x - half_player_x
            && enemy_pos.y - half_enemy_y <= player_pos.y + half_player_y
            && enemy_pos.y + half_enemy_y >= player_pos.y - half_player_y
        {
            enemy_transform.translation.x = player_pos.x + (player_size.x + enemy_size.x) / 2.0;
        }
        // LEFT
        else if enemy_pos.x + half_enemy_x >= player_pos.x - half_player_x
            && enemy_pos.x + half_enemy_x <= player_pos.x - half_player_x + half_player_x
            && enemy_pos.y - half_enemy_y <= player_pos.y + half_player_y
            && enemy_pos.y + half_enemy_y >= player_pos.y - half_player_y
        {
            enemy_transform.translation.x = player_pos.x - (player_size.x + enemy_size.x) / 2.0;
        }
        // TOP
        else if enemy_pos.x + half_enemy_x >= player_pos.x - half_player_x
            && enemy_pos.x - half_enemy_x <= player_pos.x + half_player_x
            && enemy_pos.y - half_enemy_y <= player_pos.y + half_player_y
            && enemy_pos.y - half_enemy_y >= player_pos.y + half_player_y - half_player_y
        {
            enemy_transform.translation.y = player_pos.y + (player_size.y + enemy_size.y) / 2.0;
        }
        // BOTTOM
        else if enemy_pos.x + half_enemy_x >= player_pos.x - half_player_x
            && enemy_pos.x - half_enemy_x <= player_pos.x + half_player_x
            && enemy_pos.y + half_enemy_y >= player_pos.y - half_player_y
            && enemy_pos.y + half_enemy_y <= player_pos.y - half_player_y + half_player_y
        {
            enemy_transform.translation.y = player_pos.y - (player_size.y + enemy_size.y) / 2.0;
        }
    }
}
