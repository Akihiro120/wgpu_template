use wgpu::util::DeviceExt;

pub struct Uniform {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl Uniform {
    pub fn new<T: bytemuck::Pod + bytemuck::Zeroable>(device: &wgpu::Device, data: &T) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn update<T: bytemuck::Pod + bytemuck::Zeroable>(&self, queue: &wgpu::Queue, value: &T) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(value));
    }

    pub fn bind_to_render_pass(&self, index: u32, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }
}

pub struct Shader {
    render_pipeline: wgpu::RenderPipeline,
}

impl Shader {
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        shader_file: &str,
        v_buffers: &[Option<wgpu::VertexBufferLayout>],
        uniforms: &[Option<&Uniform>],
    ) -> Self {
        let layouts: Vec<Option<&wgpu::BindGroupLayout>> = uniforms
            .iter()
            .map(|u| u.and_then(|u| Some(&u.bind_group_layout)))
            .collect();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &layouts,
            immediate_size: 0,
        });

        let shader_source = std::fs::read_to_string(shader_file)
            .unwrap_or_else(|_| panic!("ERROR: Failed to read shader file: {}", shader_file));

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: v_buffers,
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::default(),
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        Self { render_pipeline }
    }

    pub fn bind_to_render_pass(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
    }

    // NOTE: this is a future feature
    // pub fn bind_to_compute_pass(&self, render_pass: &wgpu::RenderPass) {}
}
