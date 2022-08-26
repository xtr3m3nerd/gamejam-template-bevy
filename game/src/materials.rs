use bevy::{
    prelude::*,
    render::render_resource::{
        AsBindGroup,
        ShaderRef,
    },
    sprite::Material2d,
    reflect::TypeUuid,
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "b17e3ec0-b8e2-4b66-a62e-1ed9f4374350"]
pub struct PostProcessingMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,

    /// the larger the value, the more rounded the screen (must be between 0 and 1)
    #[uniform(2)]
    pub screen_shape_factor: f32,

    /// controls amount of screen rows
    #[uniform(3)]
    pub rows: f32,

    /// screen brightness (I recommend setting it to 3 or 4 if you do not want create a horror game)
    #[uniform(4)]
    pub brightness: f32,

    /// screen edge shadow effect size
    #[uniform(5)]
    pub edges_transition_size: f32,

    /// Each pixel contains 3 sub-pixels (red, green and blue).
    /// This option allows you to display the color of all channels in any subpixels.
    /// I really recommend play with it (only use values between 0 and 1)
    #[uniform(6)]
    pub channels_mask_min: f32,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/post_processing.wgsl".into()
    }
}
