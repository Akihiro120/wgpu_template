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
}
