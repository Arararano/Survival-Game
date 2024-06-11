use bevy::input::keyboard::KeyCode;
use bevy::{    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use player::PlayerPlugin;
use tree::TreePlugin;
use world_gen::WorldGenPlugin;


use crate::types::*;
// Resources
// Components


// Plugins
mod config;
mod tree;
mod world_gen;
mod player;

mod types;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(PanCamPlugin)
        .insert_resource(Money(100.0))
        .insert_resource(Msaa::Off)
        .add_plugins(PlayerPlugin)
        .add_plugins(TreePlugin)
        .add_plugins(WorldGenPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb(128.0 / 255.0, 204.0 / 255.0, 1.0)),
            ..default()
        },
        ..default()
    };

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera).insert(PanCam::default());

}


