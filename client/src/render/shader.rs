use std::ffi::{CStr, CString};
use std::ptr;
extern crate gl_context;
extern crate glam;
use gl_context::gl;
extern crate gl_utils;

#[gl_utils::gl_error]
unsafe fn CreateProgram(gl_context : &gl::Gl, vertex_shader: &str, fragment_shader: &str) -> gl::types::GLuint{
   let vert = CompileVertexShader(gl_context,vertex_shader);
   check_gl_compile_errors(gl_context,vert);
   let frag = CompileFragmentShader(gl_context,fragment_shader);
   check_gl_compile_errors(gl_context,frag);
   let program = gl_context.CreateProgram();
   PrintError(gl_context);
   gl_context.AttachShader(program,vert);
   PrintError(gl_context);
   gl_context.AttachShader(program,frag);
   PrintError(gl_context);
   gl_context.LinkProgram(program);
   check_gl_link_errors(gl_context,program);
   return program;
}

#[gl_utils::gl_error]
unsafe fn CompileFragmentShader(gl_context : &gl::Gl, source: &str) -> gl::types::GLuint{
    let handle = gl_context.CreateShader(gl::FRAGMENT_SHADER);
    gl_context.ShaderSource(handle,1,[source.as_ptr().cast()].as_ptr(),std::ptr::null());
    gl_context.CompileShader(handle);
    return handle;
}


#[gl_utils::gl_error]
unsafe fn CompileVertexShader(gl_context : &gl::Gl, source: &str) -> gl::types::GLuint{
    let handle = gl_context.CreateShader(gl::VERTEX_SHADER);
    gl_context.ShaderSource(handle,1,[source.as_ptr().cast()].as_ptr(),std::ptr::null());
    gl_context.CompileShader(handle);
    return handle;
}

unsafe fn CheckCompileError(gl_context : &gl::Gl) ->Option<String>{
    return None;
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


fn check_gl_compile_errors(gl_context : &gl::Gl, shader: gl::types::GLuint) {
    let mut success: gl::types::GLint = 0;
    unsafe {
        gl_context.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as gl::types::GLint {
            let mut info_log_length: gl::types::GLint = 0;
            gl_context.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);
            
            let mut info_log: Vec<u8> = Vec::with_capacity(info_log_length as usize);
            info_log.extend([b' '].iter().cycle().take(info_log_length as usize));
            
            gl_context.GetShaderInfoLog(
                shader,
                info_log_length,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            
            let info_log_cstr = CString::from_vec_unchecked(info_log);
            println!(
                "Compilation error: {}",
                info_log_cstr.to_string_lossy()
            );
        }
    }
}

fn check_gl_link_errors(gl_context : &gl::Gl, shader: gl::types::GLuint) {
    let mut success: gl::types::GLint = 0;
    unsafe {
        gl_context.GetProgramiv(shader, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as gl::types::GLint {
            let mut info_log_length: gl::types::GLint = 1024;
            let mut info_log: Vec<u8> = Vec::with_capacity(info_log_length as usize);
            info_log.extend([b' '].iter().cycle().take(info_log_length as usize));
            
            gl_context.GetProgramInfoLog(
                shader,
                info_log_length,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            
            let info_log_cstr = CString::from_vec_unchecked(info_log);
            println!(
                "Link error: {}",
                info_log_cstr.to_string_lossy()
            );
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
    fn Name() -> &'static str;
}

/*
 * Count: The amount of elements in each attribtue ( 2,3,4 )
 * Stride: 0 for tightly packed
 */
pub struct Layout{
    pub count : i32,
    pub stride : usize,
    pub offset : usize,
}

impl Layout{
    pub fn Count(&self) -> i32 {
        return self.count;
    }
    pub fn Stride(&self) -> usize{
        return self.stride;
    }
    pub fn Offset(&self) -> usize{
        return self.offset;
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Shader{
    handle : gl::types::GLuint,
}

impl Shader{
    #[gl_utils::gl_error]
    pub fn new(gl_context : &gl::Gl,frag : &str,vert : &str) -> Result<Shader,&'static str>{
        unsafe{
            let handle = CreateProgram(gl_context,frag,vert);
            println!("Compiled Shader: {}",handle);
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
    
    #[gl_utils::gl_error]
    pub fn Use(&self,gl_context : &gl::Gl) {
        unsafe{
            gl_context.UseProgram(self.handle);
        }
    }
    
    
    #[gl_utils::gl_error]
    pub fn BindAttrib<T : Attribute>(&self, gl_context : &gl::Gl) -> gl::types::GLint {
        unsafe{
            return gl_context.GetAttribLocation(self.handle, T::Name().as_ptr() as *const _);
        }
    }


    #[gl_utils::gl_error]
    pub fn BindUniform<T : Attribute>(&self, gl_context : &gl::Gl) -> gl::types::GLint {
        unsafe{
            return gl_context.GetUniformLocation(self.handle, T::Name().as_ptr() as *const _);
        }
    }

    pub fn SetUniformMat(&self, location : gl::types::GLint, mat : &glam::Mat4, gl : &gl::Gl){
        let matrix_data: [f32; 16] = mat.to_cols_array();
        unsafe {
            gl.UniformMatrix4fv(location, 1, gl::FALSE, matrix_data.as_ptr());
        }
    }

    #[gl_utils::gl_error]
    pub fn SetAttrib(&self, location : gl::types::GLint, layout : &Layout, gl_context : &gl::Gl) {
        unsafe{
            gl_context.VertexAttribPointer(
                location as gl::types::GLuint,
                layout.Count(),
                gl::FLOAT,
                0,
                (layout.Stride() * std::mem::size_of::<f32>()) as gl::types::GLsizei,
                (layout.Offset() * std::mem::size_of::<f32>()) as *const gl::types::GLvoid

            )
        }
    }
}

