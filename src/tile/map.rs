


use bevy::{
    prelude::{*},
};

use crate::fixed::StaticSpriteSheetBundle;

use rand::Rng;

enum WorldTiles {
    Invalid = 304,

    LightDirtCenter = 112,
    //LightDirtSpecial= 175,

    DarkDirtCenter = 115,
    //DarkDirtSpecial= 178,

    GrassCenter = 118,
    //GrassSpecial=181,

    WaterCenter = 394,
    //WaterSpecial= 454,



}

#[derive(Debug, Default)]
pub struct Map {
    pub tile_guids: Vec<u32>,
}

impl Map {
    pub fn new(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Map {

    let texture_handle = asset_server.load("textures/world/base_out_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle, 
        Vec2::new(32.0, 32.0), 
        32, 32,
        Some(Vec2::splat(0.0)), Some(Vec2::splat(0.0))
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tile_guids: Vec<u32> = Vec::new();

    let mut rng = rand::thread_rng();

    let size = 512;

    for y in 0..size {
        let world_y = (y as f32) * 32.0;

        for x in 0..size {

            let which_land_type = rng.gen_range(0..5);

            let index: usize = match which_land_type {
                0 => WorldTiles::LightDirtCenter as usize,
                1 => WorldTiles::DarkDirtCenter as usize,
                2 => WorldTiles::GrassCenter as usize,
                3 => WorldTiles::WaterCenter as usize,
                _ => WorldTiles::Invalid as usize,
            };

            let world_x = (x as f32) * 32.0;

            let pos = GlobalTransform::from_translation(Vec3::new(world_x, world_y, 1.0));

            let sprite = TextureAtlasSprite::new( index );

            /*
            commands.spawn_batch()
            // */

            //*

            commands
                .spawn(StaticSpriteSheetBundle {
                    sprite: sprite,
                    texture_atlas: texture_atlas_handle.clone(),
                    global_transform: pos,
                    ..default()
                });
            // */
        }
    }

    Map {
        tile_guids: tile_guids,
        ..Default::default()
    }
    }

    
}


