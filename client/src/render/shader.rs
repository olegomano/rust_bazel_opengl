use std::ffi::{CStr, CString};

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
    pub use Gles2 as Gl;
}


unsafe fn CreateProgram(gl_context : &gl::Gl, vertex_shader: &str, fragment_shader: &str) -> gl::types::GLuint{
   let program = gl_context.CreateProgram();
   let vert = CompileVertexShader(gl_context,vertex_shader);
   let frag = CompileFragmentShader(gl_context,fragment_shader);
   gl_context.AttachShader(program,vert);
   gl_context.AttachShader(program,frag);
   gl_context.LinkProgram(program);
   return program;
}

// let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
unsafe fn CompileFragmentShader(gl_context : &gl::Gl, source: &str) -> gl::types::GLuint{
    let handle = gl_context.CreateShader(gl::FRAGMENT_SHADER);
    gl_context.ShaderSource(handle,1,std::ptr::null(),std::ptr::null());
    gl_context.CompileShader(handle);
    return handle;
}

unsafe fn CompileVertexShader(gl_context : &gl::Gl, source: &str) -> gl::types::GLuint{
    let handle = gl_context.CreateShader(gl::VERTEX_SHADER);
    gl_context.ShaderSource(handle,1,std::ptr::null(),std::ptr::null());
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

pub struct Shader{
    handle : gl::types::GLuint,
}

impl Shader{
    pub fn new(gl_context : &gl::Gl,frag : &str,vert : &str) -> Result<Shader,&'static str>{
        unsafe{
            let handle = CreateProgram(gl_context,frag,vert);
            return Err("Failed");
        }
    }
} 
