use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat}};

const DRAWABLE_SHADER_PATH: &str = "shaders/drawable_shader.wgsl";

/// data for drawable shader
/// 
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DrawableMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub draw_texture: Handle<Image>,
}

impl DrawableMaterial {
    pub fn new(
        draw_texture: Handle<Image>,
    ) -> Self {
        Self {
            draw_texture
        }
    }
}

impl Material for DrawableMaterial {

    fn fragment_shader() -> ShaderRef {
        DRAWABLE_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

pub fn create_drawable_material(
    mesh: &Mesh,
    asset_server: &Res<AssetServer>,
) -> DrawableMaterial {
    // for now, just make all the triangle materials the same size.
    let triangle_num = mesh.triangles().expect("mesh without triangles?").count();

    // not a fan of these conversions but whatever
    let triangle_num_f32 = triangle_num as f32;

    let square_size_f32 = triangle_num_f32.sqrt();

    let square_size = square_size_f32 as usize;

    let image = create_drawable_image(square_size);

    let image_handle = asset_server.add(image);

    DrawableMaterial::new(image_handle)
}

fn create_drawable_image(square_size: usize) -> Image {

    const TEXTURE_SIZE: usize = 128;

    // texture size^2
    let texture_data = vec![0u8; square_size * TEXTURE_SIZE * TEXTURE_SIZE * 4];

    Image::new_fill(
        Extent3d { 
            width: (TEXTURE_SIZE * square_size) as u32, 
            height: (TEXTURE_SIZE * square_size) as u32, 
            depth_or_array_layers: 1, 
        },
        TextureDimension::D2, 
        &texture_data, 
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
}

