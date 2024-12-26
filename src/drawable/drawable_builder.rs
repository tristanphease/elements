use bevy::prelude::*;

use super::{create_drawable_material, DrawableMaterial, DrawableObject};

pub trait BuildDrawable {
    /// Spawns a drawable child of this entity
    /// 
    fn add_drawable(
        &mut self,
        existing_mesh_handle: &Handle<Mesh>,
        meshes: &mut ResMut<Assets<Mesh>>,
        drawable_materials: &mut ResMut<Assets<DrawableMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> &mut Self;
}

impl BuildDrawable for EntityCommands<'_> {
    fn add_drawable(
        &mut self,
        existing_mesh_handle: &Handle<Mesh>,
        meshes: &mut ResMut<Assets<Mesh>>,
        drawable_materials: &mut ResMut<Assets<DrawableMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> &mut Self {
        let existing_mesh = meshes.get(existing_mesh_handle).unwrap();
        let (drawable_mesh, drawable_mat) = make_wrapper_objects(existing_mesh, asset_server);

        let mesh3d = Mesh3d(meshes.add(drawable_mesh));
        let mesh_material = MeshMaterial3d(drawable_materials.add(drawable_mat));

        self.with_child((
            mesh3d,
            mesh_material,
            Transform::from_xyz(0.0, 0.0, 0.0),
            DrawableObject,
        ));

        self
    }
} 

fn make_wrapper_mesh(mesh: &Mesh) -> Mesh {
    /* let new_mesh = mesh.clone()
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, values); */
    let new_mesh = mesh.clone();

    new_mesh
}

fn make_wrapper_objects(
    mesh: &Mesh,
    asset_server: &Res<AssetServer>,
) -> (
    Mesh,
    DrawableMaterial
) {
    let new_mesh = make_wrapper_mesh(mesh);
    let new_mat = create_drawable_material(&new_mesh, asset_server);

    (
        new_mesh,
        new_mat
    )
} 