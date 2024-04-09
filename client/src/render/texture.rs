extern crate gl_context;
extern crate gl_utils;
extern crate asset;
extern crate gl_error;
use gl_error::CheckError;
use gl_context::gl;
use gl_context::gl::types::GLuint;




#[derive(Clone)]
pub struct Texture{
    handle : std::cell::Cell<gl::types::GLuint>,
}

impl Texture{
    pub fn new(image_data : &[u8], gl_context : &gl::Gl) -> Self{
        let res = Self::default();
        res.Load(image_data,128,128,gl_context);
        return res;
    }

    pub fn from_asset(asset : &asset::Asset, gl_context : &gl::Gl) -> Self{
        let res = Self::default();
        res.Load(asset.data,asset.width,asset.height,gl_context);
        return res;
    }

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
    
    #[gl_utils::gl_error]
    pub fn Bind(&self, gl_context : &gl::Gl){
        gl_context.BindTexture(gl::TEXTURE_2D, self.handle.get());
    }

    #[gl_utils::timed]
    #[gl_utils::gl_error]
    pub fn Load(&self,image_data : &[u8], width : i32, height : i32, gl_context : &gl::Gl) {
        let mut texture_id: GLuint = 0;
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
