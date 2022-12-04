use bevy::{prelude::{*}, render::{Extract, primitives::{Frustum, Plane}}};
use bevy_sprite::{TextureAtlasSprite, TextureAtlas, ExtractedSprite, ExtractedSprites};

use std::cell::Cell;
use thread_local::ThreadLocal;






#[inline]
pub fn intersects_sphere(planes: [Plane; 6], sphere: &Vec3, radius: f32, intersect_far: bool) -> bool {
    let sphere_center = sphere.extend(1.0);
    let max = if intersect_far { 6 } else { 5 };
    for plane in &planes[..max] {
        if plane.normal_d().dot(sphere_center) + radius <= 0.0 {
            return false;
        }
    }
    true
}




/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`)
#[derive(Bundle, Clone, Default)]
pub struct StaticSpriteSheetBundle {
    /// The specific sprite from the texture atlas to be drawn
    pub sprite: TextureAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    pub global_transform: GlobalTransform,
}



pub fn extract_sprites(
    mut thread_sprites: Local<ThreadLocal<Cell<Vec<ExtractedSprite>>>>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    cam: Extract<Query<(&Camera, &GlobalTransform, &Frustum)>>,
    sprite_query: Extract<
        Query<(
            Entity,
            &Sprite,
            &GlobalTransform,
            &Handle<Image>,
        )>,
    >,
    atlas_query: Extract<
        Query<(
            Entity,
            &TextureAtlasSprite,
            &GlobalTransform,
            &Handle<TextureAtlas>,
        )>,
    >,
) {

    //extracted_sprites.sprites.clear();

    for( _cam, _cam_transform, frustum ) in cam.iter() {
        

        for (entity, sprite, transform, handle) in sprite_query.iter() {
            // PERF: we don't check in this function that the `Image` asset is ready, since it should be in most cases and hashing the handle is expensive
            extracted_sprites.sprites.push(ExtractedSprite {
                entity,
                color: sprite.color,
                transform: *transform,
                rect: sprite.rect,
                // Pass the custom size
                custom_size: sprite.custom_size,
                flip_x: sprite.flip_x,
                flip_y: sprite.flip_y,
                image_handle_id: handle.id(),
                anchor: sprite.anchor.as_vec(),
            });
        }

        atlas_query.par_for_each(65536, |(
            entity, 
            atlas_sprite, 
            transform, 
            texture_atlas_handle
        )| {

            let pos = transform.translation();

            let visible = intersects_sphere( frustum.planes, &pos, 16.0, false);

            if visible {
                if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                    let rect = Some(texture_atlas.textures[atlas_sprite.index]);

                    let sprite = ExtractedSprite {
                            entity,
                            color: atlas_sprite.color,
                            transform: *transform,
                            // Select the area in the texture atlas
                            rect,
                            // Pass the custom size
                            custom_size: atlas_sprite.custom_size,
                            flip_x: atlas_sprite.flip_x,
                            flip_y: atlas_sprite.flip_y,
                            image_handle_id: texture_atlas.texture.id(),
                            anchor: atlas_sprite.anchor.as_vec(),
                    };
                    
                    let cell = thread_sprites.get_or_default();
                    let mut queue = cell.take();
                    queue.push(sprite);
                    cell.set(queue); 
                }
            }


        });

        for cell in thread_sprites.iter_mut() {
            //extracted_sprites.sprites.push(cell.get_mut());
            extracted_sprites.sprites.append(cell.get_mut());
        }

        /*
        for (
            entity, 
            visibility, 
            atlas_sprite, 
            transform, 
            texture_atlas_handle
        ) in atlas_query.iter() {
            if !visibility.is_visible() {
                continue;
            }

            let pos = transform.translation();

            let visible = intersects_sphere( frustum.planes, &pos, 16.0, false);

            if visible {
                if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                let rect = Some(texture_atlas.textures[atlas_sprite.index]);
                extracted_sprites.sprites.push(ExtractedSprite {
                    entity,
                    color: atlas_sprite.color,
                    transform: *transform,
                    // Select the area in the texture atlas
                    rect,
                    // Pass the custom size
                    custom_size: atlas_sprite.custom_size,
                    flip_x: atlas_sprite.flip_x,
                    flip_y: atlas_sprite.flip_y,
                    image_handle_id: texture_atlas.texture.id(),
                    anchor: atlas_sprite.anchor.as_vec(),
                });
            }
            } else {
                //print!("Invisible")
            }

        }
        */


    }

}
