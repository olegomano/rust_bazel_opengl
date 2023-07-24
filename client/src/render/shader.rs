use std::ffi::{CStr, CString};
extern crate gl_context;
use gl_context::gl;
extern crate gl_utils;

#[gl_utils::timed]
unsafe fn CreateProgram(gl_context : &gl::Gl, vertex_shader: &str, fragment_shader: &str) -> gl::types::GLuint{
   let program = gl_context.CreateProgram();
   let vert = CompileVertexShader(gl_context,vertex_shader);
   PrintError(gl_context);
   let frag = CompileFragmentShader(gl_context,fragment_shader);
   PrintError(gl_context);
   gl_context.AttachShader(program,vert);
   gl_context.AttachShader(program,frag);
   gl_context.LinkProgram(program);
   PrintError(gl_context);
   return program;
}

#[gl_utils::timed]
unsafe fn CompileFragmentShader(gl_context : &gl::Gl, source: &str) -> gl::types::GLuint{
    let handle = gl_context.CreateShader(gl::FRAGMENT_SHADER);
    gl_context.ShaderSource(handle,1,[source.as_ptr().cast()].as_ptr(),std::ptr::null());
    gl_context.CompileShader(handle);
    return handle;
}

unsafe fn CompileVertexShader(gl_context : &gl::Gl, source: &str) -> gl::types::GLuint{
    let handle = gl_context.CreateShader(gl::VERTEX_SHADER);
    gl_context.ShaderSource(handle,1,[source.as_ptr().cast()].as_ptr(),std::ptr::null());
    gl_context.CompileShader(handle);
    return handle;
}

unsafe fn CheckError(gl_context : &gl::Gl) -> Option<String>{
    let err =  gl_context.GetError();
    match err {
        gl::NO_ERROR => {
            return None;
        } 
        _ => {
            return Some(format!(
                "Gl Error:{}",err
            ));    
        }
    }
}

fn PrintError(gl_context : &gl::Gl){
    unsafe{
        match CheckError(gl_context){
            Some(error) => {println!("{}",error)},
            None =>{}
        }
    }
}

pub trait Attribute{
    fn Count() -> gl::types::GLuint;
    fn Stride() -> gl::types::GLuint;
    fn Name() -> &'static str;
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Shader{
    handle : gl::types::GLuint,
}

impl Shader{
    pub fn new(gl_context : &gl::Gl,frag : &str,vert : &str) -> Result<Shader,&'static str>{
        unsafe{
            let handle = CreateProgram(gl_context,frag,vert);
            return Ok(Shader{
                handle : handle
            });
        }
    }
    
    pub fn default() -> Self{
        return Self{
            handle : 0
        }
    }

    pub fn BindAttrib<T : Attribute>(&self, gl_context : &gl::Gl) -> gl::types::GLint {
        unsafe{
            return gl_context.GetAttribLocation(self.handle, T::Name().as_ptr() as *const _);
        }
    }

    pub fn SetAttrib<T : Attribute>(&self, location : gl::types::GLint, gl_context : &gl::Gl) {
        unsafe{
        gl_context.VertexAttribPointer(
            location as gl::types::GLuint,
            T::Count() as i32,
            gl::FLOAT,
            0,
            (T::Stride() as usize * std::mem::size_of::<f32>()) as gl::types::GLsizei,
            std::ptr::null()
        );
        }
    }
}

