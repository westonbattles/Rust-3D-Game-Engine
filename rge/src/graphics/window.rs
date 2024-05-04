use glfw::{Action, Context, GlfwReceiver, Key,  WindowEvent, WindowHint, fail_on_errors};

use super::gl_wrapper::SHADER_PROGRAM;



pub struct Window {
    pub glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    pub input_array: [i8; 3],
}

impl Window {
    // Creates a new window
    pub fn new(width: u32, height: u32, title: &str) -> Window{

        // Initialize the window with given settings
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        glfw.window_hint(WindowHint::ContextVersion(3,3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

        // Attempt to create the window, and panic if there were any errors
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW Window!");


        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        // Returns the window
        Window {
            glfw: glfw,
            window_handle: window,
            events: events,
            input_array: [0, 0, 0],
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }


    pub fn get_aspect_ratio(&mut self) -> f32{
        let (width, height) = self.window_handle.get_framebuffer_size();
        width as f32 / height as f32
    }

    pub fn set_framebuffer_size_callback(&mut self) {
        // Since we need to access our shader_program instance behind a statically defined
        // function, we use the lazy_static macro combined with mutex
        

        let framebuffer_size_callback = move |window: &mut glfw::Window, width: i32, height: i32| {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
            let mut shader_program_gaurd = SHADER_PROGRAM.lock().unwrap();
            if let Some(shader_program) = shader_program_gaurd.as_mut() {
                shader_program.set_float_uniform("aspectRatio", width as f32 / height as f32);
                super::drawer::draw();
                window.swap_buffers();
            } else {
                panic!("Shader program not defined?");
            }
            //println!("Framebuffer size changed: {}x{}", width, height);
        };
        self.window_handle.set_framebuffer_size_callback(framebuffer_size_callback);
    }



    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    pub fn get_framebuffer_size(&mut self) -> (i32, i32) {
        self.window_handle.get_framebuffer_size()
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {

                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {gl::Viewport(0, 0, width, height)}
                }

                // If escape key was pressed, we should close
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true);
                }

                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    self.input_array[1] += 1;
                }
                glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
                    self.input_array[1] -= 1;
                }

                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    self.input_array[0] -= 1;
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
                    self.input_array[0] += 1;
                }

                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    self.input_array[1] -= 1;
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
                    self.input_array[1] += 1;
                }

                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    self.input_array[0] += 1;
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
                    self.input_array[0] -= 1;
                }
                _ => {}
            }
        }
    }

}

/*fn framebuffer_size_callback(window: &mut glfw::Window, width: i32, height: i32) {
    // Adjust the viewport to the new size of the window
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
    println!("Framebuffer size changed: {}x{}", width, height);
}*/