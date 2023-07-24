extern crate gl_context;
use gl_context::gl;

#[derive(Clone)]
pub struct Vbo{
    handle : std::cell::Cell<gl::types::GLuint>,
    elements : std::cell::Cell<gl::types::GLint>,
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];

impl Vbo{
    pub fn new() -> Self{
        Self{
            handle : std::cell::Cell::new(0),
            elements : std::cell::Cell::new(0),
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
    
    pub fn InitTriangle(&self, gl : &gl::Gl) {    
        unsafe{
            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            self.handle.replace(vbo); 
            self.elements.replace(3);
        }
    }
}

