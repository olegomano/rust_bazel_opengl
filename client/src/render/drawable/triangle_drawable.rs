extern crate glam;
extern crate shader; 
extern crate vbo;
extern crate drawable;
use gl_context::gl;

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];

pub struct TriangleDrawable {
    vbo : vbo::Vbo,
    pos_layout : shader::Layout,
    uv_layout : shader::Layout,
    transform : std::cell::Cell<glam::Mat4>,
}

impl TriangleDrawable {
    pub fn new(gl_context : &gl::Gl) -> Self{
        let mut drawable = Self::default();
        drawable.vbo = vbo::Vbo::new();
        drawable.vbo.Load(gl_context,&VERTEX_DATA);
        return drawable;    
    }
    
    pub fn Load(&self,gl_context : &gl::Gl){
        self.vbo.Load(gl_context,&VERTEX_DATA);
    }

    pub fn default() -> Self{
        return Self{
            vbo : vbo::Vbo::new(),
            pos_layout : shader::Layout{
                count : 3,
                stride : 5,
            },
            uv_layout : shader::Layout{
                count : 2,
                stride : 5,
            },
            transform : std::cell::Cell::new(glam::Mat4::IDENTITY),
        }
    }
}

impl drawable::SpriteDrawable for TriangleDrawable{
    fn PosAttribute(&self) -> &shader::Layout {
        return &self.pos_layout;
    }

    fn UvAttribute(&self) -> &shader::Layout {
        return &self.uv_layout;
    }

    fn Buffer(&self) -> &vbo::Vbo {
        return &self.vbo;
    }
    
    fn UpdateTransform(&self,t : &glam::Mat4){
        self.transform.set(*t);
    }

    fn Transform(&self) -> glam::Mat4{
        return self.transform.get();
    }
}
