
pub(crate) mod cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "src/shaders/compute.glsl"
    }
}