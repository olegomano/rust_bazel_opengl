extern crate gl_context;
extern crate image;
extern crate gl_utils;
use gl_context::gl;
use gl_context::gl::types::GLuint;
use image::io::Reader as ImageReader;


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


#[derive(Clone)]
pub struct Texture{
    handle : std::cell::Cell<gl::types::GLuint>,
}

impl Texture{
    pub fn default() -> Self{
        return Self{
            handle : std::cell::Cell::new(0),
        }
    }
    
    pub fn Handle(&self) -> gl::types::GLuint {
        return self.handle.get();
    }

    pub fn Update(&self,handle : gl::types::GLuint) {
        self.handle.replace(handle);
    }
    
    #[gl_utils::timed]
    #[gl_utils::gl_error]
    pub fn Load(&self,image_data : &[u8], gl_context : &gl::Gl) {
        let mut texture_id: GLuint = 0;
        let width = 128;
        let height = 128;
        unsafe{
             gl_context.GenTextures(1, &mut texture_id);
             gl_context.BindTexture(gl::TEXTURE_2D, texture_id);
             gl_context.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl_context.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            gl_context.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_data.as_ptr() as *const _,
            );
        }
        self.handle.replace(texture_id);
    }
}
