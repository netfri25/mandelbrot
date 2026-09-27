use macroquad::prelude::*;

use crate::renderer::texture_renderer::TextureRenderer;
use crate::types::{Dimensions, View};

const VERTEX_SHADER: &str = include_str!("./vertex_shader.glsl");
const FRAGMENT_SHADER: &str = include_str!("./fragment_shader.glsl");

pub struct GpuTextureRenderer {
    material: Material,
    iterations: u32,
    render_target: Option<RenderTarget>,
}

impl GpuTextureRenderer {
    pub fn new(iterations: u32) -> Self {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX_SHADER,
                fragment: FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("start", UniformType::Float2),
                    UniformDesc::new("size", UniformType::Float2),
                    UniformDesc::new("iterations", UniformType::Int1),
                ],
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            material,
            iterations,
            render_target: None,
        }
    }
}

impl TextureRenderer for GpuTextureRenderer {
    fn render_texture(&mut self, view: &View, dims: Dimensions) -> Texture2D {
        let target: &RenderTarget = match self.render_target.as_mut() {
            Some(target) => {
                if target.texture.width() as u64 != dims.w
                    || target.texture.height() as u64 != dims.h
                {
                    *target = render_target(dims.w as u32, dims.h as u32);
                }

                target
            }
            None => {
                let target = render_target(dims.w as u32, dims.h as u32);
                self.render_target.insert(target)
            }
        };

        push_camera_state();
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            zoom: vec2(2.0 / dims.w as f32, 2.0 / dims.h as f32),
            target: vec2(dims.w as f32 / 2.0, dims.h as f32 / 2.0),
            ..Default::default()
        });

        {
            self.material.set_uniform(
                "start",
                (view.start.x.to_f64() as f32, view.start.y.to_f64() as f32),
            );

            self.material.set_uniform(
                "size",
                (view.size.w.to_f64() as f32, view.size.h.to_f64() as f32),
            );

            self.material.set_uniform("iterations", self.iterations);

            gl_use_material(&self.material);
            draw_rectangle(0., 0., dims.w as f32, dims.h as f32, WHITE);
            gl_use_default_material();
        }

        pop_camera_state();

        target.texture.clone()
    }
}
