use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use winit::event::{Event, WindowEvent,ElementState};
use winit::window::WindowBuilder;
use glutin::context::PossiblyCurrentContext;

use raw_window_handle::HasRawWindowHandle;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::SwapInterval;

use winit::event_loop::EventLoopBuilder;
use glutin_winit::{self, DisplayBuilder, GlWindow};
extern crate gl_context;
extern crate gl_utils;
extern crate key_manager;
use gl_context::gl;

pub trait AppData{
    fn new() -> Box<Self>;
    fn Draw(&mut self,gl : &gl::Gl);
    fn Init(&mut self,gl : &gl::Gl);
    fn HandleKeyboard(&mut self, keys : &key_manager::KeyManager, gl : &gl::Gl);
}

pub struct AppInstance<T>{
    app_data : Box<T>,
    event_loop : std::cell::UnsafeCell<winit::event_loop::EventLoop<()>>,
    key_manager : key_manager::KeyManager,
    gl_context : gl::Gl,
    gl_config : glutin::config::Config,
    gl_surface : glutin::surface::Surface<glutin::surface::WindowSurface>,
    window_context : PossiblyCurrentContext,
    window : winit::window::Window,
    vao: gl::types::GLuint,
}

impl<T : AppData + 'static> AppInstance<T>{


    #[gl_utils::timed]
    pub fn new() -> Self{
        unsafe{
            let app_data = T::new();
            let event_loop = EventLoopBuilder::new().build();
            let (gl_context,gl_config,gl_surface,window_context,window) = Self::CreateGlContext(&event_loop);
            return Self{
                app_data : app_data,
                event_loop : std::cell::UnsafeCell::new(event_loop),
                key_manager : key_manager::KeyManager::new(),
                gl_context : gl_context,
                gl_config : gl_config,
                gl_surface : gl_surface,
                window_context : window_context,
                window : window,
                vao : 0,
            }
        }
    }
    
    pub fn Run(mut self : Box<Self>){
        Self::PrintGlInfo(&self.gl_context);
        self.app_data.Init(&self.gl_context);
        
        unsafe{
            let mut vao = std::mem::zeroed();
            self.gl_context.GenVertexArrays(1, &mut vao);
            self.gl_context.BindVertexArray(vao);
            self.gl_context.Disable(gl::BLEND);
            self.gl_context.Enable(gl::DEPTH_TEST);
            self.gl_context.ClearColor(1.0, 1.0, 1.0, 1.0);
        }
        self.EventLoop().run(move |event, window_target, control_flow| {
            control_flow.set_wait();
            match event {
                Event::Resumed => {
                    println!("Resumed");
                },
                Event::RedrawEventsCleared => {
                    unsafe{
                        self.gl_context.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }
                    self.app_data.HandleKeyboard(&mut self.key_manager,&self.gl_context);
                    self.app_data.Draw(&self.gl_context);
                    self.window.request_redraw();
                    self.gl_surface.swap_buffers(&self.window_context).unwrap();
                },
                Event::WindowEvent{event,..} =>{
                    match event{
                        WindowEvent::KeyboardInput{event, ..} => {
                            self.key_manager.HandleInput(event);
                        },
                        WindowEvent::Resized(size) => {
                            println!("Window has been resized");
                            self.gl_surface.resize(
                                &self.window_context,
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                            unsafe{
                                self.gl_context.Viewport(0, 0, size.width as i32, size.height as i32);
                            
                            }
                        },
                        WindowEvent::CloseRequested=>{
                            control_flow.set_exit();
                        },
                        ref default =>{
                            //println!("Unknown Event {:?}",default);
                        }
                    }
                },
                ref default =>{
                    //println!("Unknown Event {:?}",default);
                }
            }
        });
    }

    #[gl_utils::timed]
    fn EventLoop(&self) -> winit::event_loop::EventLoop<()> {
        unsafe{
            return std::ptr::read(std::cell::UnsafeCell::<winit::event_loop::EventLoop<()>>::raw_get(&self.event_loop));
        }
    }
    
    #[gl_utils::timed]
    fn PrintGlInfo(gl : &gl::Gl){
        unsafe{
            if let Some(renderer) = Self::GetGlString(&gl, gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = Self::GetGlString(&gl, gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }
            if let Some(shaders_version) = Self::GetGlString(&gl, gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }
        }
    }

    #[gl_utils::timed]
    unsafe fn CreateGlContext(event_loop : &winit::event_loop::EventLoop<()>) -> 
            (gl::Gl, 
             glutin::config::Config,
             glutin::surface::Surface<glutin::surface::WindowSurface>, 
             PossiblyCurrentContext,
             winit::window::Window,
             ){
        let window_builder = Some(WindowBuilder::new()
            .with_transparent(false)
            .with_decorations(false)
        );
        let template =  ConfigTemplateBuilder::new().with_transparency(false);
        let display_builder = DisplayBuilder::new().with_window_builder(window_builder);
        let (mut window_option, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            // Find the config with the maximum number of samples, so our triangle will
            // be smooth.
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                 if transparency_check {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

        let window = window_option.take().expect("");

        let raw_window_handle = Some(window.raw_window_handle());
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
        let gl_display = gl_config.display();
        let mut not_current_gl_context = gl_display.create_context(&gl_config, &context_attributes).unwrap();
        let attrs = window.build_surface_attributes(<_>::default());
        let gl_surface = gl_config.display().create_window_surface(&gl_config, &attrs).unwrap();
        let gl_context =  not_current_gl_context.make_current(&gl_surface).unwrap();
        
        let gl = gl::Gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()).cast()
        });
        return (gl,gl_config,gl_surface,gl_context,window);
    }



    unsafe fn GetGlString(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}



