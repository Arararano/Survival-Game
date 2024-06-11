use bevy::prelude::*;
use bevy::reflect::{Array, DynamicTypePath, List};
use bevy::utils::HashSet;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::config::*;
use crate::types::{Chunk, Player, World};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        let seed_rng = rand::thread_rng().gen_range(10000..99999);

        app.add_systems(Update, world_gen).insert_resource(World {
            seed: seed_rng,
            chunks: vec![],
        });
        app.add_systems(Update, chunk_display);
    }
}

fn world_gen(mut world: ResMut<World>, player: Query<&GlobalTransform, With<Player>>) {
    let player = player.single().translation();
    let player_chunk_coords: (i32, i32) = (
        ((player.x / TILE_W as f32).floor() / CHUNK_W as f32).floor() as i32,
        ((player.y / TILE_H as f32).floor() / CHUNK_H as f32).floor() as i32,
    );

    let mut render_distance_vec: Vec<(i32, i32)> = vec![];

    for i in (-1 * RENDER_DISTANCE)..RENDER_DISTANCE {
        for j in (-1 * RENDER_DISTANCE)..RENDER_DISTANCE {
            render_distance_vec.push((i, j));
        }
    }

    for chunk_coords in render_distance_vec {
        let (mut render_x, mut render_y) = chunk_coords;
        let (offset_x, offset_y) = player_chunk_coords;

        //info!("Player Chunk Coords: {:?}", player_chunk_coords);

        render_x += offset_x;
        render_y += offset_y;

        //info!("RenderX: {:?} RenderY: {:?}", render_x, render_y);

        let new_chunk_id: (i32, i32) = (render_x, render_y);

        let mut chunk_exists = false;

        for chunk in &world.chunks {
            if chunk.id == new_chunk_id {
                chunk_exists = true;
                break;
            }
        }

        if !chunk_exists {
            let new_chunk: Chunk = Chunk {
                map: chunk_gen(world.seed, (render_x, render_y)),
                id: (render_x, render_y),
                is_rendered: false,
            };

            world.chunks.push(new_chunk);
        }
    }

    info!("Current World Chunks Length: {:?}", world.chunks.len());
}

fn chunk_display(mut world: ResMut<World>, mut commands: Commands, asset_server: Res<AssetServer>) {
    for chunk in &mut world.chunks {
        if !chunk.is_rendered {
            for (x, y) in &chunk.map {
                let tile_path = match get_tile((*x as u32, *y as u32), chunk.map.clone()) {
                    1 => "corner1.png",
                    2 => "corner2.png",
                    3 => "corner3.png",
                    4 => "corner4.png",
                    5 => "grass.png",
                    _ => {
                        //info!("werid case");
                        "grass.png"
                    }
                };

                let texture = asset_server.load(tile_path);

                let (mut rx, mut ry) = grid_to_world(*x as f32, *y as f32);

                let offset_x = chunk.id.0 * CHUNK_W * TILE_W;
                let offset_y = chunk.id.1 * CHUNK_H * TILE_H;

                rx += offset_x as f32;
                ry += offset_y as f32;

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_W as f32, TILE_H as f32)),
                            ..default()
                        },
                        texture,
                        transform: Transform::from_xyz(rx, ry, 0.0),
                        ..default()
                    },
                    Name::new("World Tile"),
                ));
            }
            chunk.is_rendered = true;

            //info!("Chunk ID: {:?}", chunk.id);
        }
    }
}

fn chunk_gen(seed: u32, offset: (i32, i32)) -> HashSet<(usize, usize)> {
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();

    let perlin = Perlin::new(seed);

    let offset_x = offset.0 * CHUNK_W;
    let offset_y = offset.1 * CHUNK_H;

    for x in (0)..(CHUNK_W) {
        for y in (0)..(CHUNK_H) {
            let value = perlin.get([
                (x as f64 + offset_x as f64 + 0.3) / PERLIN_SCALE,
                (y as f64 + offset_y as f64 + 0.3) / PERLIN_SCALE,
            ]);

            // info!("Current Perlin Values: {:?}", value);

            if value < PERLIN_CUTOFF {
                continue;
            }

            tiles.insert((x as usize, y as usize));
        }
    }

    let mut tiles_to_remove: Vec<(usize, usize)> = vec![];
    for (x, y) in &tiles {
        if get_tile((*x as u32, *y as u32), tiles.clone()) == -1 {
            tiles_to_remove.push((*x, *y));
        }
    }

    for tile_to_remove in tiles_to_remove {
        tiles.remove(&tile_to_remove);
    }

    return tiles;
}

fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (x * TILE_W as f32, y * TILE_H as f32)
}

fn get_tile((x, y): (u32, u32), occupied: HashSet<(usize, usize)>) -> i32 {
    let nei_options: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut nei = [1, 1, 1, 1];

    for (idx, options) in nei_options.iter().enumerate() {
        let (j, k) = <(i32, i32) as FromReflect>::from_reflect(&*options).unwrap(); // OK!
                                                                                    //
                                                                                    //
                                                                                    //info!("J: {:?} K: {:?}", j,k);

        let x_index = x as i32 + j;
        let y_index = y as i32 + k;

        if occupied.contains(&(x_index as usize, y_index as usize)) {
            continue;
        }

        nei[idx] = 0
    }

    // info!("Nei: {:?}", nei);
    return match nei {
        [1, 1, 0, 0] => 1,
        [0, 1, 1, 0] => 2,
        [0, 0, 1, 1] => 3,
        [1, 0, 0, 1] => 4,
        [1, 1, 1, 1] => 5,
        [0, 0, 0, 0] => 0,

        [1, 0, 0, 0] => -1,
        [0, 1, 0, 0] => -1,
        [0, 0, 1, 0] => -1,
        [0, 0, 0, 1] => -1,
        _ => {
            //info!("Some voodoo shit in match for get_tile");
            -2
        }
    };
}
