use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{AsBindGroup, Extent3d, TextureDimension, TextureFormat},
    shader::ShaderRef,
};

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
    pub fn new(draw_texture: Handle<Image>) -> Self {
        Self { draw_texture }
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
    resolution: usize,
    asset_server: &Res<AssetServer>,
) -> DrawableMaterial {
    let image = create_drawable_image(resolution);

    let image_handle = asset_server.add(image);

    DrawableMaterial::new(image_handle)
}

fn create_drawable_image(resolution: usize) -> Image {
    let texture_data = vec![0u8; resolution * 4];

    Image::new_fill(
        Extent3d {
            width: resolution as u32,
            height: resolution as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
}
