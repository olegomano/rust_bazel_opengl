extern crate gl_context;
extern crate image;
use gl_context::gl;
use gl_context::gl::types::GLuint;
use image::io::Reader as ImageReader;

pub struct Texture{
    handle : std::cell::Cell<gl::types::GLuint>,
}


fn LoadFromPath(path : &str) -> (u32,u32,std::vec::Vec<u8>) {
    let image = ImageReader::open(path)
        .expect("Failed to open the image")
        .decode()
        .expect("Failed to decode the image");
    return (image.width(),image.height(),image.to_rgba8().into_raw());
}

impl Texture{
    pub fn new(path : &str, gl : &gl::Gl) -> Self{
        let result = Texture::default();
        result.Load(path,gl);
        return result;
    }

    pub fn default() -> Self{
        return Self{
            handle : std::cell::Cell::new(0),
        }
    }

    pub fn Load(&self,path : &str, gl : &gl::Gl) {
        let mut texture_id: GLuint = 0;
        let (width,height,image_data) = LoadFromPath(path);
        unsafe{
             gl.GenTextures(1, &mut texture_id);
             gl.BindTexture(gl::TEXTURE_2D, texture_id);
             gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            gl.TexImage2D(
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
