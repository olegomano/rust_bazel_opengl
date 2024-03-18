use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use winit::event::{Event, WindowEvent,ElementState};
use winit::window::WindowBuilder;
use winit::platform::run_return::EventLoopExtRunReturn;
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
use gl_context::gl;



pub trait App{
    fn Tick(&mut self);
}

pub struct Context<'a>{
    app : &'a mut dyn App,
}


impl<'a> Context<'a> {
    pub fn new( app : &'a mut dyn App ) -> Self{
        return Context{
            app : app
        }
    }

    pub fn Run(&mut self) -> i32{ 
        let mut event_loop = EventLoopBuilder::new().build();
        let (gl_context,gl_config,gl_surface,window_context,window) = CreateGlContext(&event_loop);
        unsafe{
            gl_context.ClearColor(1.0, 1.0, 1.0, 1.0);
        }
        return event_loop.run_return(move |event, window_target, control_flow| { 
            control_flow.set_wait();
            match event {
                Event::Resumed => {
                    println!("Resumed");
                },
                Event::RedrawEventsCleared => {
                    unsafe{
                        gl_context.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    }
                    gl_surface.swap_buffers(&window_context).unwrap();
                },
                Event::WindowEvent{event,..} =>{
                    match event{
                        WindowEvent::KeyboardInput{event, ..} => {
                            println!("{:?}",event);
                        },
                        WindowEvent::Resized(size) => {
                            println!("Window has been resized");
                            gl_surface.resize(
                                &window_context,
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                            unsafe{
                                gl_context.Viewport(0, 0, size.width as i32, size.height as i32);
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
}


fn CreateGlContext(event_loop : &winit::event_loop::EventLoop<()>) -> 
        (gl::Gl, 
        glutin::config::Config,
        glutin::surface::Surface<glutin::surface::WindowSurface>, 
        PossiblyCurrentContext,
        winit::window::Window,
        ){
    unsafe{
    let window_builder = Some(WindowBuilder::new()
        .with_transparent(false)
        .with_decorations(true)
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
}


