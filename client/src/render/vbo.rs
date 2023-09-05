extern crate gl_context;
use gl_context::gl;

#[derive(Clone)]
pub struct Vbo{
    handle : std::cell::Cell<gl::types::GLuint>,
    elements : std::cell::Cell<gl::types::GLint>,
}

impl Vbo{
    pub fn new() -> Self{
        Self{
            handle : std::cell::Cell::new(0),
            elements : std::cell::Cell::new(0),
        }
    }
    
    pub fn Load(&self, gl : &gl::Gl, buffer : &[f32] ){
        unsafe{
            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (buffer.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                buffer.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            self.handle.replace(vbo); 
            self.elements.replace(buffer.len());
        }
    }
    
    pub fn Handle(&self) -> gl::types::GLuint {
        return self.handle.get();
    }

    pub fn Elements(&self) -> gl::types::GLint {
        return self.elements.get();
    }

    pub fn Bind(&self, gl : &gl::Gl){
        unsafe{
            gl.BindBuffer(gl::ARRAY_BUFFER, self.handle.get());
        }
    }    
}

