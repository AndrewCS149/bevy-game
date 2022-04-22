use bevy::prelude::*;

use crate::{Enemy, HealthBar};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_healthbar)
            .add_system(move_healthbar);
    }
}

fn spawn_healthbar(mut commands: Commands, enemy: Query<(&Transform, &Sprite, With<Enemy>)>) {
    if let Some((enemy_pos, enemy_size, _)) = enemy.iter().next() {
        let healthbar_x = enemy_pos.translation.x;
        let healthbar_y = (enemy_pos.translation.y + enemy_size.custom_size.unwrap().y / 2.0) + 5.0;

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.48, 0.98, 0.0),
                    custom_size: Some(Vec2::new(30.0, 3.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(healthbar_x, healthbar_y, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(HealthBar(100));
    }
}

fn move_healthbar(
    mut healthbar: Query<(&mut Transform, With<HealthBar>)>,
    enemy: Query<(&Transform, With<Enemy>, Without<HealthBar>)>,
) {
    let enemy_pos = enemy.single().0.translation;
    healthbar.single_mut().0.translation = Vec3::new(enemy_pos.x, enemy_pos.y + 18.0, 0.0);
}
