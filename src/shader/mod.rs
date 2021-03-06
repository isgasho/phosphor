//! Shaders. Macro-generated with `vulkano-shaders`.

/// Shader for rendering line sets.
pub mod lines {
    pub mod vertex {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "src/shader/lines.vert"
        }
    }
    pub mod fragment {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "src/shader/lines.frag"
        }
    }
}

/// Shader for rendering the skybox.
pub mod skybox {
    pub mod vertex {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "src/shader/skybox.vert"
        }
    }
    pub mod fragment {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "src/shader/skybox.frag"
        }
    }
}

/// Shader for rendering text.
pub mod text {
    pub mod vertex {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "src/shader/text.vert"
        }
    }
    pub mod fragment {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "src/shader/text.frag"
        }
    }
}

/// Deferred pipeline shading shaders
pub mod deferred_shading {
    pub mod vertex {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "src/shader/deferred_shading.vert"
        }
    }
    pub mod fragment {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "src/shader/deferred_shading.frag"
        }
    }
}

/// Deferred pipeline lighting shaders
pub mod deferred_lighting {
    pub mod vertex {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "src/shader/deferred_lighting.vert"
        }
    }
    pub mod fragment {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "src/shader/deferred_lighting.frag"
        }
    }
}

/// Tonemapping pass shaders
pub mod tonemapper {
    pub mod vertex {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "src/shader/tonemapper.vert"
        }
    }
    pub mod fragment {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "src/shader/tonemapper.frag"
        }
    }
}

/// Occlusion pass shaders
pub mod occlusion {
    pub mod vertex {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "src/shader/occlusion.vert"
        }
    }
    pub mod fragment {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "src/shader/occlusion.frag"
        }
    }
}


pub mod histogram {
    vulkano_shaders::shader!{
        ty: "compute",
        path: "src/shader/histogram.comp"
    }
}