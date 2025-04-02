use std::ops::DerefMut;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::drawable::drawable_material::DrawableMaterial;

use super::paint::{
    paint::{PaintImage, PaintSettings},
    paint_input::PaintInput,
};

/// Component for the object
#[derive(Component, Reflect, Debug)]
#[require(Transform)]
pub struct Drawable {
    resolution: usize,
}

impl Drawable {
    pub fn resolution(&self) -> usize {
        self.resolution
    }
}

impl Default for Drawable {
    fn default() -> Self {
        Self { resolution: 1024 }
    }
}

/// Component for the actual drawable object itself
#[derive(Component)]
pub struct DrawableObject;

/// The main drawing system that handles mouse input for drawing on drawable objects
pub fn drawing_system(
    drawable_query: Query<(&Children, &Drawable)>,
    mut drawable_child_query: Query<
        (&GlobalTransform, &mut MeshMaterial3d<DrawableMaterial>),
        With<DrawableObject>,
    >,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut ray_cast: MeshRayCast,
    mut drawable_mat_assets: ResMut<Assets<DrawableMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut paint_input: ResMut<PaintInput>,
) {
    let mut updated = false;
    if buttons.pressed(MouseButton::Left) {
        if let Some(mouse_position) = window.cursor_position() {
            //ray starts at camera and screen pos
            let ray_info = ray_from_screen(window.size(), mouse_position, *camera);
            let dir = Dir3::new(ray_info.1).unwrap();
            let ray = Ray3d::new(ray_info.0, dir);

            let drawable_entity_filter = |entity| drawable_query.contains(entity);

            let ray_settings = RayCastSettings::default()
                .always_early_exit()
                .with_filter(&drawable_entity_filter)
                .with_visibility(RayCastVisibility::Visible);

            let hits = ray_cast.cast_ray(ray, &ray_settings);
            for (entity, hit_info) in hits {
                let (hit_entity, drawable) = drawable_query.get(*entity).expect("huh?");

                for child in hit_entity.iter() {
                    if let Ok((transform, mesh_material)) = drawable_child_query.get_mut(*child) {
                        let material_option = drawable_mat_assets.get_mut(&mesh_material.0);

                        if let Some(material) = material_option {
                            let image = images.get_mut(&material.draw_texture);

                            if let Some(image) = image {
                                let (u, v) = get_uv_from_position(
                                    hit_info.point,
                                    hit_info.normal,
                                    transform,
                                );

                                // let mut coord = get_coord_from_uv(-u, v, drawable.resolution());

                                let (x, y) = get_coords_from_uv(-u, v, drawable.resolution());

                                // println!("x: {x}, y: {y}, u: {u}, v: {v}");

                                //change to resource
                                let paint_settings = PaintSettings::default();
                                let plane_scale = transform.scale();
                                let scale = Vec2::new(plane_scale.x, plane_scale.z);

                                if !paint_input.mouse_down {
                                    image.draw_spot(x, y, &paint_settings, scale);
                                } else if let Some(last_pos) = paint_input.last_input_location {
                                    image.draw_thick_line(
                                        last_pos.0,
                                        last_pos.1,
                                        x as usize,
                                        y as usize,
                                        &paint_settings,
                                        scale,
                                    );
                                }

                                paint_input.last_input_location = Some((x, y));
                                paint_input.mouse_down = true;

                                updated = true;
                            }
                        }
                    }
                }
            }
        }
    } else {
        paint_input.mouse_down = false;
    }

    // need to update for the change detection to work
    // https://github.com/bevyengine/bevy/issues/15595
    if updated {
        for (_, mut image) in images.iter_mut() {
            image.deref_mut();
        }
    }
}

// https://gamedev.stackexchange.com/questions/172352/finding-texture-coordinates-for-plane
// returns between -1 and 1
fn get_uv_from_position(
    position: Vec3,
    normal: Vec3,
    plane_transform: &GlobalTransform,
) -> (f32, f32) {
    let mut e1 = Vec3::cross(normal, Vec3::new(1.0, 0.0, 0.0)).normalize_or_zero();

    if e1 == Vec3::ZERO {
        e1 = Vec3::cross(normal, Vec3::new(0.0, 0.0, 1.0)).normalize();
    }

    let e2 = Vec3::cross(normal, e1).normalize();

    let pos = position - plane_transform.translation();
    let plane_scale = plane_transform.scale();

    let v = Vec3::dot(e1, pos) / plane_scale.x;
    let u = Vec3::dot(e2, pos) / plane_scale.z;

    // println!("pos: {pos}, u: {u}, v: {v}");

    (u, v)

    // println!("position: {position:?}, plane position: {:?}", plane_transform.translation());
    // println!("plane scale: {:?}", plane_transform.scale());
}

fn get_coords_from_uv(u: f32, v: f32, resolution: usize) -> (usize, usize) {
    let x = (u + 1.0) / 2.0;
    let y = (v + 1.0) / 2.0;

    let x_index = x * resolution as f32;
    let y_index = y * resolution as f32;

    (x_index as usize, y_index as usize)
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
