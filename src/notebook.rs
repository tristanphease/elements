use bevy::{asset::RenderAssetUsages, color, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};

use crate::{drawable::{Drawable, DrawableObject}, drawable_material::DrawableMaterial};

pub fn add_notebook(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut drawable_materials: ResMut<Assets<DrawableMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cuboid = Cuboid::from_size(Vec3::new(10.0, 5.0, 20.0));
    let mesh: Handle<Mesh> = meshes.add(cuboid);

    let material = standard_materials.add(Color::from(color::palettes::basic::GRAY));

    let mut notebook_entity = commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Drawable,
    ));

    let wrapper_mesh = make_wrapper_mesh(cuboid);


    let image = create_drawable_texture();
    let image_handle = asset_server.add(image);
    let material = DrawableMaterial::new(image_handle);

    //wrapper for drawable
    notebook_entity.with_child((
        Mesh3d(meshes.add(wrapper_mesh)),
        MeshMaterial3d(drawable_materials.add(material)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        DrawableObject,
    ));

}

fn make_wrapper_mesh(mesh: Cuboid) -> Cuboid {
    let existing_size = mesh.size();
    let new_size = existing_size + 0.01;
    let new_mesh = Cuboid::from_size(new_size);

    return new_mesh;
}

fn create_drawable_texture() -> Image {

    const TEXTURE_SIZE: usize = 128;

    // texture size^2
    let texture_data = [0u8; TEXTURE_SIZE * TEXTURE_SIZE * 4];

    Image::new_fill(
        Extent3d { 
            width: TEXTURE_SIZE as u32, 
            height: TEXTURE_SIZE as u32, 
            depth_or_array_layers: 1, 
        },
        TextureDimension::D2, 
        &texture_data, 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
}
