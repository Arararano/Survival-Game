use bevy::{prelude::*, utils::HashSet};

pub struct Chunk {
    pub id: (i32,i32),
    pub map: HashSet<(usize, usize)>,
    pub is_rendered: bool,
    
}

pub struct Tile {
    pub id: (usize, usize),
    pub path: str,
}


#[derive(Resource, Default)]
pub struct World {
    // size, chunk_size
    pub chunks: Vec<Chunk>,
    pub seed: u32,
}

#[derive(Resource)]
pub struct Money(pub f32);


// Components ----------------------------------------------------------------------------------------


#[derive(Component)]
pub struct Player {
    pub speed: f32,
}
