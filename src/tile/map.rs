


use bevy::{
    prelude::{
        default, Bundle, Component, ComputedVisibility, Entity, GlobalTransform, Handle, Image,
        Transform, Visibility, Commands, SpriteSheetBundle, Res, AssetServer, ResMut, Assets,
        TextureAtlas, Vec2, Vec3, TextureAtlasSprite
    },
    render::texture::DEFAULT_IMAGE_HANDLE,
    sprite::Sprite,
};

use rand::Rng;


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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 32, 32);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tile_guids: Vec<u32> = Vec::new();

    let mut rng = rand::thread_rng();


    for y in 0..128 {
        let worldY = (y as f32) * 32.0;

        for x in 0..128 {

            let index: usize = rng.gen_range(0..1024);

            let worldX = (x as f32) * 32.0;

            let pos = Transform::from_translation(Vec3::new(worldX, worldY, 1.0));

            let sprite = TextureAtlasSprite::new( index );


            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: sprite,
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: pos,
                    ..default()
                });
        }
    }

    Map {
        tile_guids: tile_guids,
        ..Default::default()
    }
    }
}


