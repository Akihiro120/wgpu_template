use std::sync::Arc;

pub struct GPUState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub surface_configuration: wgpu::SurfaceConfiguration,
    pub rgb_format: wgpu::TextureFormat,
    pub srgb_format: wgpu::TextureFormat,
}

async fn request_adapter(instance: &wgpu::Instance) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("ERROR: Failed to request for physical device.")
}

async fn request_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(&wgpu::DeviceDescriptor {
            ..Default::default()
        })
        .await
        .expect("ERROR: Failed to request for virtual device.")
}

impl GPUState {
    pub fn new(window: Arc<winit::window::Window>) -> Self {
        let instance = wgpu::Instance::default();

        let adapter = pollster::block_on(request_adapter(&instance));
        let (device, queue) = pollster::block_on(request_device(&adapter));

        let surface = instance
            .create_surface(window.clone())
            .expect("ERROR: Failed to create surface.");

        let capabilities = surface.get_capabilities(&adapter);

        // find the compatible formats now
        let rgb_format = capabilities
            .formats
            .iter()
            .find(|f| !f.is_srgb())
            .expect("ERROR: No compatible rgb format found")
            .to_owned();

        let srgb_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .expect("ERROR: No compatible srgb format found")
            .to_owned();

        let size = window.inner_size();

        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: rgb_format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            alpha_mode: capabilities.alpha_modes[0],
            present_mode: capabilities.present_modes[0],
            width: size.width,
            height: size.height,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_configuration);

        println!("GPU State successfully created.");
        Self {
            device,
            queue,
            surface,
            surface_configuration,
            rgb_format,
            srgb_format,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let current_size = winit::dpi::PhysicalSize::new(
            self.surface_configuration.width,
            self.surface_configuration.height,
        );

        if current_size != new_size {
            self.surface_configuration.width = new_size.width;
            self.surface_configuration.height = new_size.height;
            self.surface
                .configure(&self.device, &self.surface_configuration);
            println!("Window resized.");
        }
    }

    pub fn reconfigure(&self) {
        self.surface
            .configure(&self.device, &self.surface_configuration);
    }

    pub fn get_surface_texture(&self) -> Option<wgpu::SurfaceTexture> {
        let surf = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surf) => surf,
            wgpu::CurrentSurfaceTexture::Suboptimal(surf) => {
                self.reconfigure();
                surf
            }
            _ => return None,
        };

        Some(surf)
    }

    pub fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }
}

// this function returns a render pass with a lifetime that lasts for as long as the encoder.
// this ensures that the render pass will not outlive the encoder that makes it
pub fn begin_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    color_attachments: &[Option<wgpu::RenderPassColorAttachment>],
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment>,
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments,
        depth_stencil_attachment,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    })
}

// create a basic color attachment with a view and a color, this color attachment will only last as
// long as the texture view.
pub fn create_basic_color_attachment<'tex>(
    view: &'tex wgpu::TextureView,
    color: [f64; 4],
) -> wgpu::RenderPassColorAttachment<'tex> {
    wgpu::RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
                r: color[0],
                g: color[1],
                b: color[2],
                a: color[3],
            }),
            store: wgpu::StoreOp::Store,
        },
    }
}

pub fn create_texture_view(texture: &wgpu::Texture) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}
