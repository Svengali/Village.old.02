
//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::{prelude::*, render::texture::ImageSettings};

mod tile;
use tile::tile::TileBundle;

/*
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        bundle::{TileBundle}
    };
}
*/

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Invalid,
    Splash,
    Menu,
    Game,
}


fn main() {
    let mut app = App::new();
 
    let tileTest = TileBundle {
        ..Default::default()
    };

    app.insert_resource(ImageSettings::default_nearest()); // prevents blurry sprites
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(setup);
    app.add_system(animate_sprite);
    app.run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    /*
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
    */
}


fn draw_tilemap(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    //*
    mut query: Query<(
        &Sprite,
        &Transform
    )>
    //*/
) {



}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}
