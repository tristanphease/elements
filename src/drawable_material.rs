use bevy::{prelude::*, render::render_resource::{AsBindGroup, ShaderRef}};

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
        draw_texture: Handle<Image>
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