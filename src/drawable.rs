use std::ops::DerefMut;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::drawable_material::DrawableMaterial;

#[derive(Component)]
#[require(Transform)]
pub struct Drawable;

/// Component for the actual drawable object itself
#[derive(Component)]
pub struct DrawableObject;

pub fn drawing_system(
    drawable_query: Query<&Children, With<Drawable>>,
    mut drawable_child_query: Query<(&GlobalTransform, &mut MeshMaterial3d<DrawableMaterial>), With<DrawableObject>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut ray_cast: MeshRayCast,
    mut drawable_mat_assets: ResMut<Assets<DrawableMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut updated = false;
    if buttons.pressed(MouseButton::Left) {
        if let Some(mouse_position) = window.cursor_position() {
            //ray starts at camera and screen pos
            let ray_info = ray_from_screen(
                window.size(),
                mouse_position,
                *camera
            );
            let dir = Dir3::new(ray_info.1).unwrap();
            let ray = Ray3d::new(ray_info.0,  dir);

            let drawable_entity_filter = |entity| drawable_query.contains(entity);

            let ray_settings = RayCastSettings::default()
                .always_early_exit()
                .with_filter(&drawable_entity_filter)
                .with_visibility(RayCastVisibility::Visible);

            let hits = ray_cast.cast_ray(ray, &ray_settings);
            for (entity, hit_info) in hits {
                let hit_entity = drawable_query.get(*entity)
                    .expect("huh?");

                for child in hit_entity.iter() {
                    if let Ok((transform, mesh_material)) = drawable_child_query.get_mut(*child) {
                        let material_option = drawable_mat_assets.get_mut(&mesh_material.0);
                        
                        if let Some(material) = material_option {
                            let image = images.get_mut(&material.draw_texture);

                            if let Some(image) = image {
                                let local_coords = hit_info.point - transform.translation();

                                //let mut coord = (local_coords.length() * 1000.0) as usize;

                                let mut coord = get_coord_from_camera(local_coords.x, local_coords.z);

                                // round down to 4 since colour is in 4 coords for rgba
                                coord = coord - (coord % 4);
                                println!("coords: {local_coords:?}, coord: {coord:?}");

                                for i in [0, 3] {
                                    if let Some(val) = image.data.get_mut(coord + i) {
                                        *val = 255;
                                    }
                                }

                                updated = true;
                            }
                        }
                    }
                }
            }
        }
    }
    // need to update for the change detection to work
    // https://github.com/bevyengine/bevy/issues/15595
    if updated {
        for (_, mut image) in images.iter_mut() {
            image.deref_mut();
        }
    }
}

// the coords here are world coords, converting to material coords
fn get_coord_from_camera(coord_x: f32, coord_z: f32) -> usize {
    const MESH_WIDTH: f32 = 10.0;
    const MESH_HEIGHT: f32 = 20.0;

    const CAMERA_DIST: f32 = 5.0 / 2.0 + 27.0;

    const MAT_SIZE: f32 = 128.0;

    let x = coord_x + MESH_WIDTH / 2.0;
    let z = -coord_z + MESH_HEIGHT / 2.0;

    // between 0-1
    let x = x / MESH_WIDTH;
    let z = z / MESH_HEIGHT;

    //
    let x_u = (x * MAT_SIZE) as usize;
    let z_u = (z * MAT_SIZE) as usize;

    let coord = 4 * (z_u * MAT_SIZE as usize + x_u);

    println!("x: {x}, z: {z}");
    
    coord
}

fn ray_from_screen(
    window_size: Vec2,
    cursor_pos: Vec2,
    camera: (&Camera, &GlobalTransform),
) -> (Vec3, Vec3) {
    let coords = (cursor_pos / window_size) * 2.0 - Vec2::ONE;

    let coords_world = camera.1.compute_matrix() * camera.0.clip_from_view().inverse();

    // origin of ray
    let cursor_pos_world = coords_world.project_point3(coords.extend(-1.0));

    // direction of ray
    let ray_direction = (camera.1.translation() - cursor_pos_world).normalize();

    (cursor_pos_world, ray_direction)
}
