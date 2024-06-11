use bevy::prelude::*;
use bevy::math::vec3;


use crate::types::{World, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {    
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup);
        app.add_systems(Update, character_movement);
    }
}




fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let texture = asset_server.load("wizard.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Name::new("Player"),
        Player { speed: 100.0 },
    ));
}


fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut displacement: Vec3 = vec3(0.0, 0.0, 0.0);

        if input.pressed(KeyCode::KeyW) {
            displacement.y += 1.0; 
        }
        if input.pressed(KeyCode::KeyS) {
            displacement.y -= 1.0; 
        }
        if input.pressed(KeyCode::KeyD) {
            displacement.x += 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            displacement.x -= 1.0;
        }

        let speed_scale = if input.pressed(KeyCode::ShiftLeft) {
    5.0
}
        else {
    1.0
};

        let mut new_pos = transform.translation + displacement.normalize_or_zero()  * player.speed * speed_scale * time.delta_seconds();
        new_pos.z = 1.0;
        
        //if new_pos is not on land then dont let them walk there :3
        //
        
        transform.translation = new_pos;
        
    }
}



fn tile_is_groud(
    world: Res<World>,
    tile: (usize, usize)
) {
    
}
