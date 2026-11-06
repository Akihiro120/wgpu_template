use std::sync::Arc;

use winit_input_helper::WinitInputHelper;

use crate::gpu_state::GPUState;

#[derive(Default)]
struct TemplateFactory<T: Template> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> TemplateFactory<T>
where
    T: Template + 'static,
{
    pub fn create_template(&self, gpu: &mut GPUState) -> Box<dyn Template> {
        Box::new(T::create(gpu))
    }
}

#[derive(Default)]
pub struct TemplateApplication<T: Template> {
    window: Option<Arc<winit::window::Window>>,
    template_state: Option<Box<dyn Template>>,
    template_factory: TemplateFactory<T>,
    input: WinitInputHelper,
    gpu_state: Option<GPUState>,
}

impl<T: Template> TemplateApplication<T> {
    pub fn new() -> Self {
        Self {
            window: None,
            template_state: None,
            template_factory: TemplateFactory::<T> {
                _marker: std::marker::PhantomData,
            },
            input: WinitInputHelper::default(),
            gpu_state: None,
        }
    }
}

impl<T> winit::application::ApplicationHandler for TemplateApplication<T>
where
    T: Template + 'static,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // create the window
        let window = Arc::new(
            event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_title("WGPU Template")
                        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720)),
                )
                .expect("ERROR: Failed to create window."),
        );

        self.window = Some(window.clone());

        // create gpu state
        self.gpu_state = Some(GPUState::new(window.clone()));

        // create template state
        self.template_state = Some(
            self.template_factory
                .create_template(self.gpu_state.as_mut().unwrap()),
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(size) = self.input.window_resized() {
            self.gpu_state.as_mut().unwrap().resize(size);
        }

        if self.input.process_window_event(&event) {
            // render stuff here
            self.template_state
                .as_mut()
                .unwrap()
                .draw(self.gpu_state.as_mut().unwrap());
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.input.process_device_event(&event);
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        self.input.step();
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.input.end_step();

        // if the event loop is exiting, then theres no point to continue
        if event_loop.exiting() {
            return;
        }

        if self.input.key_pressed(winit::keyboard::KeyCode::Escape)
            || self.input.close_requested()
            || self.input.destroyed()
        {
            println!("Exit detected!. Closing window.");
            event_loop.exit();
            return;
        }

        // step
        self.template_state.as_mut().unwrap().step(&mut self.input);

        // rebuild and render
        self.window.as_ref().unwrap().request_redraw();
    }
}

pub trait Template: 'static {
    fn run()
    where
        Self: Sized,
    {
        let event_loop = winit::event_loop::EventLoop::builder()
            .build()
            .expect("ERROR: Failed to create event loop.");

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop
            .run_app(&mut TemplateApplication::<Self>::new())
            .unwrap();
    }

    fn create(gpu_state: &mut GPUState) -> Self
    where
        Self: Sized;

    fn step(&mut self, input: &mut WinitInputHelper);
    fn draw(&mut self, gpu_state: &mut GPUState);
    // fn gui();
}
