
//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::input::mouse::{MouseButtonInput, MouseWheel, MouseScrollUnit, MouseMotion};
//use bevy::log::LogSettings;
use bevy::render::camera::RenderTarget;
use bevy::{
    prelude::*, 
    //render::texture::ImageSettings,
    render::texture::ImageFormat,
    app::AppExit,
};
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContext, EguiPlugin};




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
 
    let tile_test = TileBundle {
        ..Default::default()
    };

    //app.insert_resource(ImageSettings::default_nearest()); // prevents blurry sprites
    app.add_plugins(DefaultPlugins);
    //app.add_plugin(EguiPlugin);
    app.add_system(keyboard_input_system);
    app.add_startup_system(setup);
    //app.add_system(animate_sprite);
    app.add_system(camera_movement);
    app.add_system(mouse_button_events);
    app.add_system(mouse_move_event);
    app.add_system(scroll_events);
    //app.add_system(my_cursor_system);

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
    query: Query<(
        &Sprite,
        &Transform
    )>
    //*/
) {



}


#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);



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

    //*
    let map = Map::new(
        commands,
        asset_server,
        texture_atlases,
    );
    // */
}

/*
fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut egui_context: EguiContext,
    time: Res<Time>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let world_pos = fun_name(screen_pos, window_size, camera_transform, camera);

        egui::Window::new("my_cursor_system")
            .anchor(Align2::LEFT_TOP, bevy_egui::egui::Vec2::new(10.0, 10.0))
            .show(egui_context.ctx_mut(), |ui| {

            //let world_pos_str = ;

            let delta = time.delta_seconds();

            ui.label( format!("{world_pos} {delta}") );
        });

        //eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
    }
}
*/


fn fun_name(screen_pos: Vec2, window_size: Vec2, camera_transform: &GlobalTransform, camera: &Camera) -> Vec2 {
    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    world_pos
}


fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    mut exit: EventWriter<AppExit>,
    query: Query<(
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

fn mouse_move_event( 
    mut mouse: EventReader<MouseMotion>,
) {
    for ev in mouse.iter() {
        //eprintln!("Mouse Delta: {}/{}", ev.delta.x, ev.delta.y);
    }
}

fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                //println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                //println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {

    //let log_settings = LogSettings::new();
    //let handle: Handle<Level> = asset_server.load("trees.json.level");


    for (_, mut transform) in query.iter_mut() {
        for ev in scroll_evr.iter() {
            let mut scroll = 0.0;
            match ev.unit {
                MouseScrollUnit::Line => {
                    //println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    scroll = ev.y;

                }
                MouseScrollUnit::Pixel => {
                    //println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    scroll = ev.y * 10.0;
                }
            }

            let scale = scroll * 1.0;
            let old_scale = transform.scale.x;
            let new_scale = (old_scale + scale).clamp( 1.0, 20.0 );
            transform.scale = Vec3::splat( new_scale );
        }
    }
}

