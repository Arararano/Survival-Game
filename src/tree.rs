use bevy::prelude::*;

use crate::types::{Money, Player};


pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_tree, tree_lifetime));
    }
}


#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tree {
    pub liftime: Timer,
}

fn tree_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut trees: Query<(Entity, &mut Tree)>,
    mut money: ResMut<Money>,
) {
    for (tree_entity, mut tree) in &mut trees {
        tree.liftime.tick(time.delta());

        if tree.liftime.finished() {
            money.0 += 15.0;

            commands.entity(tree_entity).despawn();

            info!("Tree sold for $15! Current Money: ${:?}", money.0);
        }
    }
}

fn spawn_tree(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a tree, remaining money: ${:?}", money.0);

        let texture = asset_server.load("tree.png");

        command.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                texture,
                transform: *player_transform,

                ..default()
            },
            Tree {
                liftime: Timer::from_seconds(10.0, TimerMode::Once),
            },
        ));
    }
}
