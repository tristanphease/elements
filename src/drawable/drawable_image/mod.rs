use bevy::prelude::*;

use crate::drawable::{DrawableMaterial, DrawableObject};

/// Message for saving the drawable image(s) to a file
#[derive(Debug, Message)]
pub(crate) struct SaveDrawableImage;

// saves the drawable image(s) to a file
pub(super) fn save_drawable_image(
    mut reader: MessageReader<SaveDrawableImage>,
    drawable_query: Query<&MeshMaterial3d<DrawableMaterial>, With<DrawableObject>>,
    drawable_materials: Res<Assets<DrawableMaterial>>,
    images: Res<Assets<Image>>,
) {
    for _ in reader.read() {
        // save image
        for (index, drawable_mesh_mat) in drawable_query.iter().enumerate() {
            if let Some(drawable_mat) = drawable_materials.get(&drawable_mesh_mat.0) {
                if let Some(image) = images.get(&drawable_mat.draw_texture) {
                    // convert image and save
                    if let Ok(image_converted) = image.clone().try_into_dynamic() {
                        let path = format!(
                            "./temp/drawable_image{}.webp",
                            if index > 0 {
                                format!("_{}", index + 1)
                            } else {
                                "".to_owned()
                            }
                        );
                        let save_result = image_converted.save(&path);

                        if let Err(error) = save_result {
                            println!("Failed to save image: {:?}", error);
                        } else {
                            println!("Saved image to {}", path);
                        }
                    } else {
                        println!("Couldn't convert bevy image");
                    }
                }
            }
        }
    }
}

/// Message for clearing the drawable image(s)
#[derive(Debug, Message)]
pub(crate) struct ClearDrawableImage;

/// clears drawable image(s)
pub(super) fn clear_drawable_image(
    mut reader: MessageReader<ClearDrawableImage>,
    drawable_query: Query<&MeshMaterial3d<DrawableMaterial>, With<DrawableObject>>,
    // this is only mutable for change detection
    mut drawable_materials: ResMut<Assets<DrawableMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    for _ in reader.read() {
        for drawable_mesh_mat in drawable_query.iter() {
            if let Some(drawable_mat) = drawable_materials.get_mut(&drawable_mesh_mat.0) {
                if let Some(ref mut image) = images.get_mut(&drawable_mat.draw_texture) {
                    println!("clearing image");
                    if let Some(ref mut image_data) = image.data {
                        for x in image_data.iter_mut() {
                            *x = 0;
                        }
                    }
                }
            }
        }
    }
}
