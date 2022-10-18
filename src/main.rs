
//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::{
    prelude::*, 
    render::texture::ImageSettings,
    app::AppExit,
};

mod tile;
use tile::tile::TileBundle;
use tile::map::Map;

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
    app.add_system(keyboard_input_system);
    app.add_startup_system(setup);
    //app.add_system(animate_sprite);
    app.add_system(camera_movement);

    app.add_system(bevy::window::close_on_esc);


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
    //*
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
    //*/
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
    commands.spawn_bundle(Camera2dBundle::default());



    /*
    let texture_handle = asset_server.load("textures/world/base_out_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 32, 32);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1.0, true)));
    // */

    let map = Map::new(
        commands,
        asset_server,
        texture_atlases,
    );


}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    mut exit: EventWriter<AppExit>,
    mut query: Query<(
    &mut AnimationTimer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
    )>,

) {

    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::W) {
    }

    if keyboard_input.just_released(KeyCode::S) {
    }


    if keyboard_input.just_pressed(KeyCode::A) {
    }

    if keyboard_input.just_released(KeyCode::D) {
    }

}


fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
        }

        if keyboard_input.pressed(KeyCode::X) && scale > 1.1 {
            let scale = scale - 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
        }

        transform.translation += time.delta_seconds() * direction * 1000.;
    }
}


fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}