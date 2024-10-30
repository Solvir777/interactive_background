pub(crate) mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vert.glsl",
    }
}

pub(crate) mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/frag.glsl",
    }
}