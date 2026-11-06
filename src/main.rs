use winit_input_helper::WinitInputHelper;

use crate::{gpu_state::GPUState, template::Template};

mod gpu_state;
mod template;

pub struct SampleTemplate {}

impl Template for SampleTemplate {
    fn create(gpu_state: &mut GPUState) -> Self {
        println!("Template created.");
        Self {}
    }

    fn step(&mut self, input: &mut WinitInputHelper) {
        if input.key_pressed(winit::keyboard::KeyCode::Enter) {
            println!("The enter key has been pressed.");
        }
    }

    fn draw(&mut self, gpu_state: &mut GPUState) {
        let mut encoder = gpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let Some(surf) = gpu_state.get_surface_texture() else {
            return;
        };

        let view = surf
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        drop(render_pass);

        gpu_state.queue.submit(std::iter::once(encoder.finish()));
        gpu_state.queue.present(surf);
    }
}

fn main() {
    SampleTemplate::run();
}
