use bevy::prelude::*;

use super::{create_drawable_material, Drawable, DrawableMaterial, DrawableObject};

pub fn add_drawable_system(
    mut commands: Commands,
    drawable_mesh_query: Query<(&Drawable, &Mesh3d, &Transform, Entity), Added<Drawable>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drawable_materials: ResMut<Assets<DrawableMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for drawable in drawable_mesh_query.iter() {
        let mesh = meshes.get(&drawable.1 .0);

        if let Some(mesh) = mesh {
            // assume mesh is a plane
            let new_mesh = mesh.clone();
            let material = create_drawable_material(drawable.0.resolution(), &asset_server);
            let new_mesh_handle = meshes.add(new_mesh);

            let material_handle = drawable_materials.add(material);

            let drawable_plane = commands
                .spawn((
                    Mesh3d(new_mesh_handle),
                    MeshMaterial3d(material_handle),
                    drawable
                        .2
                        .clone()
                        .with_translation(Vec3::new(0.0, 0.01, 0.0)),
                    DrawableObject,
                ))
                .id();

            commands.entity(drawable.3).add_child(drawable_plane);
        }
    }
}

/* pub fn test_drawable_system(
    world: &World,
    drawable_mesh_query: Query<Entity, Added<Drawable>>,
) {
    for entity in drawable_mesh_query.iter() {
        println!("{:#?}", world.inspect_entity(entity).collect::<Vec<_>>());
    }

} */
