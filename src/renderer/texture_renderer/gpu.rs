use macroquad::prelude::*;

use crate::renderer::texture_renderer::TextureRenderer;
use crate::types::{Dimensions, View};

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

const VERTEX_SHADER: &str = r#"
#version 100

attribute vec3 position;
attribute vec2 texcoord;

uniform mat4 Model;
uniform mat4 Projection;

varying vec2 uv;

void main() {
    uv = texcoord;
    gl_Position = Projection * Model * vec4(position, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 100
precision highp float;

varying vec2 uv;

uniform vec2 start;
uniform vec2 size;
uniform int iterations;

vec3 color(float t) {
    t = clamp(t, 0.0, 1.0);

    return vec3(
        9.0 * (1.0 - t) * t * t * t,
        15.0 * (1.0 - t) * (1.0 - t) * t * t,
        8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t
    );
}

void main() {
    vec2 c = start + uv * size;
    vec2 z = vec2(0.0);

    int i;

    for (i = 0; i < iterations; i++) {
        z = vec2(
            z.x * z.x - z.y * z.y,
            2.0 * z.x * z.y
        ) + c;

        if (dot(z, z) > 4.0) {
            break;
        }
    }

    if (i == iterations) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    float magnitude = length(z);
    float smooth_i = float(i) + 1.0 - log(log(magnitude)) / log(2.0);
    float t = sqrt(smooth_i) * 0.05;

    gl_FragColor = vec4(color(t), 1.0);
}
"#;
