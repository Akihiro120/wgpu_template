use winit_input_helper::WinitInputHelper;

use crate::{
    gpu_state::{GPUState, begin_render_pass, create_basic_color_attachment, create_texture_view},
    mesh::Mesh,
    shader::{Shader, Uniform},
    template::Template,
};

mod gpu_state;
mod mesh;
mod shader;
mod template;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pos: glam::Vec3,
}

impl Vertex {
    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![
                0 => Float32x3
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SampleUniform {
    time: f32,
}

pub struct SampleTemplate {
    sample_shader: Shader,
    sample_uniform: Uniform,
    sample_mesh: Mesh,
}

impl Template for SampleTemplate {
    fn create(gpu_state: &mut GPUState) -> Self {
        let sample_uniform = Uniform::new(&gpu_state.device, &SampleUniform { time: 0.0 });

        let sample_shader = Shader::new(
            &gpu_state.device,
            gpu_state.rgb_format,
            "./src/sample.wgsl",
            &[Some(Vertex::layout())],
            &[Some(&sample_uniform)],
        );

        let sample_mesh = Mesh::new(
            &gpu_state.device,
            &[
                Vertex {
                    pos: glam::Vec3::new(-0.5, 0.5, 0.0),
                },
                Vertex {
                    pos: glam::Vec3::new(-0.5, -0.5, 0.0),
                },
                Vertex {
                    pos: glam::Vec3::new(0.5, -0.5, 0.0),
                },
                Vertex {
                    pos: glam::Vec3::new(0.5, 0.5, 0.0),
                },
            ],
            &[0, 1, 3, 3, 1, 2],
        );

        Self {
            sample_shader,
            sample_uniform,
            sample_mesh,
        }
    }

    fn step(&mut self, input: &mut WinitInputHelper) {
        if input.key_pressed(winit::keyboard::KeyCode::Enter) {
            println!("The enter key has been pressed.");
        }
    }

    fn draw(&mut self, gpu_state: &mut GPUState) {
        let mut encoder = gpu_state.create_command_encoder();

        let Some(surf) = gpu_state.get_surface_texture() else {
            return;
        };

        let view = create_texture_view(&surf.texture);

        let mut render_pass = begin_render_pass(
            &mut encoder,
            &[Some(create_basic_color_attachment(
                &view,
                [0.4, 0.4, 0.9, 1.0],
            ))],
            None,
        );

        self.sample_shader.bind_to_render_pass(&mut render_pass);
        self.sample_uniform.bind_to_render_pass(0, &mut render_pass);
        self.sample_mesh.bind_to_render_pass(&mut render_pass);

        drop(render_pass);

        gpu_state.queue.submit(std::iter::once(encoder.finish()));
        gpu_state.queue.present(surf);
    }
}

fn main() {
    SampleTemplate::run();
}
