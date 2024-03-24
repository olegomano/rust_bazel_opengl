extern crate glam;
extern crate shader; 
extern crate vbo;
extern crate drawable;
use gl_context::gl;
extern crate gl_error;
use gl_error::CheckError;

pub static VERTEX_DATA_TRIS: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];


pub static VERTEX_DATA_RECT: [f32; 60] = [
 -1.0,  1.0, 0.0,1.0,  0.0,0.0,1.0,1.0,  0.0,1.0,
  1.0,  1.0, 0.0,1.0,  0.0,0.0,1.0,1.0,  1.0,1.0,
 -1.0, -1.0, 0.0,1.0,  0.0,0.0,1.0,1.0,  0.0,0.0,

 -1.0, -1.0, 0.0, 1.0,  0.0,0.0,1.0,1.0,  0.0,0.0,
  1.0,  1.0, 0.0, 1.0,  0.0,0.0,1.0,1.0,  1.0,1.0,
  1.0, -1.0, 0.0, 1.0,  0.0,0.0,1.0,1.0,  1.0,0.0,
];

const VERTEX_DATA_CUBE: [f32; 360] = [
    // Front face
    -0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0,
    0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,

    // Back face
    -0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0,
    0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,

    // Left face
    -0.5, -0.5, -0.5, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    -0.5, -0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    -0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, -0.5, -0.5, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    -0.5, 0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, 0.5, -0.5, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,

    // Right face
    0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0,
    0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,

    // Top face
    -0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0,
    0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,

    // Bottom face
    -0.5, -0.5, -0.5, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    0.5, -0.5, -0.5, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    0.5, -0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, -0.5, -0.5, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    0.5, -0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    -0.5, -0.5, 0.5, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
];




pub struct MeshDrawable {
    vbo : vbo::Vbo,
    pos_layout : shader::Layout,
    uv_layout : shader::Layout,
    transform : std::cell::Cell<glam::Mat4>,
    texture : texture::Texture,
}
/*
 * Represents a mesh that has the following vertex format [x1,y1,z1,u,v]
 *
 */
impl MeshDrawable {
    pub fn new(buffer : &[f32], gl_context : &gl::Gl) -> Self{
        let mut drawable = Self::default();
        drawable.vbo.Load(gl_context,&buffer);
        return drawable;    
    }

    pub fn new_tris(gl_context : &gl::Gl) -> Self{
        return Self::new(&VERTEX_DATA_TRIS,gl_context);
    }

    pub fn new_rect(gl_context : &gl::Gl) -> Self{
        return Self::new(&VERTEX_DATA_RECT,gl_context);
    }
    
    pub fn new_cube(gl_context : &gl::Gl) -> Self{
        return Self::new(&VERTEX_DATA_CUBE,gl_context);
    }
    
    pub fn Load(&self, buffer : &[f32], gl_context : &gl::Gl){
        self.vbo.Load(gl_context,buffer);
    }
    
    pub fn SetTexture(&self,texture : &texture::Texture) {
        self.texture.Update(texture.Handle());
    }

    pub fn UpdateTransform(&self,t : &glam::Mat4){
        self.transform.set(*t);
    }

    pub fn default() -> Self{
        return Self{
            vbo : vbo::Vbo::new(),
            pos_layout : shader::Layout{
                count : 4,
                stride : 10,
                offset : 0,
            },
            uv_layout : shader::Layout{
                count : 2,
                stride : 10,
                offset : 8,
            },
            texture : texture::Texture::default(),
            transform : std::cell::Cell::new(glam::Mat4::IDENTITY),
        }
    }
}

impl drawable::SpriteDrawable for MeshDrawable{
    fn PosAttribute(&self) -> &shader::Layout {
        return &self.pos_layout;
    }

    fn UvAttribute(&self) -> &shader::Layout {
        return &self.uv_layout;
    }

    fn Buffer(&self) -> &vbo::Vbo {
        return &self.vbo;
    }
    
    fn Transform(&self) -> glam::Mat4{
        return self.transform.get();
    }

    fn Texture(&self) -> &texture::Texture{
        return &self.texture;
    }
}
